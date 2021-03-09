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

use super::MergeResult;
use crate::preparation::ExecutedState;
use crate::preparation::ExecutionTrace;

/// This slider is intended to extract states from execution trace in such a way
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
    deleted_elements_count: usize,
    position: usize,
    interval_len: usize,
}

impl TraceSlider {
    pub(super) fn new(trace: ExecutionTrace) -> Self {
        let subtree_size = trace.len();

        Self {
            trace,
            deleted_elements_count: 0,
            position: 0,
            interval_len: subtree_size,
        }
    }

    /// Returns the next state if interval length haven't been reached
    /// and None otherwise.
    pub(super) fn next_state(&mut self) -> Option<ExecutedState> {
        // TODO: consider returning an error if the second condition is false
        if self.interval_len != 0 && self.position < self.trace.len() {
            self.deleted_elements_count += 1;
            self.interval_len -= 1;
            // position isn't updated here because of
            self.trace.remove(self.position)
        } else {
            None
        }
    }

    pub(super) fn set_interval_len(&mut self, new_interval_len: usize) {
        self.interval_len = new_interval_len;
    }

    /// Set position in such a way that it reflects the initial state of trace
    /// (see comment on top of the TraceSlider struct).
    pub(super) fn adjust_position(&mut self, new_position: usize) {
        // TODO: check
        self.position = self.deleted_elements_count - new_position;
    }

    /// Returns remained states in range [position..position+interval_len].
    pub(super) fn drain_interval(&mut self) -> MergeResult<impl Iterator<Item = ExecutedState> + '_> {
        use crate::preparation::DataMergingError::ExecutedTraceTooSmall;

        if self.trace.len() < self.interval_len {
            return Err(ExecutedTraceTooSmall(self.trace.len(), self.interval_len));
        }
        self.deleted_elements_count += self.interval_len;
        Ok(self.trace.drain(self.position..self.position + self.interval_len))
    }

    pub(super) fn subtree_size(&self) -> usize {
        self.interval_len
    }
}
