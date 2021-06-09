/*
 * Copyright 2020 Fluence Labs Limited
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
use super::MergeResult;
use crate::preparation_step::DataMergingError;
use crate::preparation_step::DataMergingError::ExecutedTraceTooSmall;
use crate::preparation_step::ExecutionTrace;
use crate::JValue;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TraceSlider {
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
    pub(super) fn next_state(&self) -> Option<ExecutedState> {
        if self.seen_elements.get() >= self.interval_len.get() {
            return None;
        }

        let result = self.trace[self.position.get()].clone();
        self.position.set(self.position.get() + 1);
        self.seen_elements.set(self.seen_elements.get() + 1);
        Some(result)
    }

    pub(super) fn set_position(&self, position: usize) {
        self.position.set(position);
    }

    pub(super) fn set_interval_len(&self, interval_len: usize) {
        self.seen_elements.set(0);
        self.interval_len.set(interval_len);
    }

    pub(super) fn call_result_by_pos(&self, pos: usize) -> MergeResult<&Rc<JValue>> {
        use crate::contexts::execution_trace::CallResult;

        if pos >= self.trace.len() {
            return Err(ExecutedTraceTooSmall(self.trace.len(), pos));
        }

        match &self.trace[pos] {
            ExecutedState::Call(CallResult::Executed(value)) => Ok(value),
            _ => Err(DataMergingError::IncompatibleState),
        }
    }

    /// Returns remained states in range [position..position+interval_len].
    pub(super) fn remaining_interval(&self) -> MergeResult<impl ExactSizeIterator<Item = ExecutedState> + '_> {
        let remaining_len = self.interval_len.get() - self.seen_elements.get();
        let interval = self.trace.iter().cloned().skip(self.position.get()).take(remaining_len);
        self.seen_elements.set(self.interval_len.get());
        self.position.set(self.position.get() + remaining_len);

        Ok(interval)
    }

    pub(super) fn interval_len(&self) -> usize {
        self.interval_len.get() - self.seen_elements.get()
    }
}
