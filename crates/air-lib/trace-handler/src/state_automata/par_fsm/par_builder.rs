/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
