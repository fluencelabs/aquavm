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

use super::*;
use crate::exec_err;
use crate::execution_step::RSecurityTetraplet;
use crate::JValue;
use crate::LambdaAST;

use air_parser::ast;

use std::ops::Deref;

// TODO: refactor this file after switching to boxed value

pub(crate) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

pub(crate) enum FoldIterableScalar {
    Empty,
    Scalar(IterableValue),
}

pub(crate) enum FoldIterableStream {
    Empty,
    Stream(Vec<IterableValue>),
}

/// Constructs iterable value for given scalar iterable.
pub(crate) fn construct_scalar_iterable_value<'ctx>(
    iterable: &ast::ScalarWithLambda<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    match &iterable.lambda {
        None => create_scalar_iterable(exec_ctx, iterable.name),
        Some(lambda) => create_scalar_lambda_iterable(exec_ctx, iterable.name, lambda),
    }
}

/// Constructs iterable value for given stream iterable.
pub(crate) fn construct_stream_iterable_value<'ctx>(
    stream_name: &'ctx str,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableStream> {
    match exec_ctx.streams.get(stream_name) {
        Some(stream) => {
            let stream = stream.borrow();
            if stream.is_empty() {
                return Ok(FoldIterableStream::Empty);
            }

            let mut iterables = Vec::with_capacity(stream.generations_count());

            for iterable in stream.slice_iter(Generation::Last).unwrap() {
                if iterable.is_empty() {
                    continue;
                }

                let call_results = iterable.to_vec();
                let foldable = IterableVecResolvedCall::init(call_results);
                let foldable: IterableValue = Box::new(foldable);
                iterables.push(foldable);
            }

            Ok(FoldIterableStream::Stream(iterables))
        }
        // it's possible to met streams without variables at the moment in fold,
        // they should be treated as empty.
        None => Ok(FoldIterableStream::Empty),
    }
}

fn create_scalar_iterable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
) -> ExecutionResult<FoldIterableScalar> {
    match exec_ctx.scalars.get(variable_name)? {
        ScalarRef::Value(call_result) => from_call_result(call_result.clone()),
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let call_result = iterable_value.into_resolved_result();
            from_call_result(call_result)
        }
    }
}

/// Constructs iterable value from resolved call result.
fn from_call_result(call_result: ValueAggregate) -> ExecutionResult<FoldIterableScalar> {
    use ExecutionError::IncompatibleJValueType;

    let len = match &call_result.result.deref() {
        JValue::Array(array) => {
            if array.is_empty() {
                // skip fold if array is empty
                return Ok(FoldIterableScalar::Empty);
            }
            array.len()
        }
        v => return exec_err!(IncompatibleJValueType((*v).clone(), "array")),
    };

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);
    let iterable = FoldIterableScalar::Scalar(foldable);

    Ok(iterable)
}

fn create_scalar_lambda_iterable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    scalar_name: &str,
    lambda: &LambdaAST<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    use crate::execution_step::lambda_applier::select;

    match exec_ctx.scalars.get(scalar_name)? {
        ScalarRef::Value(variable) => {
            let jvalues = select(&variable.result, lambda.iter())?;
            from_jvalue(jvalues, variable.tetraplet.clone(), lambda)
        }
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.apply_lambda(lambda)?;
            let tetraplet = as_tetraplet(&iterable_value);

            from_jvalue(jvalue, tetraplet, lambda)
        }
    }
}

/// Construct IterableValue from the result and given triplet.
fn from_jvalue(
    jvalue: &JValue,
    tetraplet: RSecurityTetraplet,
    lambda: &LambdaAST<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let formatted_lambda_ast = air_lambda_ast::format_ast(lambda);
    tetraplet.borrow_mut().add_lambda(&formatted_lambda_ast);

    let iterable = match jvalue {
        JValue::Array(array) => array,
        _ => {
            return exec_err!(ExecutionError::FoldIteratesOverNonArray(
                jvalue.clone(),
                formatted_lambda_ast
            ))
        }
    };

    if iterable.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    let iterable = iterable.to_vec();
    let foldable = IterableLambdaResult::init(iterable, tetraplet);
    let iterable = FoldIterableScalar::Scalar(Box::new(foldable));
    Ok(iterable)
}

fn as_tetraplet(iterable: &IterableItem<'_>) -> RSecurityTetraplet {
    use IterableItem::*;

    let tetraplet = match iterable {
        RefRef((_, tetraplet, _)) => tetraplet,
        RefValue((_, tetraplet, _)) => tetraplet,
        RcValue((_, tetraplet, _)) => tetraplet,
    };

    (*tetraplet).clone()
}
