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

use air_interpreter_data::TraceLen;

use super::ExecutedState;
use super::ExecutionTrace;
use super::KeeperError::*;
use super::KeeperResult;
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
        let subtrace_len = trace.trace_states_count();

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
        if self.seen_elements >= self.subtrace_len || self.position >= self.trace.trace_states_count().into() {
            return None;
        }

        let result = self.trace[self.position].clone();
        self.position += 1;
        self.seen_elements += 1;
        Some(result)
    }

    pub(crate) fn set_position_and_len(&mut self, position: TracePos, subtrace_len: TraceLen) -> KeeperResult<()> {
        // it's possible to set empty subtrace_len and inconsistent position
        if subtrace_len != 0 && position + subtrace_len > self.trace.trace_states_count().into() {
            return Err(SetSubtraceLenAndPosFailed {
                requested_pos: position,
                requested_subtrace_len: subtrace_len,
                trace_len: self.trace.trace_states_count(),
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
                trace_len: self.trace.trace_states_count(),
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
        debug_assert!(self.subtrace_len >= self.seen_elements);
        self.subtrace_len - self.seen_elements
    }

    pub(crate) fn state_at_position(&self, position: TracePos) -> Option<&ExecutedState> {
        // it would be nice to have the `impl SliceIndex for TracePos`, but it is unstable
        self.trace.get(position)
    }

    pub(super) fn trace_len(&self) -> TraceLen {
        self.trace.trace_states_count()
    }
}
