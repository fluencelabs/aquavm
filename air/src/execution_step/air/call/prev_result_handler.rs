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
use crate::exec_err;
use crate::execution_step::air::call::call_result_setter::set_result_from_value;
use crate::execution_step::RSecurityTetraplet;

use air_interpreter_data::CallResult;
use air_interpreter_data::Sender;
use air_interpreter_interface::CallServiceResult;
use air_parser::ast::CallOutputValue;
use air_trace_handler::TraceHandler;

pub(crate) struct StateDescriptor {
    should_execute: bool,
    prev_state: Option<CallResult>,
}

/// This function looks at the existing call state, validates it,
/// and returns Ok(true) if the call should be executed further.
pub(super) fn handle_prev_state<'i>(
    tetraplet: &RSecurityTetraplet,
    output: &CallOutputValue<'i>,
    prev_result: CallResult,
    trace_pos: usize,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<StateDescriptor> {
    use CallResult::*;

    match &prev_result {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        CallServiceFailed(ret_code, err_msg) => {
            exec_ctx.subtree_complete = false;
            exec_err!(ExecutionError::LocalServiceError(*ret_code, err_msg.clone()))
        }
        RequestSentBy(Sender::PeerIdWithCallId { peer_id, call_id })
            if peer_id.as_str() == exec_ctx.current_peer_id.as_str() =>
        {
            // call results are identified by call_id that is saved in data
            match exec_ctx.call_results.remove(call_id) {
                Some(call_result) => {
                    update_state_with_service_result(tetraplet, output, call_result, exec_ctx, trace_ctx)?;
                    return Ok(StateDescriptor::executed());
                }
                // result hasn't been prepared yet
                None => {
                    exec_ctx.subtree_complete = false;
                    Ok(StateDescriptor::not_prepared(prev_result))
                }
            }
        }
        RequestSentBy(..) => {
            // check whether current node can execute this call
            let is_current_peer = tetraplet.borrow().peer_pk.as_str() == exec_ctx.current_peer_id.as_str();
            if is_current_peer {
                return Ok(StateDescriptor::can_execute_now(prev_result));
            }

            exec_ctx.subtree_complete = false;
            Ok(StateDescriptor::cant_execute_now(prev_result))
        }
        // this instruction's been already executed
        Executed(value) => {
            set_result_from_value(value.clone(), tetraplet.clone(), trace_pos, output, exec_ctx)?;

            exec_ctx.subtree_complete = true;
            Ok(StateDescriptor::executed())
        }
    }
}

use super::call_result_setter::*;
use crate::execution_step::ValueAggregate;
use crate::JValue;

fn update_state_with_service_result<'i>(
    tetraplet: &RSecurityTetraplet,
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

    let executed_result = ValueAggregate::new(result, tetraplet.clone(), trace_pos);
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
    let error = ExecutionError::LocalServiceError(service_result.ret_code, error_message.clone());
    let error = Rc::new(error);

    trace_ctx.meet_call_end(CallServiceFailed(service_result.ret_code, error_message));

    Err(error)
}

fn try_to_service_result(
    service_result: CallServiceResult,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<Rc<JValue>> {
    use CallResult::CallServiceFailed;

    match serde_json::from_str(&service_result.result) {
        Ok(result) => Ok(Rc::new(result)),
        Err(e) => {
            let error_msg = format!(
                "call_service result '{0}' can't be serialized or deserialized with an error: {1}",
                service_result.result, e
            );
            let error_msg = Rc::new(error_msg);

            let error = CallServiceFailed(i32::MAX, error_msg.clone());
            trace_ctx.meet_call_end(error);

            Err(Rc::new(ExecutionError::LocalServiceError(i32::MAX, error_msg)))
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

    pub(crate) fn not_prepared(prev_state: CallResult) -> Self {
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
