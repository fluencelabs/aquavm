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

use super::call_result_setter::set_local_call_result;
use super::*;
use crate::exec_err;
use crate::execution_step::trace_handler::TraceHandler;
use crate::execution_step::Generation;
use crate::execution_step::RSecurityTetraplet;

use air_interpreter_data::CallResult;
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
        Executed(result, generation) => {
            let executed_result = ResolvedCallResult::new(result.clone(), tetraplet.clone(), trace_pos);
            set_local_call_result(executed_result, Generation::Nth(*generation), output, exec_ctx)?;

            exec_ctx.subtree_complete = true;
            Ok(false)
        }
    };

    trace_ctx.meet_call_end(prev_result);
    result
}
