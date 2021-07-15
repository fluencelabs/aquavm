/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;

/// At the end of a Par execution it's needed to update subtrace_len and positions of both sliders.
///
/// To see why it's really needed, imagine the following trace:
/// [par 9, 3]
///     [par 3, 5]                                                       <- left subtree of [par 9, 3]
///         [call rs 1] [call rs 2] [call rs 3]                          <- left subtree of [par 3, 5]
///         [call rs 4] [call rs 5] [call rs 6] [call rs 7] [call rs 8]  <- right subtree of [par 3, 5]
///     [par 1, 1]                                                       <- right subtree of [par 9, 3]
///         [call e 9]                                                   <- left subtree of [par 1, 1]
///         [call e 10]                                                  <- right subtree of [par 1, 1]
///
/// where
///     call rs N - request sent state of Nth call
///     call e N - executed state of Nth call
///
/// and the following script:
/// (par
///     (xor
///         (par
///             (call 1-3)
///             (call 4-8)
///         )
///         (null)  <- here could be any non-fallible set of instructions
///     )
///     (par
///         (call 9)
///         (call 10)
///     )
/// )
///
/// Suppose that call 5 (corresponds to [call rs 5]) will fail (f.e. call_service returns a service
/// error). Since it's wrapped with xor, then right subtree of xor (null) will be executed.
/// After that next par will be executed. This par has corresponding state [par 1, 1] in a trace,
/// and to allow slider to pop it it's needed to set updated position in a proper way, because
/// otherwise [call rs 6] will be returned.
///
/// This struct manages to save the updated lens and pos and update slider states to prevent
/// such situations.
///
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct CtxStateHandler {
    prev_total_len: usize,
    current_total_len: usize,
    prev_state: CtxState,
    current_state: CtxState,
}

impl CtxStateHandler {
    pub(super) fn prepare_left_start(
        data_keeper: &mut DataKeeper,
        prev_par: ParResult,
        current_par: ParResult,
    ) -> FSMResult<Self> {
        let (prev_total_len, current_total_len) = prepare_total_lens(prev_par, current_par, data_keeper)?;
        let (prev_state, current_state) = compute_new_states(data_keeper, prev_par, current_par, SubtreeType::Left)?;
        prepare_sliders(prev_par, current_par, data_keeper, SubtreeType::Left)?;

        let handler = Self {
            prev_total_len,
            current_total_len,
            prev_state,
            current_state,
        };

        Ok(handler)
    }

    pub(super) fn prepare_right_start(
        &mut self,
        data_keeper: &mut DataKeeper,
        prev_par: ParResult,
        current_par: ParResult,
    ) -> FSMResult<()> {
        let (mut prev_state, mut current_state) =
            compute_new_states(data_keeper, prev_par, current_par, SubtreeType::Right)?;
        prev_state.total_subtrace_len = self.prev_total_len;
        current_state.total_subtrace_len = self.current_total_len;

        self.prev_state = prev_state;
        self.current_state = current_state;

        prepare_sliders(prev_par, current_par, data_keeper, SubtreeType::Right)?;

        Ok(())
    }

    pub(super) fn handle_subtree_end(self, data_keeper: &mut DataKeeper) {
        update_with_states(self.prev_state, self.current_state, data_keeper)
    }
}

fn compute_new_states(
    data_keeper: &DataKeeper,
    prev_par: ParResult,
    current_par: ParResult,
    subtree_type: SubtreeType,
) -> FSMResult<(CtxState, CtxState)> {
    let (prev_len, current_len) = match subtree_type {
        SubtreeType::Left => (prev_par.0, current_par.0),
        SubtreeType::Right => (prev_par.1, current_par.1),
    };

    let prev_state = compute_new_state(data_keeper, prev_len as usize, MergeCtxType::Previous, prev_par)?;
    let current_state = compute_new_state(data_keeper, current_len as usize, MergeCtxType::Current, current_par)?;

    Ok((prev_state, current_state))
}

fn compute_new_state(
    data_keeper: &DataKeeper,
    par_subtree_len: usize,
    ctx_type: MergeCtxType,
    par: ParResult,
) -> FSMResult<CtxState> {
    let (slider, total_subtrace_len) = match ctx_type {
        MergeCtxType::Previous => (data_keeper.prev_slider(), data_keeper.prev_ctx.total_subtrace_len()),
        MergeCtxType::Current => (
            data_keeper.current_slider(),
            data_keeper.current_ctx.total_subtrace_len(),
        ),
    };

    let pos = slider
        .position()
        .checked_add(par_subtree_len)
        .ok_or_else(|| StateFSMError::ParPosOverflow(par, slider.position(), MergeCtxType::Previous))?;

    let subtrace_len = total_subtrace_len
        .checked_sub(par_subtree_len)
        .ok_or_else(|| StateFSMError::ParLenUnderflow(par, slider.subtrace_len(), MergeCtxType::Current))?;

    let state = CtxState::new(pos, subtrace_len, subtrace_len);

    Ok(state)
}

fn prepare_sliders(
    prev_par: ParResult,
    current_par: ParResult,
    data_keeper: &mut DataKeeper,
    subtree_type: SubtreeType,
) -> FSMResult<()> {
    let (prev_len, current_len) = match subtree_type {
        SubtreeType::Left => (prev_par.0, current_par.0),
        SubtreeType::Right => (prev_par.1, current_par.1),
    };

    data_keeper.prev_slider_mut().set_subtrace_len(prev_len as _)?;
    data_keeper.current_slider_mut().set_subtrace_len(current_len as _)?;

    Ok(())
}

fn prepare_total_lens(
    prev_par: ParResult,
    current_par: ParResult,
    data_keeper: &mut DataKeeper,
) -> FSMResult<(usize, usize)> {
    let (prev_size, current_size) = compute_par_total_lens(prev_par, current_par)?;
    sizes_suits(prev_size, current_size, data_keeper)?;

    let prev_total_len = data_keeper.prev_ctx.total_subtrace_len() - prev_size;
    let current_total_len = data_keeper.current_ctx.total_subtrace_len() - current_size;

    data_keeper.prev_ctx.set_total_subtrace_len(prev_size);
    data_keeper.current_ctx.set_total_subtrace_len(current_size);

    Ok((prev_total_len, current_total_len))
}

fn compute_par_total_lens(prev_par: ParResult, current_par: ParResult) -> FSMResult<(usize, usize)> {
    let prev_par_len = prev_par.size().ok_or(StateFSMError::ParLenOverflow(prev_par))?;
    let current_par_len = current_par.size().ok_or(StateFSMError::ParLenOverflow(prev_par))?;

    Ok((prev_par_len, current_par_len))
}

fn sizes_suits(prev_par_len: usize, current_par_len: usize, data_keeper: &DataKeeper) -> FSMResult<()> {
    let prev_total_len = data_keeper.prev_ctx.total_subtrace_len();
    if prev_par_len > prev_total_len {
        return Err(StateFSMError::TotalSubtraceLenIsLess(
            prev_par_len,
            prev_total_len,
            MergeCtxType::Previous,
        ));
    }

    let current_total_len = data_keeper.current_ctx.total_subtrace_len();
    if current_par_len > current_total_len {
        return Err(StateFSMError::TotalSubtraceLenIsLess(
            prev_par_len,
            current_total_len,
            MergeCtxType::Current,
        ));
    }

    Ok(())
}
