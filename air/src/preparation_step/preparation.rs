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

use super::PreparationError;
use crate::build_targets::get_current_peer_id;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::Stream;
use crate::execution_step::TraceHandler;
use crate::log_targets::RUN_PARAMS;

use air_interpreter_data::InterpreterData;
use air_parser::ast::Instruction;

type PreparationResult<T> = Result<T, PreparationError>;

/// Represents result of the preparation_step step.
pub(crate) struct PreparationDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) trace_handler: TraceHandler,
    pub(crate) air: Instruction<'i>,
}

/// Parse and prepare supplied data and AIR script.
pub(crate) fn prepare<'i>(
    prev_data: &[u8],
    current_data: &[u8],
    raw_air: &'i str,
    call_results: &[u8],
    init_peer_id: String,
) -> PreparationResult<PreparationDescriptor<'static, 'i>> {
    let prev_data = try_to_data(prev_data)?;
    let current_data = try_to_data(current_data)?;

    let air: Instruction<'i> = *air_parser::parse(raw_air).map_err(PreparationError::AIRParseError)?;

    log::trace!(
        target: RUN_PARAMS,
        "air: {:?}\nprev_trace: {:?}\ncurrent_trace: {:?}",
        air,
        prev_data,
        current_data
    );

    let exec_ctx = make_exec_ctx(init_peer_id, &prev_data, call_results)?;
    let trace_handler = TraceHandler::from_data(prev_data, current_data);

    let result = PreparationDescriptor {
        exec_ctx,
        trace_handler,
        air,
    };

    Ok(result)
}

fn try_to_data(raw_data: &[u8]) -> PreparationResult<InterpreterData> {
    use PreparationError::DataDeFailed;

    InterpreterData::try_from_slice(raw_data).map_err(|err| DataDeFailed(err, raw_data.to_vec()))
}

fn make_exec_ctx(
    init_peer_id: String,
    prev_data: &InterpreterData,
    call_results: &[u8],
) -> PreparationResult<ExecutionCtx<'static>> {
    let current_peer_id = get_current_peer_id().map_err(PreparationError::CurrentPeerIdEnvError)?;
    log::trace!(target: RUN_PARAMS, "current peer id {}", current_peer_id);

    let call_results = serde_json::from_slice(call_results)
        .map_err(|e| PreparationError::CallResultsDeFailed(e, call_results.to_vec()))?;

    let mut ctx = ExecutionCtx::new(current_peer_id, init_peer_id, call_results);
    create_streams(&mut ctx, prev_data);

    Ok(ctx)
}

fn create_streams(ctx: &mut ExecutionCtx<'_>, prev_data: &InterpreterData) {
    use std::cell::RefCell;

    for (stream_name, generation_count) in prev_data.streams.iter() {
        let new_stream = Stream::from_generations_count(*generation_count as usize);
        let new_stream = RefCell::new(new_stream);

        // it's impossible to have duplicates of streams in data because of HashMap in data
        ctx.streams.insert(stream_name.to_string(), new_stream);
    }
}
