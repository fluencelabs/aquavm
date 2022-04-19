/*
 * Copyright 2021 Fluence Labs Limited
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
use air_lambda_parser::LambdaAST;
use air_parser::ast;

pub(super) fn apply_to_arg(
    argument: &ast::ApArgument<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
    should_touch_trace: bool,
) -> ExecutionResult<ValueAggregate> {
    use ast::ApArgument::*;

    let result = match argument {
        InitPeerId => apply_const(exec_ctx.run_parameters.init_peer_id.as_str(), exec_ctx, trace_ctx),
        LastError(error_accessor) => apply_last_error(error_accessor, exec_ctx, trace_ctx)?,
        Literal(value) => apply_const(*value, exec_ctx, trace_ctx),
        Timestamp => apply_const(exec_ctx.run_parameters.timestamp, exec_ctx, trace_ctx),
        Number(value) => apply_const(value, exec_ctx, trace_ctx),
        Boolean(value) => apply_const(*value, exec_ctx, trace_ctx),
        EmptyArray => apply_const(serde_json::json!([]), exec_ctx, trace_ctx),
        Scalar(scalar) => apply_scalar(scalar, exec_ctx, trace_ctx, should_touch_trace)?,
    };

    Ok(result)
}

fn apply_const(value: impl Into<JValue>, exec_ctx: &ExecutionCtx<'_>, trace_ctx: &TraceHandler) -> ValueAggregate {
    let value = Rc::new(value.into());
    let tetraplet = SecurityTetraplet::literal_tetraplet(exec_ctx.run_parameters.init_peer_id.as_ref());
    let tetraplet = Rc::new(tetraplet);

    ValueAggregate::new(value, tetraplet, trace_ctx.trace_pos())
}

fn apply_last_error<'i>(
    error_accessor: &Option<LambdaAST<'i>>,
    exec_ctx: &ExecutionCtx<'i>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ValueAggregate> {
    let (value, mut tetraplets) = crate::execution_step::resolver::prepare_last_error(error_accessor, exec_ctx)?;
    let value = Rc::new(value);
    // removing is safe because prepare_last_error always returns a vec with one element.
    let tetraplet = tetraplets.remove(0);

    let result = ValueAggregate::new(value, tetraplet, trace_ctx.trace_pos());
    Ok(result)
}

fn apply_scalar(
    scalar: &ast::ScalarWithLambda<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
    should_touch_trace: bool,
) -> ExecutionResult<ValueAggregate> {
    // TODO: refactor this code after boxed value
    match &scalar.lambda {
        Some(lambda) => apply_scalar_wl_impl(scalar.name, scalar.position, lambda, exec_ctx, trace_ctx),
        None => apply_scalar_impl(scalar.name, exec_ctx, trace_ctx, should_touch_trace),
    }
}

fn apply_scalar_impl(
    scalar_name: &str,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
    should_touch_trace: bool,
) -> ExecutionResult<ValueAggregate> {
    use crate::execution_step::ScalarRef;

    let scalar = exec_ctx.scalars.get(scalar_name)?;

    let mut result = match scalar {
        ScalarRef::Value(result) => result.clone(),
        ScalarRef::IterableValue(iterator) => {
            let result = iterator.iterable.peek().expect(
                "peek always return elements inside fold,\
            this guaranteed by implementation of next and avoiding empty folds",
            );
            result.into_resolved_result()
        }
    };

    if should_touch_trace {
        result.trace_pos = trace_ctx.trace_pos();
    }

    Ok(result)
}

fn apply_scalar_wl_impl(
    scalar_name: &str,
    position: usize,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ValueAggregate> {
    let variable = Variable::scalar(scalar_name, position);
    let (jvalue, tetraplet) = apply_lambda(variable, lambda, exec_ctx)?;
    let tetraplet = Rc::new(tetraplet);
    let result = ValueAggregate::new(Rc::new(jvalue), tetraplet, trace_ctx.trace_pos());

    Ok(result)
}
