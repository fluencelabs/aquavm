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

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::TraceHandler;
use crate::exec_err;
use crate::log_instruction;

use air_parser::ast::Fail;

impl<'i> super::ExecutableInstruction<'i> for Fail<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fail, exec_ctx, trace_ctx);

        match self {
            &Fail::Literal {
                ret_code,
                error_message,
            } => fail_with_literals(ret_code, error_message, exec_ctx),
            // bubble last error up
            Fail::LastError => fail_with_last_error(exec_ctx),
        }
    }
}

fn fail_with_literals<'i>(ret_code: i64, error_message: &str, exec_ctx: &mut ExecutionCtx<'i>) -> ExecutionResult<()> {
    update_context_state(exec_ctx);
    exec_err!(ExecutionError::FailWithoutXorError {
        ret_code,
        error_message: error_message.to_string(),
    })
}

fn fail_with_last_error(exec_ctx: &mut ExecutionCtx<'_>) -> ExecutionResult<()> {
    let last_error = match &exec_ctx.last_error {
        Some(last_error) => last_error.error.clone(),
        None => return Ok(()),
    };

    update_context_state(exec_ctx);
    Err(last_error)
}

fn update_context_state(exec_ctx: &mut ExecutionCtx<'_>) {
    exec_ctx.subtree_complete = false;
    exec_ctx.last_error_could_be_set = false;
}
