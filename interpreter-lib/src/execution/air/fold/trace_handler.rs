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

use crate::contexts::execution_trace::ExecutionTraceCtx;
use crate::contexts::execution_trace::FoldStatePositions;
use crate::contexts::execution_trace::ExecutedState;
use crate::contexts::execution_trace::FoldResult;

/// This is an automate
pub(super) struct TraceHandler {
    last_iteration_start_pos: Option<usize>,
    last_next_start_pos: Option<usize>,
    fold_positions: Vec<FoldStatePositions>,
    fold_execution_trace_pos: usize,
}

impl TraceHandler {
    pub(super) fn met_fold_start(trace_ctx: &mut ExecutionTraceCtx<'_>, states_count_hint: usize) -> Self {
        let fold_execution_trace_pos = trace_ctx.new_trace.len();
        trace_ctx.new_trace.push_back(ExecutedState::Fold())
        Self {
            last_iteration_start_pos: None,
            last_next_start_pos: None,
            fold_positions: Vec::with_capacity(states_count_hint),
            fold_execution_trace_pos:
        }
    }

    pub(super) fn met_fold_end(&self, exec_ctx: &mut ExecutionCtx<'_>) {

    }

    pub(super) fn met_next_start(&mut self) {

    }

    pub(super) fn met_next_end(&mut self) {

    }
}
