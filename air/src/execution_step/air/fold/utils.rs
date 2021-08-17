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

use air_parser::ast;
use jsonpath_lib::select;

use std::ops::Deref;
use std::rc::Rc;

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
    ast_iterable: &ast::IterableScalarValue<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterableScalar> {
    match ast_iterable {
        ast::IterableScalarValue::ScalarVariable(scalar_name) => create_scalar_iterable(exec_ctx, scalar_name),
        ast::IterableScalarValue::JsonPath {
            scalar_name,
            path,
            should_flatten,
        } => create_scalar_json_path_iterable(exec_ctx, scalar_name, path, *should_flatten),
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
    match exec_ctx.scalars.get(variable_name) {
        Some(Scalar::JValueRef(call_result)) => from_call_result(call_result.clone()),
        Some(Scalar::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let call_result = iterable_value.into_resolved_result();
            from_call_result(call_result)
        }
        _ => return exec_err!(ExecutionError::VariableNotFound(variable_name.to_string())),
    }
}

/// Constructs iterable value from resolved call result.
fn from_call_result(call_result: ResolvedCallResult) -> ExecutionResult<FoldIterableScalar> {
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

fn create_scalar_json_path_iterable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    scalar_name: &str,
    json_path: &str,
    should_flatten: bool,
) -> ExecutionResult<FoldIterableScalar> {
    match exec_ctx.scalars.get(scalar_name) {
        Some(Scalar::JValueRef(variable)) => {
            let jvalues = apply_json_path(&variable.result, json_path)?;
            from_jvalues(jvalues, variable.tetraplet.clone(), json_path, should_flatten)
        }
        Some(Scalar::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalues = iterable_value.apply_json_path(json_path)?;
            let tetraplet = as_tetraplet(&iterable_value);

            from_jvalues(jvalues, tetraplet, json_path, should_flatten)
        }
        _ => return exec_err!(ExecutionError::VariableNotFound(scalar_name.to_string())),
    }
}

fn apply_json_path<'jvalue, 'str>(
    jvalue: &'jvalue JValue,
    json_path: &'str str,
) -> ExecutionResult<Vec<&'jvalue JValue>> {
    use ExecutionError::JValueJsonPathError;

    select(jvalue, json_path).map_err(|e| Rc::new(JValueJsonPathError(jvalue.clone(), json_path.to_string(), e)))
}

/// Applies json_path to provided jvalues and construct IterableValue from the result and given triplet.
fn from_jvalues(
    jvalues: Vec<&JValue>,
    tetraplet: RSecurityTetraplet,
    json_path: &str,
    should_flatten: bool,
) -> ExecutionResult<FoldIterableScalar> {
    let jvalues = construct_iterable_jvalues(jvalues, should_flatten)?;

    if jvalues.is_empty() {
        return Ok(FoldIterableScalar::Empty);
    }

    tetraplet.borrow_mut().add_json_path(json_path);

    let foldable = IterableJsonPathResult::init(jvalues, tetraplet);
    let iterable = FoldIterableScalar::Scalar(Box::new(foldable));
    Ok(iterable)
}

fn construct_iterable_jvalues(jvalues: Vec<&JValue>, should_flatten: bool) -> ExecutionResult<Vec<JValue>> {
    if !should_flatten {
        let jvalues = jvalues.into_iter().cloned().collect();
        return Ok(jvalues);
    }

    if jvalues.len() != 1 {
        let jvalues = jvalues.into_iter().cloned().collect();
        let jvalue = JValue::Array(jvalues);
        return exec_err!(ExecutionError::FlatteningError(jvalue));
    }

    match jvalues[0] {
        JValue::Array(values) => Ok(values.clone()),
        _ => {
            let jvalues = jvalues.into_iter().cloned().collect();
            let jvalue = JValue::Array(jvalues);
            exec_err!(ExecutionError::FlatteningError(jvalue))
        }
    }
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
