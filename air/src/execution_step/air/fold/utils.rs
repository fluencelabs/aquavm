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
use crate::execution_step::CatchableError;
use crate::AIRLambdaAST;

use air_lambda_ast::AIRLambda;
use air_parser::ast;
use air_values::boxed_value::RcSecurityTetraplet;
use air_values::boxed_value::{AIRValueAlgebra, BoxedValue, ValueAggregate};
use air_values::fold_iterable_state::IterableValue;
use air_values::scalar::ScalarRef;
use air_values::stream::Generation;
use air_values::stream::Stream;

pub(crate) enum FoldIterableScalar {
    Empty,
    Scalar(IterableValue),
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
pub(crate) fn construct_stream_iterable_values(
    stream: &Stream,
    start: Generation,
    end: Generation,
) -> Vec<IterableValue> {
    let stream_iter = match stream.slice_iter(start, end) {
        Some(stream_iter) => stream_iter,
        None => return vec![],
    };

    stream_iter
        .filter(|iterable| !iterable.is_empty())
        .map(|iterable| {
            let call_results = iterable.to_vec();
            let foldable = IterableVecResolvedCall::init(call_results);
            let foldable: IterableValue = Box::new(foldable);
            foldable
        })
        .collect::<Vec<_>>()
}

fn create_scalar_iterable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
) -> ExecutionResult<FoldIterableScalar> {
    match exec_ctx.scalars.get(variable_name)? {
        ScalarRef::Value(call_result) => from_call_result(call_result.clone(), variable_name),
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let call_result = iterable_value.into_value_aggregate();
            from_call_result(call_result, variable_name)
        }
    }
}

/// Constructs iterable value from resolved call result.
fn from_call_result(call_result: ValueAggregate, variable_name: &str) -> ExecutionResult<FoldIterableScalar> {
    let iter = call_result
        .value
        .as_iter()
        .ok_or_else(|| CatchableError::IncompatibleJValueType {
            variable_name: variable_name.to_string(),
            actual_value: call_result.value.to_string(),
            expected_value_type: "array",
        })?;

    if iter.is_empty() {
        // skip fold if array is empty
        return Ok(FoldIterableScalar::Empty);
    }

    let len = iter.len();

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);
    let iterable = FoldIterableScalar::Scalar(foldable);

    Ok(iterable)
}

fn create_scalar_lambda_iterable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    scalar_name: &str,
    lambda: &AIRLambdaAST<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let resolved_lambda = crate::execution_step::lambda_applier::resolve_lambda(lambda, exec_ctx)?;

    match exec_ctx.scalars.get(scalar_name)? {
        ScalarRef::Value(variable) => {
            let value = variable.value.apply_lambda(&resolved_lambda)?;
            let tetraplet = variable.tetraplet.clone();
            from_value(value, tetraplet, &resolved_lambda)
        }
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.apply_lambda(&resolved_lambda, exec_ctx)?;
            let tetraplet = iterable_value.tetraplet.clone();

            from_value(jvalue, tetraplet, &resolved_lambda)
        }
    }
}

/// Construct IterableValue from the result and given triplet.
fn from_value(
    value: &dyn BoxedValue,
    mut tetraplet: RcSecurityTetraplet,
    lambda: &AIRLambda<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let formatted_lambda = air_lambda_ast::format_lambda(lambda);
    tetraplet.add_lambda(&formatted_lambda);

    let iterable = value.as_iter().ok_or_else(|| {
        CatchableError::FoldIteratesOverNonArray {
            value: value.to_string(),
            lambda: formatted_lambda,
        }
        .into()
    })?;

    if iterable.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    let iterable = iterable.to_vec();
    let foldable = IterableLambdaResult::init(iterable, tetraplet);
    let iterable = FoldIterableScalar::Scalar(Box::new(foldable));
    Ok(iterable)
}
