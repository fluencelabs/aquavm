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

use super::ExecutionTrace;
use super::TraceSlider;

use air_interpreter_data::InterpreterData;
use air_interpreter_data::StreamGenerations;

use std::collections::HashMap;

/// Contains all necessary information about data.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct MergeCtx {
    pub(crate) slider: TraceSlider,
    pub(crate) streams: StreamGenerations,
    /// This value is used to track the whole trace that each fold is described.
    /// total_subtrace_len and subtrace_len from a slider are changed in the following way:
    ///     fold:
    ///         start: total = fold_states_count, subtrace_len = len of the first iteration
    ///         i iteration_end: total -= iteration_i len, subtrace_len = len of the i+1 iteration
    ///         end: total = 0
    ///     par => total -= [left, right], new_subtrace_len = total - [left, right], pos += [left, right]
    total_subtrace_len: usize,
}

impl MergeCtx {
    #[allow(dead_code)]
    pub(crate) fn from_trace(trace: ExecutionTrace) -> Self {
        let total_subtrace_len = trace.len();
        let slider = TraceSlider::new(trace);

        Self {
            slider,
            streams: HashMap::new(),
            total_subtrace_len,
        }
    }

    pub(crate) fn from_data(data: InterpreterData) -> Self {
        let total_subtrace_len = data.trace.len();
        let slider = TraceSlider::new(data.trace);

        Self {
            slider,
            streams: data.streams,
            total_subtrace_len,
        }
    }

    pub(crate) fn stream_generation(&self, stream_name: &str) -> Option<u32> {
        self.streams.get(stream_name).copied()
    }

    pub(crate) fn set_total_subtrace_len(&mut self, total_subtrace_len: usize) {
        self.total_subtrace_len = total_subtrace_len;
    }

    pub(crate) fn total_subtrace_len(&self) -> usize {
        self.total_subtrace_len
    }
}
