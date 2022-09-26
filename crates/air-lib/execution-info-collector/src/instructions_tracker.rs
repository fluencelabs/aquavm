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

use air_parser::TextPos;

use std::collections::HashMap;

/// Intended to track a number of executed instruction of each type. For instructions that
/// have a corresponding state in data, it tracks number of executed instructions on
/// current peer (executed) and overall number (seen) of met instructions of such type.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct InstructionTracker {
    pub ap: ApTracker,
    pub call: CallTracker,
    pub fold: FoldTracker,
    pub match_count: u32,
    pub mismatch_count: u32,
    pub new_tracker: NewTracker,
    pub next_count: u32,
    pub null_count: u32,
    pub par: ParTracker,
    pub seq_count: u32,
    pub xor_count: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ApTracker {
    pub seen_count: u32,
    pub executed_count: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CallTracker {
    pub seen_count: u32,
    pub executed_count: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FoldTracker {
    pub seen_scalar_count: u32,
    pub seen_stream_count: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ParTracker {
    pub seen_count: u32,
    pub executed_count: u32,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct NewTracker {
    /// Mapping from a new instruction position in a script
    /// to a number of their execution. This is needed to
    /// support private stream generation mappings.
    pub executed_count: HashMap<TextPos, u32>,
}

impl InstructionTracker {
    pub fn meet_ap(&mut self) {
        self.ap.seen_count += 1;
    }

    pub fn meet_executed_ap(&mut self) {
        self.ap.executed_count += 1;
    }

    pub fn meet_call(&mut self) {
        self.call.seen_count += 1;
    }

    pub fn meet_executed_call(&mut self) {
        self.call.executed_count += 1;
    }

    pub fn meet_fold_scalar(&mut self) {
        self.fold.seen_scalar_count += 1;
    }

    pub fn meet_fold_stream(&mut self) {
        self.fold.seen_stream_count += 1;
    }

    pub fn meet_match(&mut self) {
        self.match_count += 1;
    }

    pub fn meet_mismatch(&mut self) {
        self.mismatch_count += 1;
    }

    pub fn meet_next(&mut self) {
        self.next_count += 1;
    }

    pub fn meet_null(&mut self) {
        self.null_count += 1;
    }

    pub fn meet_par(&mut self) {
        self.par.seen_count += 1;
    }

    pub fn meet_executed_par(&mut self) {
        self.par.executed_count += 1;
    }

    pub fn meet_seq(&mut self) {
        self.seq_count += 1;
    }

    pub fn meet_xor(&mut self) {
        self.xor_count += 1;
    }

    pub fn meet_new(&mut self, position: TextPos) {
        use std::collections::hash_map::Entry::{Occupied, Vacant};

        match self.new_tracker.executed_count.entry(position) {
            Occupied(mut entry) => *entry.get_mut() += 1,
            Vacant(entry) => {
                entry.insert(1);
            }
        };
    }
}

impl NewTracker {
    pub fn get_iteration(&self, position: TextPos) -> u32 {
        self.executed_count
            .get(&position)
            .copied()
            .unwrap_or_default()
    }
}
