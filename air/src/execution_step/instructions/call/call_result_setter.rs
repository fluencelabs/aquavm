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
use crate::execution_step::ServiceResultAggregate;
use crate::execution_step::ValueAggregate;
use crate::UncatchableError;

use air_interpreter_cid::value_to_json_cid;
use air_interpreter_data::CallResult;
use air_interpreter_data::TracePos;
use air_interpreter_data::ValueRef;
use air_parser::ast::CallOutputValue;
use air_trace_handler::merger::ValueSource;
use air_trace_handler::TraceHandler;

pub(crate) fn populate_context_from_peer_service_result<'i>(
    executed_result: ServiceResultAggregate,
    output: &CallOutputValue<'i>,
    tetraplet: RcSecurityTetraplet,
    argument_hash: Rc<str>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<CallResult> {
    match output {
        CallOutputValue::Scalar(scalar) => {
            let peer_id = tetraplet.peer_pk.clone();
            let service_result_agg_cid =
                exec_ctx
                    .cid_state
                    .track_service_result(executed_result.result.clone(), tetraplet, argument_hash)?;
            let executed_result = ValueAggregate::from_service_result(executed_result, service_result_agg_cid.clone());

            exec_ctx.scalars.set_scalar_value(scalar.name, executed_result)?;
            exec_ctx.record_call_cid(&peer_id, &service_result_agg_cid);
            Ok(CallResult::executed_scalar(service_result_agg_cid))
        }
        CallOutputValue::Stream(stream) => {
            let peer_id = tetraplet.peer_pk.clone();
            let service_result_agg_cid =
                exec_ctx
                    .cid_state
                    .track_service_result(executed_result.result.clone(), tetraplet, argument_hash)?;

            let executed_result = ValueAggregate::from_service_result(executed_result, service_result_agg_cid.clone());

            let value_descriptor =
                StreamValueDescriptor::new(executed_result, stream.name, Generation::New, stream.position);
            exec_ctx.streams.add_stream_value(value_descriptor)?;
            exec_ctx.record_call_cid(&peer_id, &service_result_agg_cid);
            Ok(CallResult::executed_stream_stub(service_result_agg_cid))
        }
        CallOutputValue::None => {
            let value_cid = value_to_json_cid(&*executed_result.result)
                .map_err(UncatchableError::from)?
                .into();

            Ok(CallResult::executed_unused(value_cid))
        }
    }
}

pub(crate) fn populate_context_from_data<'i>(
    value: ValueRef,
    argument_hash: &str,
    tetraplet: RcSecurityTetraplet,
    trace_pos: TracePos,
    value_source: ValueSource,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    match (output, value) {
        (CallOutputValue::Scalar(scalar), ValueRef::Scalar(cid)) => {
            let ResolvedServiceInfo {
                value,
                tetraplet: current_tetraplet,
                service_result_aggregate,
            } = exec_ctx.cid_state.resolve_service_info(&cid)?;

            verifier::verify_call(
                argument_hash,
                &tetraplet,
                &service_result_aggregate.argument_hash,
                &current_tetraplet,
            )?;

            let result = ServiceResultAggregate::new(value, tetraplet, trace_pos);
            let result = ValueAggregate::from_service_result(result, cid);
            exec_ctx.scalars.set_scalar_value(scalar.name, result)?;
        }
        (CallOutputValue::Stream(stream), ValueRef::Stream { cid, generation }) => {
            let ResolvedServiceInfo {
                value,
                tetraplet: current_tetraplet,
                service_result_aggregate,
            } = exec_ctx.cid_state.resolve_service_info(&cid)?;

            verifier::verify_call(
                argument_hash,
                &tetraplet,
                &service_result_aggregate.argument_hash,
                &current_tetraplet,
            )?;

            let result = ServiceResultAggregate::new(value, tetraplet, trace_pos);
            let result = ValueAggregate::from_service_result(result, cid);
            let generation = Generation::from_data(value_source, generation);
            let value_descriptor = StreamValueDescriptor::new(result, stream.name, generation, stream.position);
            exec_ctx.streams.add_stream_value(value_descriptor)?;
        }
        (CallOutputValue::None, ValueRef::Unused(_)) => {}
        (_, value) => {
            return Err(ExecutionError::Uncatchable(
                UncatchableError::CallResultNotCorrespondToInstr(value),
            ))
        }
    }

    Ok(())
}

/// Writes an executed state of a particle being sent to remote node.
pub(crate) fn handle_remote_call(peer_pk: String, exec_ctx: &mut ExecutionCtx<'_>, trace_ctx: &mut TraceHandler) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.make_subgraph_incomplete();

    let new_call_result = CallResult::sent_peer_id(exec_ctx.run_parameters.current_peer_id.clone());
    trace_ctx.meet_call_end(new_call_result);
}
