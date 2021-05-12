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

mod merge_ctx;
#[cfg(test)]
mod tests;
mod trace_merger;
mod trace_slider;

pub(self) type MergeResult<T> = Result<T, crate::preparation::DataMergingError>;

use crate::contexts::execution_trace::ExecutionTrace;
use trace_merger::TraceMerger;

use air_parser::ast::Instruction;

pub(crate) fn merge_execution_traces<'i>(
    prev_trace: ExecutionTrace,
    current_trace: ExecutionTrace,
    air: &'i Instruction<'i>,
) -> MergeResult<ExecutionTrace> {
    let trace_merger = TraceMerger::new(prev_trace, current_trace, air);
    trace_merger.merge()
}

pub(self) use merge_ctx::MergeCtx;
pub(self) use trace_slider::TraceSlider;
