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
use crate::execution_step::value_types::populate_tetraplet_with_lambda;
use crate::execution_step::value_types::IterableValue;
use crate::execution_step::CatchableError;
use crate::execution_step::PEEK_ALLOWED_ON_NON_EMPTY;
use crate::JValue;
use crate::LambdaAST;
use crate::SecurityTetraplet;

use air_interpreter_data::Provenance;
use air_parser::ast;

use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

// TODO: refactor this file after switching to boxed value

pub(crate) enum FoldIterableScalar {
    Empty,
    ScalarBased(IterableValue),
}

/// Creates iterable value for given scalar iterable.
pub(crate) fn create_scalar_iterable(
    exec_ctx: &ExecutionCtx<'_>,
    variable_name: &str,
) -> ExecutionResult<FoldIterableScalar> {
    match exec_ctx.scalars.get_value(variable_name)? {
        ScalarRef::Value(call_result) => from_value(call_result.clone(), variable_name),
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY);
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
    use crate::execution_step::lambda_applier::select_by_lambda_from_scalar;
    let scalar_name = scalar_iterable.name;
    let lambda = &scalar_iterable.lambda;

    match exec_ctx.scalars.get_value(scalar_name)? {
        ScalarRef::Value(variable) => {
            let jvalues = select_by_lambda_from_scalar(variable.get_result(), lambda, exec_ctx)?;
            let tetraplet = variable.get_tetraplet().deref().clone();
            from_jvalue(jvalues, tetraplet, variable.get_provenance(), lambda)
        }
        ScalarRef::IterableValue(fold_state) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.apply_lambda(lambda, exec_ctx)?;
            let tetraplet = to_tetraplet(&iterable_value);
            let provenance = to_provenance(&iterable_value);

            from_jvalue(jvalue, tetraplet, provenance, lambda)
        }
    }
}

/// Creates iterable value for given canon stream.
pub(crate) fn create_canon_stream_iterable_value<'ctx>(
    ast_canon_stream: &ast::CanonStream<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    let canon_stream = exec_ctx.scalars.get_canon_stream(ast_canon_stream.name)?;
    if canon_stream.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    // TODO: this one is a relatively long operation and will be refactored in Boxed Value
    let iterable_ingredients = CanonStreamIterableIngredients::init((**canon_stream).clone());
    let iterable = Box::new(iterable_ingredients);
    Ok(FoldIterableScalar::ScalarBased(iterable))
}

/// Creates iterable value for a canon stream map.
pub(crate) fn create_canon_stream_map_iterable_value(
    ast_canon_stream_map: &ast::CanonStreamMap<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let canon_stream_map = exec_ctx.scalars.get_canon_map(ast_canon_stream_map.name)?;

    if canon_stream_map.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    // TODO: this one is a relatively long operation and will be refactored in Boxed Value
    // Can not create iterable from existing CanonStreamMap b/c CSM contains a map with
    // a limited lifetime but the boxed value needs static lifetime.
    let values = canon_stream_map.iter().cloned().collect::<Vec<_>>();
    let iterable_ingredients = CanonStreamMapIterableIngredients::init(values);
    let iterable = Box::new(iterable_ingredients);
    Ok(FoldIterableScalar::ScalarBased(iterable))
}

/// Creates iterable value for a canon stream map with a lens applied.
pub(crate) fn create_canon_stream_map_wl_iterable_value(
    ast_canon_stream_map: &ast::CanonStreamMapWithLambda<'_>,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let canon_stream_map = exec_ctx.scalars.get_canon_map(ast_canon_stream_map.name)?;
    // Source canon map provenance is used here to mimic the semantics used for scalar.
    let provenance = Provenance::canon(canon_stream_map.cid.clone());
    if canon_stream_map.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    let canon_stream_map = &canon_stream_map.canon_stream_map;
    let tetraplet = canon_stream_map.tetraplet().deref().clone();
    let lambda = &ast_canon_stream_map.lambda;
    // Source canon map tetraplet is used here similar with a scalar with lens processing path.
    let jvalues = JValuable::apply_lambda(&canon_stream_map, lambda, exec_ctx)?;

    from_jvalue(jvalues, tetraplet, provenance, lambda)
}

/// Constructs iterable value from resolved call result.
fn from_value(call_result: ValueAggregate, variable_name: &str) -> ExecutionResult<FoldIterableScalar> {
    let len = match call_result.get_result().deref() {
        JValue::Array(array) => {
            if array.is_empty() {
                // skip fold if array is empty
                return Ok(FoldIterableScalar::Empty);
            }
            array.len()
        }
        v => {
            return Err(CatchableError::FoldIteratesOverNonArray((*v).clone(), variable_name.to_string()).into());
        }
    };

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);
    let iterable = FoldIterableScalar::ScalarBased(foldable);

    Ok(iterable)
}

/// Construct IterableValue from the result and given triplet.
fn from_jvalue(
    jvalue: Cow<'_, JValue>,
    tetraplet: SecurityTetraplet,
    provenance: Provenance,
    lambda: &LambdaAST<'_>,
) -> ExecutionResult<FoldIterableScalar> {
    let tetraplet = populate_tetraplet_with_lambda(tetraplet, lambda);
    let tetraplet = Rc::new(tetraplet);

    let iterable = match jvalue.as_ref() {
        JValue::Array(array) => array,
        _ => {
            return Err(CatchableError::FoldIteratesOverNonArray(jvalue.into_owned(), lambda.to_string()).into());
        }
    };

    if iterable.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    let iterable = iterable.to_vec();
    let foldable = IterableLambdaResult::init(iterable, tetraplet, provenance);
    let iterable = FoldIterableScalar::ScalarBased(Box::new(foldable));
    Ok(iterable)
}

fn to_tetraplet(iterable: &IterableItem<'_>) -> SecurityTetraplet {
    use IterableItem::*;

    let tetraplet = match iterable {
        RefValue((_, tetraplet, _, _)) => tetraplet,
        RcValue((_, tetraplet, _, _)) => tetraplet,
    };

    (*tetraplet).deref().clone()
}

fn to_provenance(iterable: &IterableItem<'_>) -> Provenance {
    use IterableItem::*;

    let provenance = match iterable {
        RefValue((_, _, _, provenance)) => provenance,
        RcValue((_, _, _, provenance)) => provenance,
    };

    provenance.clone()
}
