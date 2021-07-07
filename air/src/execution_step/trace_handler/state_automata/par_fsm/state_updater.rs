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
    pub(self) prev_size: usize,
    pub(self) prev_pos: usize,
    pub(self) current_size: usize,
    pub(self) current_pos: usize,
}

impl SubTraceStateUpdater {
    pub(super) fn from_keeper(data_keeper: &DataKeeper, ingredients: MergerParResult) -> FSMResult<Self> {
        let prev_subtree_size = data_keeper.prev_slider().subtrace_len();
        // overflow here was checked in slider
        let prev_pos = data_keeper.prev_slider().position() + prev_subtree_size;
        let prev_size = Self::compute_new_size(prev_subtree_size, ingredients.prev_par)?;

        let current_subtree_size = data_keeper.current_slider().subtrace_len();
        let current_pos = data_keeper.current_slider().position() + current_subtree_size;
        let current_size = Self::compute_new_size(current_subtree_size, ingredients.current_par)?;

        let updater = Self {
            prev_size,
            prev_pos,
            current_size,
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
            .set_position_and_len(self.prev_pos, self.prev_size);
        let _ = data_keeper
            .current_slider_mut()
            .set_position_and_len(self.current_pos, self.current_size);
    }

    fn compute_new_size(initial_size: usize, par_result: Option<ParResult>) -> FSMResult<usize> {
        let par_size = par_result
            .map(|p| p.size().ok_or(StateFSMError::ParLenOverflow(p)))
            .transpose()?
            .unwrap_or_default();

        let new_size = initial_size
            .checked_sub(par_size)
            // unwrap is safe here, because underflow could be caused only if par is Some
            .ok_or_else(|| StateFSMError::ParSubtreeUnderflow(par_result.unwrap(), initial_size))?;

        Ok(new_size)
    }
}
