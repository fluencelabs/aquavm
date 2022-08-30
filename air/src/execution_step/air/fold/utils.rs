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
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_parser::ast;

use std::ops::Deref;
use std::rc::Rc;

// TODO: refactor this file after switching to boxed value

pub(crate) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

pub(crate) enum FoldIterableScalar {
    Empty,
    ScalarBased(IterableValue),
}

/// Creates iterable value for given scalar iterable.
pub(crate) fn create_scalar_iterable<'ctx>(
    scalar_iterable: &ast::Scalar<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    let scalar_name = scalar_iterable.name();
    match exec_ctx.scalars.get_value(scalar_name)? {
        ScalarRef::Value(call_result) => from_value(call_result.clone(), variable_name),
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let call_result = iterable_value.into_resolved_result();
            from_value(call_result, variable_name)
        }
    }
}

/// Creates iterable value for given scalar with lambda iterable.
pub(crate) fn create_scalar_wl_iterable<'ctx>(
    scalar_iterable: &ast::ScalarWithLambda<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    use crate::execution_step::lambda_applier::select_from_scalar;
    let scalar_name = scalar_iterable.name;
    let lambda = &scalar_iterable.lambda;

    match exec_ctx.scalars.get_value(scalar_name)? {
        ScalarRef::Value(variable) => {
            let jvalues = select_from_scalar(&variable.result, lambda.iter(), exec_ctx)?;
            let tetraplet = variable.tetraplet.deref().clone();
            from_jvalue(jvalues, tetraplet, lambda)
        }
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.apply_lambda(lambda, exec_ctx)?;
            let tetraplet = to_tetraplet(&iterable_value);

            from_jvalue(jvalue, tetraplet, lambda)
        }
    }
}

/// Creates iterable value for given canon stream.
pub(crate) fn create_canon_stream_iterable_value<'ctx>(
    ast_canon_stream: &ast::CanonStream<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    let canon_stream = exec_ctx.scalars.get_canon_stream(ast_canon_stream.name)?;
    // TODO: this one is a relatively long operation and will be refactored in Boxed Value
    let iterable_ingredients = CanonStreamIterableIngredients::init(canon_stream.clone());
    let iterable = Box::new(iterable_ingredients);
    Ok(FoldIterableScalar::ScalarBased(iterable))
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

/// Constructs iterable value from resolved call result.
fn from_value(call_result: ValueAggregate, variable_name: &str) -> ExecutionResult<FoldIterableScalar> {
    let len = match &call_result.result.deref() {
        JValue::Array(array) => {
            if array.is_empty() {
                // skip fold if array is empty
                return Ok(FoldIterableScalar::Empty);
            }
            array.len()
        }
        v => {
            return Err(CatchableError::IncompatibleJValueType {
                variable_name: variable_name.to_string(),
                actual_value: (*v).clone(),
                expected_value_type: "array",
            }
            .into());
        }
    };

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);
    let iterable = FoldIterableScalar::ScalarBased(foldable);

    Ok(iterable)
}

/// Construct IterableValue from the result and given triplet.
fn from_jvalue(
    jvalue: &JValue,
    mut tetraplet: SecurityTetraplet,
    lambda: &LambdaAST<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let formatted_lambda_ast = air_lambda_ast::format_ast(lambda);
    tetraplet.add_lambda(&formatted_lambda_ast);
    let tetraplet = Rc::new(tetraplet);

    let iterable = match jvalue {
        JValue::Array(array) => array,
        _ => {
            return Err(CatchableError::FoldIteratesOverNonArray(jvalue.clone(), formatted_lambda_ast).into());
        }
    };

    if iterable.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    let iterable = iterable.to_vec();
    let foldable = IterableLambdaResult::init(iterable, tetraplet);
    let iterable = FoldIterableScalar::ScalarBased(Box::new(foldable));
    Ok(iterable)
}

fn to_tetraplet(iterable: &IterableItem<'_>) -> SecurityTetraplet {
    use IterableItem::*;

    let tetraplet = match iterable {
        RefRef((_, tetraplet, _)) => tetraplet,
        RefValue((_, tetraplet, _)) => tetraplet,
        RcValue((_, tetraplet, _)) => tetraplet,
    };

    (*tetraplet).deref().clone()
}
