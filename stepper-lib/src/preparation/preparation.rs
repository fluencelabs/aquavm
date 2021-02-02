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

use super::merge_execution_traces;
use super::ExecutionCtx;
use super::ExecutionTrace;
use super::ExecutionTraceCtx;
use super::PreparationError;
use crate::build_targets::get_current_peer_id;
use crate::log_targets::RUN_PARAMS;

use air_parser::ast::Instruction;

type PreparationResult<T> = Result<T, PreparationError>;

/// Represents result of the preparation step.
pub(crate) struct PreparationDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) trace_ctx: ExecutionTraceCtx,
    pub(crate) aqua: Instruction<'i>,
}

/// Parse and prepare supplied data and aqua script.
pub(crate) fn prepare<'i>(
    prev_data: &[u8],
    data: &[u8],
    raw_aqua: &'i str,
    init_peer_id: String,
) -> PreparationResult<PreparationDescriptor<'static, 'i>> {
    fn to_executed_trace(raw_data: &[u8]) -> PreparationResult<ExecutionTrace> {
        use PreparationError::ExecutedTraceDeError as CallDeError;

        // treat empty string as an empty executed trace allows abstracting from
        // the internal format for empty data.
        if raw_data.is_empty() {
            Ok(ExecutionTrace::new())
        } else {
            serde_json::from_slice(&raw_data).map_err(|err| CallDeError(err, raw_data.to_vec()))
        }
    }

    let prev_trace = to_executed_trace(prev_data)?;
    let trace = to_executed_trace(data)?;

    let aqua: Instruction<'i> = *air_parser::parse(raw_aqua).map_err(PreparationError::AIRParseError)?;

    log::trace!(
        target: RUN_PARAMS,
        "aqua: {:?}\nprev_trace: {:?}\ncurrent_trace: {:?}",
        aqua,
        prev_trace,
        trace
    );

    let (exec_ctx, trace_ctx) = make_contexts(prev_trace, trace, init_peer_id, &aqua)?;
    let result = PreparationDescriptor {
        exec_ctx,
        trace_ctx,
        aqua,
    };

    Ok(result)
}

/// Make execution and execution trace contexts from supplied data.
/// Internally, it unites variable from previous and current data and merges executed traces.
fn make_contexts<'i>(
    prev_trace: ExecutionTrace,
    trace: ExecutionTrace,
    init_peer_id: String,
    aqua: &Instruction<'i>,
) -> PreparationResult<(ExecutionCtx<'static>, ExecutionTraceCtx)> {
    let current_peer_id = get_current_peer_id().map_err(|e| PreparationError::CurrentPeerIdEnvError(e))?;
    log::trace!(target: RUN_PARAMS, "current peer id {}", current_peer_id);

    let exec_ctx = ExecutionCtx::new(current_peer_id, init_peer_id);
    let current_trace = merge_execution_traces(prev_trace, trace, aqua)?;
    let trace_ctx = ExecutionTraceCtx::new(current_trace);

    Ok((exec_ctx, trace_ctx))
}
