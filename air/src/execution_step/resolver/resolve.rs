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

use super::RcSecurityTetraplets;
use crate::execution_step::boxed_value::JValuable;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::lambda_applier::select_from_scalar;
use crate::execution_step::ExecutionResult;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_parser::ast;

use serde_json::json;
use std::rc::Rc;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_args<'i>(
    value: &ast::Value<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, RcSecurityTetraplets)> {
    use ast::Value::*;

    match value {
        InitPeerId => prepare_const(ctx.run_parameters.init_peer_id.as_str(), ctx),
        LastError(error_accessor) => prepare_last_error(error_accessor, ctx),
        Literal(value) => prepare_const(value.to_string(), ctx),
        Timestamp => prepare_const(ctx.run_parameters.timestamp, ctx),
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
) -> ExecutionResult<(JValue, RcSecurityTetraplets)> {
    let jvalue = arg.into();
    let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.run_parameters.init_peer_id.as_ref());
    let tetraplet = Rc::new(tetraplet);

    Ok((jvalue, vec![tetraplet]))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn prepare_last_error<'i>(
    error_accessor: &Option<LambdaAST<'i>>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, RcSecurityTetraplets)> {
    use crate::LastError;

    let LastError { error, tetraplet } = ctx.last_error();

    let jvalue = match error_accessor {
        Some(error_accessor) => select_from_scalar(error.as_ref(), error_accessor.iter(), ctx)?,
        None => error.as_ref(),
    };

    let tetraplets = match tetraplet {
        Some(tetraplet) => vec![tetraplet.clone()],
        None => {
            let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.run_parameters.init_peer_id.as_ref());
            let tetraplet = Rc::new(tetraplet);
            vec![tetraplet]
        }
    };

    Ok((jvalue.clone(), tetraplets))
}

pub(crate) fn resolve_variable<'ctx, 'i>(
    variable: Variable<'_>,
    ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<Box<dyn JValuable + 'ctx>> {
    use crate::execution_step::boxed_value::StreamJvaluableIngredients;

    match variable {
        Variable::Scalar { name, .. } => Ok(ctx.scalars.get_value(name)?.into_jvaluable()),
        Variable::Stream {
            name,
            generation,
            position,
        } => {
            match ctx.streams.get(name, position) {
                Some(stream) => {
                    let ingredients = StreamJvaluableIngredients::new(stream, generation);
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
) -> ExecutionResult<(JValue, RcSecurityTetraplets)> {
    let variable: Variable<'_> = ast_variable.into();
    match ast_variable.lambda() {
        Some(lambda) => apply_lambda(variable, lambda, exec_ctx).map(|(value, tetraplet)| {
            let tetraplet = Rc::new(tetraplet);
            (value, vec![tetraplet])
        }),
        None => {
            let value = resolve_variable(variable, exec_ctx)?;
            let tetraplets = value.as_tetraplets();
            Ok((value.into_jvalue(), tetraplets))
        }
    }
}

pub(crate) fn resolve_ast_scalar_wl<'ctx, 'i>(
    ast_scalar: &ast::ScalarWithLambda<'_>,
    exec_ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, RcSecurityTetraplets)> {
    // TODO: wrap lambda path with Rc to make this clone cheaper
    let variable = ast::VariableWithLambda::Scalar(ast_scalar.clone());
    resolve_ast_variable_wl(&variable, exec_ctx)
}

pub(crate) fn apply_lambda<'i>(
    variable: Variable<'_>,
    lambda: &LambdaAST<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, SecurityTetraplet)> {
    let resolved = resolve_variable(variable, exec_ctx)?;
    let (jvalue, tetraplet) = resolved.apply_lambda_with_tetraplets(lambda, exec_ctx)?;

    // it's known that apply_lambda_with_tetraplets returns vec of one value
    Ok((jvalue.clone(), tetraplet))
}
