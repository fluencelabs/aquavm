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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(super) struct FoldLoreCtor {
    value_pos: usize,
    before_tracker: PositionsTracker,
    after_tracker: PositionsTracker,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct PositionsTracker {
    pub(self) start_pos: usize,
    pub(self) end_pos: usize,
}

impl FoldLoreCtor {
    pub(super) fn from_before_start(value_pos: usize, data_keeper: &DataKeeper) -> Self {
        let before_tracker = PositionsTracker {
            start_pos: data_keeper.result_states_count(),
            end_pos: 0,
        };

        Self {
            value_pos,
            before_tracker,
            ..<_>::default()
        }
    }

    pub(super) fn before_end(&mut self, data_keeper: &DataKeeper) {
        self.before_tracker.start_pos = data_keeper.result_states_count();
    }

    pub(super) fn after_start(&mut self, data_keeper: &DataKeeper) {
        self.after_tracker.start_pos = data_keeper.result_states_count();
    }

    pub(super) fn after_end(&mut self, data_keeper: &DataKeeper) {
        self.after_tracker.end_pos = data_keeper.result_states_count();
    }

    pub(super) fn into_subtrace(self) -> Vec<FoldSubTraceLore> {
        let before_lore = FoldSubTraceLore {
            value_pos: self.value_pos,
            begin_pos: self.before_tracker.start_pos,
            interval_len: self.before_tracker.len(),
        };

        let after_lore = FoldSubTraceLore {
            value_pos: self.value_pos,
            begin_pos: self.after_tracker.start_pos,
            interval_len: self.after_tracker.len(),
        };

        vec![before_lore, after_lore]
    }
}

impl PositionsTracker {
    pub(self) fn len(&self) -> usize {
        self.end_pos - self.start_pos
    }
}
