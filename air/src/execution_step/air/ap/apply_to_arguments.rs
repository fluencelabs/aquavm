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
use crate::execution_step::PEEK_ALLOWED_ON_NON_EMPTY;

use air_lambda_parser::LambdaAST;
use air_parser::ast;

pub(super) fn apply_to_arg(
    argument: &ast::ApArgument<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ValueAggregate> {
    use ast::ApArgument::*;

    let result = match argument {
        InitPeerId => apply_const(exec_ctx.run_parameters.init_peer_id.as_str(), exec_ctx, trace_ctx),
        LastError(error_accessor) => apply_last_error(error_accessor, exec_ctx, trace_ctx)?,
        Literal(value) => apply_const(*value, exec_ctx, trace_ctx),
        Timestamp => apply_const(exec_ctx.run_parameters.timestamp, exec_ctx, trace_ctx),
        TTL => apply_const(exec_ctx.run_parameters.ttl, exec_ctx, trace_ctx),
        Number(value) => apply_const(value, exec_ctx, trace_ctx),
        Boolean(value) => apply_const(*value, exec_ctx, trace_ctx),
        EmptyArray => apply_const(serde_json::json!([]), exec_ctx, trace_ctx),
        Scalar(scalar) => apply_scalar(scalar, exec_ctx, trace_ctx)?,
        CanonStream(canon_stream) => apply_canon_stream(canon_stream, exec_ctx)?,
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
) -> ExecutionResult<ValueAggregate> {
    // TODO: refactor this code after boxed value
    match &scalar.lambda {
        Some(lambda) => apply_scalar_wl_impl(scalar.name, lambda, exec_ctx, trace_ctx),
        None => apply_scalar_impl(scalar.name, exec_ctx, trace_ctx),
    }
}

fn apply_scalar_impl(
    scalar_name: &str,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ValueAggregate> {
    use crate::execution_step::ScalarRef;

    let scalar = exec_ctx.scalars.get_value(scalar_name)?;

    let mut result = match scalar {
        ScalarRef::Value(result) => result.clone(),
        ScalarRef::IterableValue(iterator) => {
            let result = iterator.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY);
            result.into_resolved_result()
        }
    };

    result.trace_pos = trace_ctx.trace_pos();

    Ok(result)
}

fn apply_scalar_wl_impl(
    scalar_name: &str,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ValueAggregate> {
    let variable = Variable::scalar(scalar_name);
    let (jvalue, tetraplet) = apply_lambda(variable, lambda, exec_ctx)?;
    let tetraplet = Rc::new(tetraplet);
    let result = ValueAggregate::new(Rc::new(jvalue), tetraplet, trace_ctx.trace_pos());

    Ok(result)
}

fn apply_canon_stream(
    canon_stream: &ast::CanonStreamWithLambda<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<ValueAggregate> {
    match &canon_stream.lambda {
        Some(lambda) => apply_canon_stream_with_lambda(canon_stream.name, lambda, exec_ctx),
        None => apply_canon_stream_without_lambda(canon_stream.name, exec_ctx),
    }
}

fn apply_canon_stream_with_lambda(
    stream_name: &str,
    lambda: &LambdaAST<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<ValueAggregate> {
    use crate::execution_step::boxed_value::JValuable;

    let canon_stream = exec_ctx.scalars.get_canon_stream(stream_name)?;
    let (result, tetraplet) = JValuable::apply_lambda_with_tetraplets(&canon_stream, lambda, exec_ctx)?;
    // TODO: refactor this code after boxed value
    let value = ValueAggregate::new(
        Rc::new(result.into_owned()),
        Rc::new(tetraplet),
        canon_stream.position(),
    );
    Ok(value)
}

fn apply_canon_stream_without_lambda(
    stream_name: &str,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<ValueAggregate> {
    use crate::execution_step::boxed_value::JValuable;

    let canon_stream = exec_ctx.scalars.get_canon_stream(stream_name)?;
    // TODO: refactor this code after boxed value
    let value = JValuable::as_jvalue(&canon_stream).into_owned();

    let tetraplet = canon_stream.tetraplet().clone();
    let value = ValueAggregate::new(Rc::new(value), tetraplet, canon_stream.position());
    Ok(value)
}
