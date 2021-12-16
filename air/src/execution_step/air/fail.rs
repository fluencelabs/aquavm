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
use super::LastErrorDescriptor;
use super::TraceHandler;
use crate::log_instruction;

use air_parser::ast::Fail;
use polyplets::SecurityTetraplet;

use std::cell::RefCell;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Fail<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fail, exec_ctx, trace_ctx);

        match self {
            &Fail::Literal {
                ret_code,
                error_message,
            } => fail_with_literals(ret_code, error_message, self, exec_ctx),
            // bubble last error up
            Fail::LastError => fail_with_last_error(exec_ctx),
        }
    }
}

fn fail_with_literals<'i>(
    ret_code: i64,
    error_message: &str,
    fail: &Fail<'_>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    let fail_error = ExecutionError::FailWithoutXorError {
        ret_code,
        error_message: error_message.to_string(),
    };
    let fail_error = Rc::new(fail_error);
    let instruction = fail.to_string();

    // TODO: wrap exec.init_peer_id in Rc
    let literal_tetraplet = SecurityTetraplet::literal_tetraplet(exec_ctx.init_peer_id.clone());
    let literal_tetraplet = Rc::new(RefCell::new(literal_tetraplet));

    let last_error = LastErrorDescriptor::new(
        fail_error.clone(),
        instruction,
        // init_peer_id is used here in order to obtain determinism,
        // so %last_error%.peer_id will produce the same result on each peer
        exec_ctx.init_peer_id.clone(),
        Some(literal_tetraplet),
    );
    exec_ctx.last_error = Some(last_error);

    update_context_state(exec_ctx);

    Err(fail_error)
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
