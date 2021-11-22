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

use super::SecurityTetraplets;
use crate::execution_step::boxed_value::JValuable;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::execution_context::LastErrorWithTetraplet;
use crate::execution_step::ExecutionResult;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_parser::ast;
use air_parser::ast::LastErrorPath;

use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_args<'i>(
    value: &ast::Value<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, SecurityTetraplets)> {
    use ast::Value::*;

    match value {
        InitPeerId => prepare_const(ctx.init_peer_id.clone(), ctx),
        LastError(path) => prepare_last_error(path, ctx),
        Literal(value) => prepare_const(value.to_string(), ctx),
        Boolean(value) => prepare_const(*value, ctx),
        Number(value) => prepare_const(value, ctx),
        EmptyArray => prepare_const(json!([]), ctx),
        Variable(variable) => resolve_ast_variable_wl(variable, ctx),
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn prepare_const(
    arg: impl Into<JValue>,
    ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<(JValue, SecurityTetraplets)> {
    let jvalue = arg.into();
    let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.clone());
    let tetraplet = Rc::new(RefCell::new(tetraplet));

    Ok((jvalue, vec![tetraplet]))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn prepare_last_error(
    path: &LastErrorPath,
    ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<(JValue, SecurityTetraplets)> {
    let LastErrorWithTetraplet {
        last_error,
        tetraplet: tetraplets,
    } = ctx.last_error();
    let jvalue = match path {
        LastErrorPath::Instruction => JValue::String(last_error.instruction),
        LastErrorPath::Message => JValue::String(last_error.msg),
        LastErrorPath::PeerId => JValue::String(last_error.peer_id),
        LastErrorPath::None => json!(last_error),
    };

    Ok((jvalue, vec![tetraplets]))
}

pub(crate) fn resolve_variable<'ctx, 'i>(
    variable: Variable<'_>,
    ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<Box<dyn JValuable + 'ctx>> {
    use crate::execution_step::boxed_value::StreamJvaluableIngredients;

    match variable {
        Variable::Scalar(name) => Ok(ctx.scalars.get(name)?.into_jvaluable()),
        Variable::Stream { name, generation } => {
            match ctx.streams.get(name) {
                Some(stream) => {
                    let ingredients = StreamJvaluableIngredients::new(stream.borrow(), generation);
                    Ok(Box::new(ingredients))
                }
                // return an empty stream for not found stream
                // here it ignores the join behaviour
                None => Ok(Box::new(())),
            }
        }
    }
}

pub(crate) fn resolve_ast_variable_wl<'ctx, 'i>(
    ast_variable: &ast::VariableWithLambda<'_>,
    exec_ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, SecurityTetraplets)> {
    let variable: Variable<'_> = ast_variable.into();
    match ast_variable.lambda() {
        Some(lambda) => apply_lambda(variable, lambda, exec_ctx).map(|(value, tetraplet)| (value, vec![tetraplet])),
        None => {
            let value = resolve_variable(variable, exec_ctx)?;
            let tetraplets = value.as_tetraplets();
            Ok((value.into_jvalue(), tetraplets))
        }
    }
}

pub(crate) fn apply_lambda<'i>(
    variable: Variable<'_>,
    lambda: &LambdaAST<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, RSecurityTetraplet)> {
    let resolved = resolve_variable(variable, exec_ctx)?;
    let (jvalue, tetraplet) = resolved.apply_lambda_with_tetraplets(lambda)?;

    // it's known that apply_lambda_with_tetraplets returns vec of one value
    Ok((jvalue.clone(), tetraplet))
}
