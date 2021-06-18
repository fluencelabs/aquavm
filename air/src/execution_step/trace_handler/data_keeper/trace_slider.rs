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
use super::KeeperError;
use super::KeeperError::ExecutedTraceTooSmall;
use super::KeeperResult;

use std::cell::Cell;
use std::rc::Rc;

/// This slider is intended to extract states from execution_step trace in such a way
/// that it keeps count of deleted states. It allows to set position from which
/// states will be deleted according the initial version of trace. This one is
/// especially needed to handle fold states that consist of several non-intersecting
/// intervals.
///
/// Basically, the API of this slider provides us facilities to set position from
/// which states will be extracted and interval length from reaching that states
/// woudn't be extracted (None will be returned).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct TraceSlider {
    trace: ExecutionTrace,
    position: Cell<usize>,
    seen_elements: Cell<usize>,
    interval_len: Cell<usize>,
}

impl TraceSlider {
    pub(super) fn new(trace: ExecutionTrace) -> Self {
        let interval_len = Cell::new(trace.len());

        Self {
            trace,
            position: Cell::new(0),
            seen_elements: Cell::new(0),
            interval_len,
        }
    }

    /// Returns the next state if interval length haven't been reached
    /// and None otherwise.
    pub(crate) fn next_state(&self) -> Option<ExecutedState> {
        if self.seen_elements.get() >= self.interval_len.get() || self.position.get() >= self.trace.len() {
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

    pub(crate) fn set_interval_len(&self, interval_len: usize) -> KeeperResult<()> {
        if self.trace.len() - self.position.get() < interval_len {
            return Err(ExecutedTraceTooSmall(
                self.trace.len() - self.position.get(),
                interval_len,
            ));
        }

        self.seen_elements.set(0);
        self.interval_len.set(interval_len);

        Ok(())
    }

    pub(crate) fn state_by_pos(&self, pos: usize) -> KeeperResult<&ExecutedState> {
        if pos >= self.trace.len() {
            return Err(ExecutedTraceTooSmall(self.trace.len(), pos));
        }

        let state = &self.trace[pos];
        Ok(state)
    }

    pub(crate) fn position(&self) -> usize {
        self.position.get()
    }

    pub(crate) fn interval_len(&self) -> usize {
        self.interval_len.get() - self.seen_elements.get()
    }
}
