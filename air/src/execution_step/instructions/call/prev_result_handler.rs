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

use super::*;
use crate::execution_step::execution_context::ResolvedServiceInfo;
use crate::execution_step::instructions::call::call_result_setter::populate_context_from_data;
use crate::execution_step::CatchableError;
use crate::execution_step::RcSecurityTetraplet;
use crate::UncatchableError;

use air_interpreter_data::CallResult;
use air_interpreter_data::CallServiceFailed;
use air_interpreter_data::Sender;
use air_interpreter_interface::CallServiceResult;
use air_parser::ast::CallOutputValue;
use air_trace_handler::merger::MetCallResult;
use air_trace_handler::TraceHandler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StateDescriptor {
    should_execute: bool,
    prev_state: Option<CallResult>,
}

/// This function looks at the existing call state, validates it,
/// and returns Ok(true) if the call should be executed further.
pub(super) fn handle_prev_state<'i>(
    met_result: MetCallResult,
    tetraplet: &RcSecurityTetraplet,
    argument_hash: Option<&Rc<str>>,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<StateDescriptor> {
    use CallResult::*;

    match met_result.result {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        Failed(ref failed_cid) => {
            let ResolvedServiceInfo {
                value: err_value,
                tetraplet: current_tetraplet,
                service_result_aggregate,
            } = exec_ctx
                .cid_state
                .resolve_service_info(failed_cid)
                .map_err(UncatchableError::from)?;

            verifier::verify_call(
                argument_hash.as_ref().unwrap(),
                tetraplet,
                &service_result_aggregate.argument_hash,
                &current_tetraplet,
            )?;

            let call_service_failed: CallServiceFailed =
                serde_json::from_value(serde_json::to_value(&err_value).expect("TODO"))
                    .map_err(UncatchableError::MalformedCallServiceFailed)?;

            exec_ctx.make_subgraph_incomplete();
            exec_ctx.record_call_cid(&tetraplet.peer_pk, failed_cid);
            trace_ctx.meet_call_end(met_result.result);

            let err_msg = call_service_failed.message;
            Err(CatchableError::LocalServiceError(call_service_failed.ret_code, err_msg).into())
        }
        RequestSentBy(Sender::PeerIdWithCallId { ref peer_id, call_id })
            if peer_id.as_str() == exec_ctx.run_parameters.current_peer_id.as_str() =>
        {
            // call results are identified by call_id that is saved in data;
            // for compatiblity with JavaScript with binary formats, string IDs are used
            let call_id = call_id.to_string();
            match exec_ctx.call_results.remove(&call_id) {
                Some(call_result) => {
                    update_state_with_service_result(
                        tetraplet.clone(),
                        argument_hash.expect("Result for joinable error").clone(),
                        output,
                        call_result,
                        exec_ctx,
                        trace_ctx,
                    )?;
                    Ok(StateDescriptor::executed())
                }
                // result hasn't been prepared yet
                None => {
                    exec_ctx.make_subgraph_incomplete();
                    Ok(StateDescriptor::not_ready(met_result.result))
                }
            }
        }
        RequestSentBy(..) => {
            // check whether current node can execute this call
            let is_current_peer = tetraplet.peer_pk.as_str() == exec_ctx.run_parameters.current_peer_id.as_str();
            if is_current_peer {
                return Ok(StateDescriptor::can_execute_now(met_result.result));
            }

            exec_ctx.make_subgraph_incomplete();
            Ok(StateDescriptor::cant_execute_now(met_result.result))
        }
        // this instruction's been already executed
        Executed(value) => {
            use air_interpreter_data::ValueRef;

            populate_context_from_data(
                value.clone(),
                argument_hash.as_ref().unwrap(),
                tetraplet.clone(),
                met_result.trace_pos,
                met_result.source,
                output,
                exec_ctx,
            )?;

            match &value {
                ValueRef::Scalar(ref cid) | ValueRef::Stream { ref cid, .. } => {
                    exec_ctx.record_call_cid(&tetraplet.peer_pk, cid);
                }
                ValueRef::Unused(_) => {}
            }

            let call_result = CallResult::Executed(value);
            trace_ctx.meet_call_end(call_result);

            Ok(StateDescriptor::executed())
        }
    }
}

