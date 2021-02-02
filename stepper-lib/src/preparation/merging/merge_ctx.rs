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
use crate::JValue;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeCtx<'i> {
    trace: ExecutionTrace,
    subtree_size: usize,
    streams: HashMap<String, Vec<&'i JValue>>,
}

impl MergeCtx<'_> {
    pub(super) fn new(trace: ExecutionTrace) -> Self {
        let subtree_size = trace.len();

        Self {
            trace,
            subtree_size,
            streams: HashMap::new(),
        }
    }

    pub(super) fn next_subtree_state(&mut self) -> Option<ExecutedState> {
        if self.subtree_size != 0 {
            self.subtree_size -= 1;
            self.trace.pop_front()
        } else {
            None
        }
    }

    pub(super) fn set_subtree_size(&mut self, new_subtree_size: usize) {
        self.subtree_size = new_subtree_size;
    }

    pub(super) fn drain_subtree_states(&mut self) -> MergeResult<impl Iterator<Item = ExecutedState> + '_> {
        use crate::preparation::DataMergingError::ExecutedTraceTooSmall;

        if self.trace.len() < self.subtree_size {
            return Err(ExecutedTraceTooSmall(self.trace.len(), self.subtree_size));
        }

        Ok(self.trace.drain(..self.subtree_size))
    }

    pub(super) fn subtree_size(&self) -> usize {
        self.subtree_size
    }
}
