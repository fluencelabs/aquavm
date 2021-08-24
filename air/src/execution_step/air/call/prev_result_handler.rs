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
use crate::execution_step::trace_handler::TraceHandler;
use crate::execution_step::RSecurityTetraplet;

use air_interpreter_data::CallResult;
use air_interpreter_interface::CallServiceResult;
use air_parser::ast::CallOutputValue;

/// This function looks at the existing call state, validates it,
/// and returns Ok(true) if the call should be executed further.
pub(super) fn handle_prev_state<'i>(
    tetraplet: &RSecurityTetraplet,
    output: &CallOutputValue<'i>,
    prev_result: CallResult,
    trace_pos: usize,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<bool> {
    use CallResult::*;

    let result = match &prev_result {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        CallServiceFailed(ret_code, err_msg) => {
            exec_ctx.subtree_complete = false;
            exec_err!(ExecutionError::LocalServiceError(*ret_code, err_msg.clone()))
        }
        RequestSentBy(sent_by) if sent_by.as_str() == exec_ctx.current_peer_id.as_str() => {
            let call_id = exec_ctx.tracker.call.seen_count - exec_ctx.tracker.call.executed_count;
            match exec_ctx.call_results.remove(&call_id) {
                Some(call_result) => {
                    update_state_with_service_result(tetraplet, output, call_result, exec_ctx, trace_ctx)?
                }
                // result hasn't been prepared yet
                None => exec_ctx.subtree_complete = false,
            }
            Ok(false)
        }
        RequestSentBy(..) => {
            // check whether current node can execute this call
            let is_current_peer = tetraplet.borrow().triplet.peer_pk.as_str() == exec_ctx.current_peer_id.as_str();
            if is_current_peer {
                // if this peer could execute this call early return and
                return Ok(true);
            }

            exec_ctx.subtree_complete = false;
            Ok(false)
        }
        // this instruction's been already executed
        Executed(value) => {
            set_result_from_value(value.clone(), tetraplet.clone(), trace_pos, output, exec_ctx)?;

            exec_ctx.subtree_complete = true;
            Ok(false)
        }
    };

    trace_ctx.meet_call_end(prev_result);
    result
}

use super::call_result_setter::*;
use crate::execution_step::ResolvedCallResult;
use crate::JValue;

fn update_state_with_service_result<'i>(
    tetraplet: &RSecurityTetraplet,
    output: &CallOutputValue<'i>,
    service_result: CallServiceResult,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    use ExecutionError::CallServiceResultDeError as DeError;

    // check that service call succeeded
    let service_result = handle_service_error(service_result, trace_ctx)?;

    let result: JValue = serde_json::from_str(&service_result.result).map_err(|e| DeError(service_result, e))?;
    let result = Rc::new(result);

    let trace_pos = trace_ctx.trace_pos();

    let executed_result = ResolvedCallResult::new(result, tetraplet.clone(), trace_pos);
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
