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
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::CatchableError;
use crate::execution_step::LastError;
use crate::execution_step::RSecurityTetraplet;
use crate::log_instruction;
use crate::ExecutionError;
use crate::JValue;

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
    // TODO: decouple error object creation into a separate function
    let error_object = serde_json::json!({
        "error_code": ret_code,
        "message": error_message,
        "instruction": fail.to_string(),
    });
    let error_object = Rc::new(error_object);

    // TODO: wrap exec.init_peer_id in Rc
    let literal_tetraplet = SecurityTetraplet::literal_tetraplet(exec_ctx.init_peer_id.clone());
    let literal_tetraplet = Rc::new(RefCell::new(literal_tetraplet));

    fail_with_error_object(exec_ctx, error_object, Some(literal_tetraplet))
}

fn fail_with_last_error(exec_ctx: &mut ExecutionCtx<'_>) -> ExecutionResult<()> {
    let LastError { error, tetraplet } = exec_ctx.last_error_descriptor.last_error();

    // to avoid warnings from https://github.com/rust-lang/rust/issues/59159
    let error = error.clone();
    let tetraplet = tetraplet.clone();

    fail_with_error_object(exec_ctx, error, tetraplet)
}

fn fail_with_error_object(
    exec_ctx: &mut ExecutionCtx<'_>,
    error: Rc<JValue>,
    tetraplet: Option<RSecurityTetraplet>,
) -> ExecutionResult<()> {
    exec_ctx
        .last_error_descriptor
        .set_from_error_object(error.clone(), tetraplet);
    exec_ctx.subtree_complete = false;

    Err(ExecutionError::Catchable(Rc::new(CatchableError::UserError { error })))
}
