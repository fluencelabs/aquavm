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
mod utils;

use super::*;
use crate::execution_step::trace_handler::state_automata::par_fsm::state_handler::utils::compute_par_total_lens;
use new_states_calculation::compute_new_states;
use utils::prepare_total_lens;

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
    left_pair: CtxStatesPair,
    right_pair: CtxStatesPair,
}

impl CtxStateHandler {
    pub(super) fn prepare(
        prev_par: ParResult,
        current_par: ParResult,
        data_keeper: &mut DataKeeper,
    ) -> FSMResult<Self> {
        let left_pair = prepare_left_pair(prev_par, current_par, data_keeper)?;
        let right_pair = prepare_right_pair(prev_par, current_par, data_keeper)?;

        let handler = Self { left_pair, right_pair };

        Ok(handler)
    }

    pub(super) fn handle_subtree_end(self, data_keeper: &mut DataKeeper, subtree_type: SubtreeType) {
        match subtree_type {
            SubtreeType::Left => update_ctx_states(self.left_pair, data_keeper),
            SubtreeType::Right => update_ctx_states(self.right_pair, data_keeper),
        }
    }
}

fn prepare_left_pair(
    prev_par: ParResult,
    current_par: ParResult,
    data_keeper: &mut DataKeeper,
) -> FSMResult<CtxStatesPair> {
    let (prev_nibble, current_nibble) = compute_new_states(data_keeper, prev_par, current_par, SubtreeType::Left)?;
    let prev_state = CtxState::from_nibble(prev_nibble, prev_nibble.subtrace_len);
    let current_state = CtxState::from_nibble(current_nibble, current_nibble.subtrace_len);
    let pair = CtxStatesPair::new(prev_state, current_state);

    Ok(pair)
}

fn prepare_right_pair(
    prev_par: ParResult,
    current_par: ParResult,
    data_keeper: &mut DataKeeper,
) -> FSMResult<CtxStatesPair> {
    let (prev_par_len, current_par_len) = compute_par_total_lens(prev_par, current_par)?;
    let (prev_total_len, current_total_len) = prepare_total_lens(prev_par_len, current_par_len, data_keeper)?;

    let prev_pos = data_keeper.prev_slider().position() + prev_par_len;
    let current_pos = data_keeper.current_slider().position() + current_par_len;

    let prev_state = CtxState::new(prev_pos, prev_total_len, prev_total_len);
    let current_state = CtxState::new(current_pos, current_total_len, current_total_len);
    let pair = CtxStatesPair::new(prev_state, current_state);

    Ok(pair)
}
