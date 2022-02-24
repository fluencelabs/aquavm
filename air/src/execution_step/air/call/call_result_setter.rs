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

use super::*;
use crate::execution_step::execution_context::*;
use crate::execution_step::Generation;
use crate::execution_step::ValueAggregate;

use air_interpreter_data::CallResult;
use air_interpreter_data::Value;
use air_parser::ast::CallOutputValue;
use air_parser::ast::Variable;
use air_trace_handler::TraceHandler;

/// Writes result of a local `Call` instruction to `ExecutionCtx` at `output`.
/// Returns call result.
pub(crate) fn set_local_result<'i>(
    executed_result: ValueAggregate,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<CallResult> {
    let result_value = executed_result.result.clone();
    match output {
        CallOutputValue::Variable(Variable::Scalar(scalar)) => {
            exec_ctx.scalars.set_value(scalar.name, executed_result)?;
            Ok(CallResult::executed_scalar(result_value))
        }
        CallOutputValue::Variable(Variable::Stream(stream)) => {
            // TODO: refactor this generation handling
            let generation = match exec_ctx.streams.get(stream.name, stream.position) {
                Some(stream) => {
                    let generation = match stream.borrow().generations_count() {
                        0 => 0,
                        n => n - 1,
                    };
                    Generation::Nth(generation as u32)
                }
                None => Generation::Last,
            };

            let generation =
                exec_ctx
                    .streams
                    .add_stream_value(executed_result, generation, stream.name, stream.position)?;
            Ok(CallResult::executed_stream(result_value, generation))
        }
        CallOutputValue::None => Ok(CallResult::executed_scalar(result_value)),
    }
}

pub(crate) fn set_result_from_value<'i>(
    value: Value,
    tetraplet: RcSecurityTetraplet,
    trace_pos: usize,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    match (output, value) {
        (CallOutputValue::Variable(Variable::Scalar(scalar)), Value::Scalar(value)) => {
            let result = ValueAggregate::new(value, tetraplet, trace_pos);
            exec_ctx.scalars.set_value(scalar.name, result)?;
        }
        (CallOutputValue::Variable(Variable::Stream(stream)), Value::Stream { value, generation }) => {
            let result = ValueAggregate::new(value, tetraplet, trace_pos);
            let generation = Generation::Nth(generation);
            let _ = exec_ctx
                .streams
                .add_stream_value(result, generation, stream.name, stream.position)?;
        }
        // it isn't needed to check there that output and value matches because
        // it's been already checked in trace handler
        _ => {}
    };

    Ok(())
}

/// Writes an executed state of a particle being sent to remote node.
pub(crate) fn set_remote_call_result<'i>(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_call_result = CallResult::sent_peer_id(exec_ctx.current_peer_id.clone());
    trace_ctx.meet_call_end(new_call_result);
}
