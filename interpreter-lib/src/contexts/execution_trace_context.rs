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

mod executed_state;

pub use executed_state::CallResult;
pub use executed_state::ExecutedState;

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

pub type ExecutionTrace = std::collections::VecDeque<ExecutedState>;

/// Encapsulates all necessary state regarding to the call pathes1.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct ExecutionTraceCtx {
    /// Contains trace (serialized tree of states) after merging current and previous data,
    /// interpreter used it to realize which instructions've been already executed.
    pub(crate) current_trace: ExecutionTrace,

    /// Size of a current considered subtree inside current path.
    pub(crate) current_subtree_size: usize,

    // TODO: consider change it to Vec for optimization
    /// Accumulator for resulted path produced by the interpreter after execution.
    pub(crate) new_trace: ExecutionTrace,
}

impl ExecutionTraceCtx {
    pub fn new(current_trace: ExecutionTrace) -> Self {
        let current_subtree_size = current_trace.len();
        // a new execution trace will contain at least current_path.len() elements
        let new_trace = ExecutionTrace::with_capacity(current_subtree_size);

        Self {
            current_trace,
            current_subtree_size,
            new_trace,
        }
    }
}

impl Display for ExecutionTraceCtx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "current trace:\n{:?}", self.current_trace)?;
        writeln!(f, "current subtree elements count:\n{:?}", self.current_subtree_size)?;
        writeln!(f, "new trace:\n{:?}", self.new_trace)
    }
}
