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

use super::ExecutedState;
use super::ExecutionTrace;
use super::KeeperError::ExecutedTraceTooSmall;
use super::KeeperResult;

use std::cell::Cell;

/// This slider is intended to slide on a subtrace inside provided trace. This subtrace
/// is identified by position and len.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct TraceSlider {
    /// Trace that slider slide on.
    trace: ExecutionTrace,

    /// Position of current subtrace inside trace.
    position: Cell<usize>,

    /// Length of a current subtrace.
    subtrace_len: Cell<usize>,

    /// Count of seen elements since the last position update.
    seen_elements: Cell<usize>,
}

impl TraceSlider {
    pub(super) fn new(trace: ExecutionTrace) -> Self {
        let subtrace_len = Cell::new(trace.len());

        Self {
            trace,
            position: Cell::new(0),
            subtrace_len,
            seen_elements: Cell::new(0),
        }
    }

    /// Returns the next state if current interval length hasn't been reached
    /// and None otherwise.
    pub(crate) fn next_state(&self) -> Option<ExecutedState> {
        if self.seen_elements.get() >= self.subtrace_len.get() || self.position.get() >= self.trace.len() {
            return None;
        }

        let result = self.trace[self.position.get()].clone();
        self.position.set(self.position.get() + 1);
        self.seen_elements.set(self.seen_elements.get() + 1);
        Some(result)
    }

    pub(crate) fn set_position(&self, position: usize) -> KeeperResult<()> {
        if self.trace.len() >= position {
            return Err(ExecutedTraceTooSmall(self.trace.len(), position));
        }

        self.position.set(position);
        Ok(())
    }

    pub(crate) fn set_subtrace_len(&self, subtrace_len: usize) -> KeeperResult<()> {
        if self.trace.len() - self.position.get() < subtrace_len {
            return Err(ExecutedTraceTooSmall(
                self.trace.len() - self.position.get(),
                subtrace_len,
            ));
        }

        self.seen_elements.set(0);
        self.subtrace_len.set(subtrace_len);

        Ok(())
    }

    pub(crate) fn state_by_pos(&self, pos: u32) -> KeeperResult<&ExecutedState> {
        let pos = pos as usize;
        if pos >= self.trace.len() {
            return Err(ExecutedTraceTooSmall(self.trace.len(), pos));
        }

        let state = &self.trace[pos];
        Ok(state)
    }

    pub(crate) fn position(&self) -> usize {
        self.position.get()
    }

    pub(crate) fn subtrace_len(&self) -> usize {
        self.subtrace_len.get() - self.seen_elements.get()
    }
}
