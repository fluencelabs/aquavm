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

use super::DataKeeper;
use super::ExecutedState;

/// This one is intended to optimize insertion in data to avoid insertion in a middle of it.
/// This is achieved by inserting a temporary state, track insert position and insert there
/// the final state.
#[derive(Debug, Default, Clone)]
pub(super) struct StateInserter {
    position: usize,
}

impl StateInserter {
    pub(super) fn from_keeper<VT>(data_keeper: &mut DataKeeper<VT>) -> Self {
        let position = data_keeper.result_trace.len();
        // this par is a temporary state
        data_keeper.result_trace.push(ExecutedState::par(0, 0));

        Self { position }
    }

    pub(super) fn insert<VT>(self, data_keeper: &mut DataKeeper<VT>, state: ExecutedState<VT>) {
        data_keeper.result_trace[self.position] = state;
    }
}
