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

use std::convert::TryInto;

use super::ExecutedState;
use super::ExecutionTrace;
use super::KeeperError::*;
use super::KeeperResult;
use crate::state_automata::TraceLen;
use crate::TracePos;

type SeenElements = u32;

/// This slider is intended to slide on a subtrace inside provided trace. This subtrace
/// is identified by position and len.
// TODO: check for overflow
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TraceSlider {
    /// Trace that slider slide on.
    trace: ExecutionTrace,

    /// Position of current subtrace inside trace.
    position: TracePos,

    /// Length of a current subtrace.
    subtrace_len: TraceLen,

    /// Count of seen elements since the last position update.
    seen_elements: SeenElements,
}

impl TraceSlider {
    pub(crate) fn new(trace: impl Into<ExecutionTrace>) -> Self {
        let trace = trace.into();
        let subtrace_len = trace.len().try_into().unwrap();

        Self {
            trace,
            subtrace_len,
            ..<_>::default()
        }
    }

    /// Returns the next state if current interval length hasn't been reached
    /// and None otherwise.
    #[allow(clippy::suspicious_operation_groupings)]
    pub(crate) fn next_state(&mut self) -> Option<ExecutedState> {
        if self.seen_elements >= self.subtrace_len.into() || usize::from(self.position) >= self.trace.len() {
            return None;
        }

        let result = self.trace[self.position].clone();
        self.position += 1;
        self.seen_elements += 1;
        Some(result)
    }

    pub(crate) fn set_position_and_len(&mut self, position: TracePos, subtrace_len: TraceLen) -> KeeperResult<()> {
        // it's possible to set empty subtrace_len and inconsistent position
        if subtrace_len != 0 && position + subtrace_len > self.trace.len().try_into().unwrap() {
            return Err(SetSubtraceLenAndPosFailed {
                requested_pos: position,
                requested_subtrace_len: subtrace_len,
                trace_len: self.trace.len(),
            });
        }

        self.position = position;
        self.subtrace_len = subtrace_len;
        self.seen_elements = 0;

        Ok(())
    }

    pub(crate) fn set_subtrace_len(&mut self, subtrace_len: TraceLen) -> KeeperResult<()> {
        let trace_remainder: TraceLen = (TracePos::from(self.trace_len()) - self.position).into();
        if trace_remainder < subtrace_len {
            return Err(SetSubtraceLenFailed {
                requested_subtrace_len: subtrace_len,
                trace_position: self.position,
                trace_len: self.trace.len(),
            });
        }

        self.seen_elements = 0;
        self.subtrace_len = subtrace_len;

        Ok(())
    }

    pub(crate) fn position(&self) -> TracePos {
        self.position
    }

    pub(crate) fn subtrace_len(&self) -> TraceLen {
        self.subtrace_len - self.seen_elements
    }

    pub(crate) fn state_at_position(&self, position: TracePos) -> Option<&ExecutedState> {
        // it would be nice to have the `impl SliceIndex for TracePos`, but it is unstable
        self.trace.get(position)
    }

    pub(super) fn trace_len(&self) -> TraceLen {
        self.trace.len().try_into().unwrap()
    }
}
