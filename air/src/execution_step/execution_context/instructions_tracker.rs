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

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct InstrTracker {
    pub(crate) ap: ApTracker,
    pub(crate) call: CallTracker,
    pub(crate) fold: FoldTracker,
    pub(crate) match_count: usize,
    pub(crate) mismatch_count: usize,
    pub(crate) next_count: usize,
    pub(crate) null_count: usize,
    pub(crate) par: ParTracker,
    pub(crate) seq_count: usize,
    pub(crate) xor_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct ApTracker {
    pub(crate) seen_count: usize,
    pub(crate) executed_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct CallTracker {
    pub(crate) seen_count: usize,
    pub(crate) executed_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct FoldTracker {
    pub(crate) seen_scalar_count: usize,
    pub(crate) seen_stream_count: usize,
}

#[derive(Default)]
#[allow(dead_code)]
pub(crate) struct ParTracker {
    pub(crate) seen_count: usize,
    pub(crate) executed_count: usize,
}

impl InstrTracker {
    pub(crate) fn met_call(&mut self) {
        self.call.seen_count += 1;
    }

    pub(crate) fn met_executed_call(&mut self) {
        self.call.executed_count += 1;
    }

    pub(crate) fn met_fold_stream(&mut self) {
        self.fold.seen_stream_count += 1;
    }
}
