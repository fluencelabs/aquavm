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

use super::*;
use new_states_calculation::compute_new_states;

/// At the end of a Par execution it's needed to update subtrace_len and positions of both sliders.
///
/// To see why it's really needed, imagine the following trace:
/// [par 12, 3]
///     [par 3, 5]                                                       <- left subgraph of [par 9, 3]
///         [call rs 1] [call rs 2] [call rs 3]                          <- left subgraph of [par 3, 5]
///         [call rs 4] [call rs 5] [call rs 6] [call rs 7] [call rs 8]  <- right subgraph of [par 3, 5]
///     [par 1, 1]                                                       <- right subgraph of [par 9, 3]
///         [call e 9]                                                   <- left subgraph of [par 1, 1]
///         [call e 10]                                                  <- right subgraph of [par 1, 1]
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
/// error). Since it's wrapped with xor, then right subgraph of xor (null) will be executed.
/// After that next par will be executed. This par has corresponding state [par 1, 1] in a trace,
/// and to allow slider to pop it it's needed to set updated position in a proper way, because
/// otherwise [call rs 6] will be returned.
///
/// This struct manages to save the updated lens and pos and update slider states to prevent
/// such situations.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct CtxStateHandler {
    left_pair: CtxStatesPair,
    right_pair: CtxStatesPair,
}

impl CtxStateHandler {
    /// Prepare new states that sliders will have after finishing executing of each subgraph.
    pub(super) fn prepare(
        prev_par: ParResult,
        current_par: ParResult,
        data_keeper: &mut DataKeeper,
    ) -> FSMResult<Self> {
        let left_pair = compute_new_states(data_keeper, prev_par, current_par, SubgraphType::Left)?;
        let right_pair = compute_new_states(data_keeper, prev_par, current_par, SubgraphType::Right)?;

        let handler = Self { left_pair, right_pair };

        Ok(handler)
    }

    pub(super) fn handle_subgraph_end(self, data_keeper: &mut DataKeeper, subgraph_type: SubgraphType) {
        match subgraph_type {
            SubgraphType::Left => update_ctx_states(self.left_pair, data_keeper),
            SubgraphType::Right => update_ctx_states(self.right_pair, data_keeper),
        }
    }
}
