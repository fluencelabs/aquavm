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

#[derive(Debug, Default)]
pub(super) struct SubtreeSizeTracker {
    saved_states_count: usize,
    left_subtree_size: usize,
    right_subtree_size: usize,
}

impl SubtreeSizeTracker {
    pub(super) fn update(&mut self, data_keeper: &DataKeeper, subtree_type: SubtreeType) {
        let prev_states_count = self.saved_states_count;
        let states_count = data_keeper.result_trace.len();
        let resulted_states_count = states_count - prev_states_count;

        match subtree_type {
            SubtreeType::Left => self.left_subtree_size = resulted_states_count,
            SubtreeType::Right => self.right_subtree_size = resulted_states_count,
        }
        self.saved_states_count = data_keeper.result_trace.len();
    }

    pub(super) fn into_par(self) -> ExecutedState {
        let par = ParResult(self.left_subtree_size, self.right_subtree_size);
        ExecutedState::Par(par)
    }
}
