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
use crate::execution_step::execution_context::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::SecurityTetraplet;

use air_lambda_ast::AIRLambdaAST;
use air_parser::ast;
use air_values::boxed_value::AIRValueAlgebra;
use air_values::boxed_value::BoxedValue;
use air_values::boxed_value::RcBoxedValue;
use air_values::boxed_value::ValueWithTetraplet;
use air_values::stream::Stream;
use air_values::variable::Variable;

use std::rc::Rc;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_ser_arg<'i>(
    value: &ast::Value<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(String, RcSecurityTetraplets)> {
    use ast::Value::*;

    match value {
        InitPeerId => prepare_const(ctx.init_peer_id.as_str(), ctx),
        LastError(error_accessor) => prepare_last_error(error_accessor, ctx),
        Literal(value) => prepare_const(value.to_string(), ctx),
        Boolean(value) => prepare_const(*value, ctx),
        Number(value) => prepare_const(value, ctx),
        EmptyArray => prepare_const("", ctx),
        Variable(variable) => {
            resolve_ast_variable_wl(variable, ctx).map(|(value, tetraplets)| (value.serialize(), tetraplets))
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn prepare_const(
    arg: impl Into<String>,
    ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<(String, RcSecurityTetraplets)> {
    let serialized_arg = arg.into();
    let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.as_ref());
    let tetraplet = Rc::new(tetraplet);

    Ok((serialized_arg, vec![tetraplet]))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn prepare_last_error<'i>(
    error_accessor: &Option<AIRLambdaAST<'i>>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(String, RcSecurityTetraplets)> {
    use crate::LastError;

    let LastError { error, tetraplet } = ctx.last_error();

    let value = match error_accessor {
        Some(lambda) => {
            let resolved_lambda = crate::execution_step::lambda_applier::resolve_lambda(lambda, ctx)?;
            error.apply_lambda(&resolved_lambda)?
        }
        None => error.as_ref(),
    };

    let tetraplets = match tetraplet {
        Some(tetraplet) => vec![tetraplet.clone()],
        None => {
            let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.as_ref());
            let tetraplet = Rc::new(tetraplet);
            vec![tetraplet]
        }
    };

    Ok((value.serialize(), tetraplets))
}

pub(crate) fn resolve_variable<'ctx, 'i>(
    variable: Variable<'_>,
    ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<Box<dyn AIRValueAlgebra<Error = boxed_value::ValueLambdaError> + 'ctx>> {
    use super::StreamValueAlgebraIngredients;

    match variable {
        Variable::Scalar { name, .. } => Ok(ctx.scalars.get(name)?.as_air_value()),
        Variable::Stream {
            name,
            generation,
            position,
        } => {
            match ctx.streams.get(name, position) {
                Some(stream) => {
                    let ingredients = StreamValueAlgebraIngredients::new(stream, generation);
                    Ok(Box::new(ingredients))
                }
                // return an empty stream for not found stream
                // here it ignores the join behaviour
                None => {
                    let empty_stream = Stream::from_generations_count(0);
                    Ok(Box::new(empty_stream))
                }
            }
        }
    }
}

pub(crate) fn resolve_ast_scalar_wl<'ctx, 'i>(
    ast_scalar: &ast::ScalarWithLambda<'_>,
    exec_ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<(&'ctx RcBoxedValue, RcSecurityTetraplets)> {
    // TODO: wrap lambda path with Rc to make this clone cheaper
    let variable = ast::VariableWithLambda::Scalar(ast_scalar.clone());
    resolve_ast_variable_wl(&variable, exec_ctx)
}

pub(crate) fn resolve_ast_variable_wl<'ctx, 'i>(ast_variable: &ast::VariableWithLambda<'_>, exec_ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<(&'ctx RcBoxedValue, RcSecurityTetraplets)> {
    let variable: Variable<'_> = ast_variable.into();
    match ast_variable.lambda() {
        Some(lambda) => apply_lambda(variable, lambda, exec_ctx).map(|vt| (vt.value, vec![vt.tetraplet.clone()])),
        None => {
            let value = resolve_variable(variable, exec_ctx)?;
            let tetraplets = value.as_tetraplets();

            Ok((value.as_value(), tetraplets))
        }
    }
}

pub(crate) fn apply_lambda<'ctx, 'i>(
    variable: Variable<'_>,
    lambda: &AIRLambdaAST<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<ValueWithTetraplet<'ctx, 'ctx>> {
    let resolved_lambda = crate::execution_step::lambda_applier::resolve_lambda(lambda, exec_ctx)?;
    let resolved = resolve_variable(variable, exec_ctx)?;
    resolved.apply_lambda_with_tetraplets(&resolved_lambda).map_err(Into::into)
}
