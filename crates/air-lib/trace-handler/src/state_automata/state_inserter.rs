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

use super::DataKeeper;
use super::ExecutedState;
use air_interpreter_data::TracePos;

/// This one is intended to optimize insertion in data to avoid insertion in a middle of it.
/// This is achieved by inserting a temporary state, track insert position and insert there
/// the final state.
#[derive(Debug, Default, Clone)]
pub(super) struct StateInserter {
    position: TracePos,
}

impl StateInserter {
    pub(super) fn from_keeper(data_keeper: &mut DataKeeper) -> Self {
        let position = data_keeper.result_trace_next_pos();
        // this par is a temporary state
        data_keeper.result_trace.push(ExecutedState::par(0, 0));

        Self { position }
    }

    pub(super) fn insert(self, data_keeper: &mut DataKeeper, state: ExecutedState) {
        data_keeper.result_trace[self.position] = state;
    }
}
