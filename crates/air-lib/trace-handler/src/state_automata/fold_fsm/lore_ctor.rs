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
use air_interpreter_data::SubTraceDesc;

/// This struct is a form of FSM and is intended to construct a fold subtrace lore element.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(super) struct SubTraceLoreCtor {
    value_pos: TracePos,
    before_tracker: PositionsTracker,
    after_tracker: PositionsTracker,
    state: CtorState,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct PositionsTracker {
    pub(self) start_pos: TracePos,
    pub(self) end_pos: TracePos,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CtorState {
    BeforeStarted,
    BeforeCompleted,
    AfterStarted,
    AfterCompleted,
}

impl SubTraceLoreCtor {
    pub(super) fn from_before_start(value_pos: TracePos, data_keeper: &DataKeeper) -> Self {
        let before_tracker = PositionsTracker {
            start_pos: data_keeper.result_trace_next_pos(),
            end_pos: 0.into(),
        };

        Self {
            value_pos,
            before_tracker,
            ..<_>::default()
        }
    }

    pub(super) fn before_end(&mut self, data_keeper: &DataKeeper) {
        self.before_tracker.end_pos = data_keeper.result_trace_next_pos();
        self.state.next();
    }

    pub(super) fn maybe_before_end(&mut self, data_keeper: &DataKeeper) {
        if self.state != CtorState::BeforeStarted {
            return;
        }

        self.before_tracker.end_pos = data_keeper.result_trace_next_pos();
        self.state.next();
    }

    pub(super) fn after_start(&mut self, data_keeper: &DataKeeper) {
        self.after_tracker.start_pos = data_keeper.result_trace_next_pos();
        self.state.next();
    }

    pub(super) fn after_end(&mut self, data_keeper: &DataKeeper) {
        self.after_tracker.end_pos = data_keeper.result_trace_next_pos();
        self.state.next();
    }

    pub(super) fn into_subtrace_lore(self) -> FoldSubTraceLore {
        let before = SubTraceDesc {
            begin_pos: self.before_tracker.start_pos,
            subtrace_len: self.before_tracker.len() as _,
        };

        let after = SubTraceDesc {
            begin_pos: self.after_tracker.start_pos,
            subtrace_len: self.after_tracker.len() as _,
        };

        FoldSubTraceLore {
            value_pos: self.value_pos,
            subtraces_desc: vec![before, after],
        }
    }

    // this function should be called in a situation of early exit from fold,
    // for more details see the comment above SubTraceLoreCtorQueue::finish().
    pub(super) fn finish(&mut self, data_keeper: &DataKeeper) {
        use CtorState::*;

        match self.state {
            BeforeStarted => {
                self.before_end(data_keeper);
                self.after_start(data_keeper);
                self.after_end(data_keeper);
            }
            BeforeCompleted => {
                self.after_start(data_keeper);
                self.after_end(data_keeper);
            }
            AfterStarted => {
                self.after_end(data_keeper);
            }
            AfterCompleted => {}
        }
    }
}

impl PositionsTracker {
    pub(self) fn len(&self) -> usize {
        (self.end_pos - self.start_pos).into()
    }
}

impl Default for CtorState {
    fn default() -> Self {
        Self::BeforeStarted
    }
}

impl CtorState {
    pub(self) fn next(&mut self) {
        use CtorState::*;

        let next_state = match self {
            BeforeStarted => BeforeCompleted,
            BeforeCompleted => AfterStarted,
            AfterStarted => AfterCompleted,
            AfterCompleted => AfterCompleted,
        };

        *self = next_state;
    }
}
