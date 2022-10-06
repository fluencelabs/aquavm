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

use crate::UncatchableError;
use air_interpreter_data::CallResult;
use air_interpreter_data::TracePos;
use air_interpreter_data::Value;
use air_parser::ast::CallOutputValue;
use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;

pub(crate) fn populate_context_from_peer_service_result<'i>(
    executed_result: ValueAggregate,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<CallResult> {
    let result_value = executed_result.result.clone();
    match output {
        CallOutputValue::Scalar(scalar) => {
            exec_ctx.scalars.set_scalar_value(scalar.name, executed_result)?;
            Ok(CallResult::executed_scalar(result_value))
        }
        CallOutputValue::Stream(stream) => {
            let generation =
                exec_ctx
                    .streams
                    .add_stream_value(executed_result, Generation::Last, stream.name, stream.position)?;
            Ok(CallResult::executed_stream(result_value, generation))
        }
        // by the internal conventions if call has no output value,
        // corresponding data should have scalar type
        CallOutputValue::None => Ok(CallResult::executed_scalar(result_value)),
    }
}

pub(crate) fn populate_context_from_data<'i>(
    value: Value,
    tetraplet: RcSecurityTetraplet,
    trace_pos: TracePos,
    value_source: ValueSource,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<Value> {
    match (output, value) {
        (CallOutputValue::Scalar(scalar), Value::Scalar(value)) => {
            let result = ValueAggregate::new(value.clone(), tetraplet, trace_pos);
            exec_ctx.scalars.set_scalar_value(scalar.name, result)?;
            Ok(Value::Scalar(value))
        }
        (CallOutputValue::Stream(stream), Value::Stream { value, generation }) => {
            let result = ValueAggregate::new(value.clone(), tetraplet, trace_pos);
            let adjusted_generation = maybe_adjust_generation(generation, value_source);
            let resulted_generation =
                exec_ctx
                    .streams
                    .add_stream_value(result, adjusted_generation, stream.name, stream.position)?;

            let result = Value::Stream {
                value,
                generation: resulted_generation,
            };
            Ok(result)
        }
        // by the internal conventions if call has no output value,
        // corresponding data should have scalar type
        (CallOutputValue::None, value @ Value::Scalar(_)) => Ok(value),
        (_, value) => Err(ExecutionError::Uncatchable(
            UncatchableError::CallResultNotCorrespondToInstr(value),
        )),
    }
}

/// Writes an executed state of a particle being sent to remote node.
pub(crate) fn handle_remote_call<'i>(peer_pk: String, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subgraph_complete = false;

    let new_call_result = CallResult::sent_peer_id(exec_ctx.run_parameters.current_peer_id.clone());
    trace_ctx.meet_call_end(new_call_result);
}

fn maybe_adjust_generation(prev_stream_generation: u32, value_source: ValueSource) -> Generation {
    match value_source {
        ValueSource::PreviousData => Generation::Nth(prev_stream_generation),
        ValueSource::CurrentData => Generation::Last,
    }
}
