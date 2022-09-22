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

use super::resolved_call::NewOrOldValue;
use super::*;
use crate::execution_step::air::call::call_result_setter::set_result_from_value;
use crate::execution_step::CatchableError;
use crate::execution_step::RcSecurityTetraplet;

use air_interpreter_data::CallResult;
use air_interpreter_data::Sender;
use air_interpreter_data::TracePos;
use air_interpreter_data::Value;
use air_interpreter_interface::CallServiceResult;
use air_parser::ast::CallOutputValue;
use air_trace_handler::TraceHandler;

use fstrings::f;
use fstrings::format_args_f;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StateDescriptor {
    should_execute: bool,
    prev_state: Option<CallResult>,
}

// TODO update as per 12:00
/// This function looks at the existing call state, validates it,
/// and returns Ok(true) if the call should be executed further.
pub(super) fn handle_prev_state<'i>(
    tetraplet: &RcSecurityTetraplet,
    output: &CallOutputValue<'i>,
    new_or_old_value: NewOrOldValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<StateDescriptor> {
    match new_or_old_value {
        NewOrOldValue::NewStreamValue(new_stream_value) => {
            let jvalue = new_stream_value.value.clone();
            let generation = set_result_from_new_value(&new_stream_value, tetraplet.clone(), exec_ctx)?;
            let prev_result = CallResult::Executed(Value::Stream {
                value: jvalue,
                generation,
            });
            trace_ctx.meet_call_end(prev_result);

            Ok(StateDescriptor::executed())
        }
        NewOrOldValue::CallResult { value, trace_pos } => {
            handle_prev_known_state(tetraplet, output, value, trace_pos, exec_ctx, trace_ctx)
        }
    }
}

fn handle_prev_known_state<'i>(
    tetraplet: &RcSecurityTetraplet,
    output: &CallOutputValue<'i>,
    prev_result: CallResult,
    trace_pos: TracePos,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<StateDescriptor> {
    use CallResult::*;

    match &prev_result {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        CallServiceFailed(ret_code, err_msg) => {
            exec_ctx.subgraph_complete = false;
            let ret_code = *ret_code;
            let err_msg = err_msg.clone();
            trace_ctx.meet_call_end(prev_result);
            Err(CatchableError::LocalServiceError(ret_code, err_msg).into())
        }
        RequestSentBy(Sender::PeerIdWithCallId { peer_id, call_id })
            if peer_id.as_str() == exec_ctx.run_parameters.current_peer_id.as_str() =>
        {
            // call results are identified by call_id that is saved in data
            match exec_ctx.call_results.remove(call_id) {
                Some(call_result) => {
                    update_state_with_service_result(tetraplet.clone(), output, call_result, exec_ctx, trace_ctx)?;
                    Ok(StateDescriptor::executed())
                }
                // result hasn't been prepared yet
                None => {
                    exec_ctx.subgraph_complete = false;
                    Ok(StateDescriptor::not_ready(prev_result))
                }
            }
        }
        RequestSentBy(..) => {
            // check whether current node can execute this call
            let is_current_peer = tetraplet.peer_pk.as_str() == exec_ctx.run_parameters.current_peer_id.as_str();
            if is_current_peer {
                return Ok(StateDescriptor::can_execute_now(prev_result));
            }

            exec_ctx.subgraph_complete = false;
            Ok(StateDescriptor::cant_execute_now(prev_result))
        }
        // this instruction's been already executed
        Executed(value) => {
            set_result_from_value(value.clone(), tetraplet.clone(), trace_pos, output, exec_ctx)?;
            trace_ctx.meet_call_end(prev_result);

            Ok(StateDescriptor::executed())
        }
    }
}

use super::call_result_setter::*;
use crate::execution_step::ValueAggregate;
use crate::JValue;

fn update_state_with_service_result<'i>(
    tetraplet: RcSecurityTetraplet,
    output: &CallOutputValue<'i>,
    service_result: CallServiceResult,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    // check that service call succeeded
    let service_result = handle_service_error(service_result, trace_ctx)?;
    // try to get service result from call service result
    let result = try_to_service_result(service_result, trace_ctx)?;

    let trace_pos = trace_ctx.trace_pos();

    let executed_result = ValueAggregate::new(result, tetraplet, trace_pos);
    let new_call_result = set_local_result(executed_result, output, exec_ctx)?;
    trace_ctx.meet_call_end(new_call_result);

    Ok(())
}

fn handle_service_error(
    service_result: CallServiceResult,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<CallServiceResult> {
    use air_interpreter_interface::CALL_SERVICE_SUCCESS;
    use CallResult::CallServiceFailed;

    if service_result.ret_code == CALL_SERVICE_SUCCESS {
        return Ok(service_result);
    }

    let error_message = Rc::new(service_result.result);
    let error = CatchableError::LocalServiceError(service_result.ret_code, error_message.clone());

    trace_ctx.meet_call_end(CallServiceFailed(service_result.ret_code, error_message));

    Err(error.into())
}

fn try_to_service_result(
    service_result: CallServiceResult,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<Rc<JValue>> {
    use CallResult::CallServiceFailed;

    match serde_json::from_str(&service_result.result) {
        Ok(result) => Ok(Rc::new(result)),
        Err(e) => {
            let error_msg =
                f!("call_service result '{service_result}' can't be serialized or deserialized with an error: {e}");
            let error_msg = Rc::new(error_msg);

            let error = CallServiceFailed(i32::MAX, error_msg.clone());
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
