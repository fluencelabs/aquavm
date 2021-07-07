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
#[derive(Debug, Default, Clone)]
pub(super) struct SubTraceStateUpdater {
    prev_pos: usize,
    prev_len: usize,
    current_pos: usize,
    current_len: usize,
}

impl SubTraceStateUpdater {
    pub(super) fn from_keeper(data_keeper: &DataKeeper, ingredients: MergerParResult) -> FSMResult<Self> {
        let (prev_pos, prev_len) = compute_new_pos_and_len(data_keeper, &ingredients.prev_par, MergeCtxType::Previous)?;
        let (current_pos, current_len) =
            compute_new_pos_and_len(data_keeper, &ingredients.current_par, MergeCtxType::Current)?;

        let updater = Self {
            prev_len,
            prev_pos,
            current_len,
            current_pos,
        };

        Ok(updater)
    }

    pub(super) fn update(self, data_keeper: &mut DataKeeper) {
        // these calls shouldn't produce a error, because sizes become less and
        // they have been already checked in the ctor. It's important to make it
        // in a such way, because this functions is called from error_exit that
        // shouldn't fail.
        let _ = data_keeper
            .prev_slider_mut()
            .set_position_and_len(self.prev_pos, self.prev_len);
        let _ = data_keeper
            .current_slider_mut()
            .set_position_and_len(self.current_pos, self.current_len);
    }
}

fn compute_new_pos_and_len(
    data_keeper: &DataKeeper,
    par_result: &Option<ParResult>,
    ctx_type: MergeCtxType,
) -> FSMResult<(usize, usize)> {
    let slider = match ctx_type {
        MergeCtxType::Previous => data_keeper.prev_slider(),
        MergeCtxType::Current => data_keeper.current_slider(),
    };

    let par_size = par_result
        .map(|p| p.size().ok_or(StateFSMError::ParLenOverflow(p)))
        .transpose()?
        .unwrap_or_default();

    let position = slider
        .position()
        .checked_add(par_size)
        // unwrap is safe here, because underflow could be caused only if par is Some
        .ok_or_else(|| StateFSMError::ParPosOverflow(par_result.unwrap(), slider.subtrace_len(), ctx_type))?;

    let len = slider
        .subtrace_len()
        .checked_sub(par_size)
        // unwrap is safe here, because underflow could be caused only if par is Some
        .ok_or_else(|| StateFSMError::ParLenUnderflow(par_result.unwrap(), slider.subtrace_len(), ctx_type))?;

    Ok((position, len))
}
