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

/// Tracks lens of data_keeper.result_trace to build resulted Par state at the end.
#[derive(Debug, Default, Clone)]
pub(super) struct ParBuilder {
    saved_states_count: usize,
    left_subgraph_size: usize,
    right_subgraph_size: usize,
}

impl ParBuilder {
    // StateInserter here needs to guaranteed that ParBuilder creates after it,
    // it must be so to right track a left subgraph size
    pub(super) fn from_keeper(data_keeper: &DataKeeper, _: &StateInserter) -> Self {
        let saved_states_count = data_keeper.result_states_count();

        Self {
            saved_states_count,
            left_subgraph_size: 0,
            right_subgraph_size: 0,
        }
    }

    pub(super) fn track(&mut self, data_keeper: &DataKeeper, subgraph_type: SubgraphType) {
        let prev_states_count = self.saved_states_count;
        let states_count = data_keeper.result_states_count();
        let resulted_states_count = states_count - prev_states_count;

        match subgraph_type {
            SubgraphType::Left => self.left_subgraph_size = resulted_states_count,
            SubgraphType::Right => self.right_subgraph_size = resulted_states_count,
        }
        self.saved_states_count = data_keeper.result_trace.len();
    }

    pub(super) fn build(self) -> ExecutedState {
        // TODO: check that usize could be converted into u32
        ExecutedState::par(self.left_subgraph_size, self.right_subgraph_size)
    }
}
