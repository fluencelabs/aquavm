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
use crate::execution_step::execution_context::check_error_object;
use crate::execution_step::resolver::Resolvable;
use crate::execution_step::CatchableError;
use crate::execution_step::LastError;
use crate::execution_step::RcSecurityTetraplet;
use crate::log_instruction;
use crate::ExecutionError;
use crate::JValue;

use air_parser::ast;
use air_parser::ast::Fail;
use polyplets::SecurityTetraplet;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Fail<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fail, exec_ctx, trace_ctx);

        match self {
            Fail::Scalar(scalar) => fail_with_scalar(scalar, exec_ctx),
            Fail::ScalarWithLambda(scalar) => fail_with_scalar_wl(scalar, exec_ctx),
            &Fail::Literal {
                ret_code,
                error_message,
            } => fail_with_literals(ret_code, error_message, self, exec_ctx),
            Fail::CanonStreamWithLambda(canon_stream) => fail_with_canon_stream(canon_stream, exec_ctx),
            // bubble last error up
            Fail::LastError => fail_with_last_error(exec_ctx),
        }
    }
}

fn fail_with_scalar<'i>(scalar: &ast::Scalar<'i>, exec_ctx: &mut ExecutionCtx<'i>) -> ExecutionResult<()> {
    let (value, mut tetraplet) = scalar.resolve(exec_ctx)?;
    // tetraplets always have one element here and it'll be refactored after boxed value
    let tetraplet = tetraplet.remove(0);
    check_error_object(&value).map_err(CatchableError::InvalidLastErrorObjectError)?;

    fail_with_error_object(exec_ctx, Rc::new(value), Some(tetraplet))
}

fn fail_with_scalar_wl<'i>(scalar: &ast::ScalarWithLambda<'i>, exec_ctx: &mut ExecutionCtx<'i>) -> ExecutionResult<()> {
    let (value, mut tetraplet) = scalar.resolve(exec_ctx)?;
    // tetraplets always have one element here and it'll be refactored after boxed value
    let tetraplet = tetraplet.remove(0);
    check_error_object(&value).map_err(CatchableError::InvalidLastErrorObjectError)?;

    fail_with_error_object(exec_ctx, Rc::new(value), Some(tetraplet))
}

fn fail_with_literals(
    error_code: i64,
    error_message: &str,
    fail: &Fail<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<()> {
    let error_object = crate::execution_step::execution_context::error_from_raw_fields(
        error_code,
        error_message,
        &fail.to_string(),
        exec_ctx.run_parameters.init_peer_id.as_ref(),
    );

    let literal_tetraplet = SecurityTetraplet::literal_tetraplet(exec_ctx.run_parameters.init_peer_id.as_ref());
    let literal_tetraplet = Rc::new(literal_tetraplet);

    fail_with_error_object(exec_ctx, Rc::new(error_object), Some(literal_tetraplet))
}

fn fail_with_canon_stream(
    ast_canon: &ast::CanonStreamWithLambda<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<()> {
    let (value, mut tetraplets) = ast_canon.resolve(exec_ctx)?;

    // tetraplets always have one element here and it'll be refactored after boxed value
    check_error_object(&value).map_err(CatchableError::InvalidLastErrorObjectError)?;

    fail_with_error_object(exec_ctx, Rc::new(value), Some(tetraplets.remove(0)))
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
    tetraplet: Option<RcSecurityTetraplet>,
) -> ExecutionResult<()> {
    exec_ctx
        .last_error_descriptor
        .set_from_error_object(error.clone(), tetraplet);
    exec_ctx.make_subgraph_incomplete();

    Err(ExecutionError::Catchable(Rc::new(CatchableError::UserError { error })))
}