use super::call_result_setter::*;
use crate::execution_step::ServiceResultAggregate;
use crate::JValue;

fn update_state_with_service_result<'i>(
    tetraplet: RcSecurityTetraplet,
    argument_hash: Rc<str>,
    output: &CallOutputValue<'i>,
    service_result: CallServiceResult,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    // check that service call succeeded
    let service_result = handle_service_error(
        service_result,
        argument_hash.clone(),
        tetraplet.clone(),
        exec_ctx,
        trace_ctx,
    )?;

    // try to get service result from call service result
    let result = try_to_service_result(service_result, &argument_hash, &tetraplet, exec_ctx, trace_ctx)?;

    let trace_pos = trace_ctx.trace_pos().map_err(UncatchableError::from)?;

    let executed_result = ServiceResultAggregate::new(result, tetraplet.clone(), trace_pos);
    let new_call_result =
        populate_context_from_peer_service_result(executed_result, output, tetraplet, argument_hash, exec_ctx)?;
    trace_ctx.meet_call_end(new_call_result);

    Ok(())
}

fn handle_service_error(
    service_result: CallServiceResult,
    argument_hash: Rc<str>,
    tetraplet: RcSecurityTetraplet,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<CallServiceResult> {
    use air_interpreter_interface::CALL_SERVICE_SUCCESS;
    use CallResult::Failed;

    if service_result.ret_code == CALL_SERVICE_SUCCESS {
        return Ok(service_result);
    }

    let error_message = Rc::new(service_result.result.clone());
    let error = CatchableError::LocalServiceError(service_result.ret_code, error_message.clone());

    let failed_value = CallServiceFailed::new(service_result.ret_code, error_message).to_value();

    let peer_id = tetraplet.peer_pk.clone();
    let service_result_agg_cid = exec_ctx
        .cid_state
        .track_service_result(failed_value, tetraplet, argument_hash)?;

    exec_ctx.record_call_cid(&peer_id, &service_result_agg_cid);
    trace_ctx.meet_call_end(Failed(service_result_agg_cid));

    Err(error.into())
}

fn try_to_service_result(
    service_result: CallServiceResult,
    argument_hash: &Rc<str>,
    tetraplet: &RcSecurityTetraplet,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<JValue> {
    match serde_json::from_str(&service_result.result) {
        Ok(result) => Ok(result),
        Err(e) => {
            let error_msg = format!(
                "call_service result '{service_result}' can't be serialized or deserialized with an error: {e}"
            );
            let error_msg = Rc::new(error_msg);

            let failed_value = CallServiceFailed::new(i32::MAX, error_msg.clone()).to_value();

            let service_result_agg_cid =
                exec_ctx
                    .cid_state
                    .track_service_result(failed_value, tetraplet.clone(), argument_hash.clone())?;
            let error = CallResult::failed(service_result_agg_cid);

            trace_ctx.meet_call_end(error);

            Err(CatchableError::LocalServiceError(i32::MAX, error_msg).into())
        }
    }
}

impl StateDescriptor {
    pub(crate) fn executed() -> Self {
        Self {
            should_execute: false,
            prev_state: None,
        }
    }

    pub(crate) fn not_ready(prev_state: CallResult) -> Self {
        Self {
            should_execute: false,
            prev_state: Some(prev_state),
        }
    }

    pub(crate) fn can_execute_now(prev_state: CallResult) -> Self {
        Self {
            should_execute: true,
            prev_state: Some(prev_state),
        }
    }

    pub(crate) fn cant_execute_now(prev_state: CallResult) -> Self {
        Self {
            should_execute: false,
            prev_state: Some(prev_state),
        }
    }

    pub(crate) fn no_previous_state() -> Self {
        Self {
            should_execute: true,
            prev_state: None,
        }
    }

    pub(crate) fn should_execute(&self) -> bool {
        self.should_execute
    }

    pub(crate) fn maybe_set_prev_state(self, trace_ctx: &mut TraceHandler) {
        if let Some(call_result) = self.prev_state {
            trace_ctx.meet_call_end(call_result);
        }
    }
}
