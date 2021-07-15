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

mod new_states_calculation;
mod total_len_preparation;

use super::*;
use new_states_calculation::compute_new_states;
use total_len_preparation::prepare_total_lens;

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

