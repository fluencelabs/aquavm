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
use crate::execution_step::utils::get_variable_name;
use crate::JValue;
use crate::ResolvedTriplet;
use crate::SecurityTetraplet;

use air_parser::ast;
use jsonpath_lib::select;

use std::ops::Deref;
use std::rc::Rc;

pub(super) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItem<'ctx>>>;

pub(crate) enum FoldIterable {
    Empty,
    Scalar(IterableValue),
    Stream(Vec<IterableValue>),
}

/// Constructs iterable value for given instruction value,
/// return Some if iterable isn't empty and None otherwise.
pub(super) fn construct_iterable_value<'ctx>(
    ast_iterable: &ast::IterableValue<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> ExecutionResult<FoldIterable> {
    match ast_iterable {
        ast::IterableValue::Variable(variable) => {
            let name = get_variable_name(variable);
            handle_instruction_variable(exec_ctx, name)
        }
        ast::IterableValue::JsonPath {
            scalar_name,
            path,
            should_flatten,
        } => handle_instruction_json_path(exec_ctx, scalar_name, path, *should_flatten),
    }
}

fn handle_instruction_variable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
) -> ExecutionResult<FoldIterable> {
    match exec_ctx.data_cache.get(variable_name) {
        Some(AValue::JValueRef(call_result)) => from_call_result(call_result.clone()),
        Some(AValue::StreamRef(stream)) => {
            let stream = stream.borrow();
            if stream.is_empty() {
                return Ok(FoldIterable::Empty);
            }

            let mut iterables = Vec::with_capacity(stream.0.len());

            for iterable in stream.0.iter() {
                let call_results = iterable.to_vec();
                let foldable = IterableVecResolvedCall::init(call_results);
                let foldable: IterableValue = Box::new(foldable);
                iterables.push(foldable);
            }

            Ok(FoldIterable::Stream(iterables))
        }
        Some(AValue::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.as_jvalue();
            let result = Rc::new(jvalue.into_owned());
            let triplet = as_triplet(&iterable_value);

            // TODO: it's safe to use 0 here, because trace_pos isn't needed for scalars,
            // but it's needed to be refactored in future
            let call_result = ResolvedCallResult {
                result,
                triplet,
                trace_pos: 0,
            };
            from_call_result(call_result)
        }
        _ => return exec_err!(ExecutionError::VariableNotFound(variable_name.to_string())),
    }
}

/// Constructs iterable value from resolved call result.
fn from_call_result(call_result: ResolvedCallResult) -> ExecutionResult<FoldIterable> {
    use ExecutionError::IncompatibleJValueType;

    let len = match &call_result.result.deref() {
        JValue::Array(array) => {
            if array.is_empty() {
                // skip fold if array is empty
                return Ok(FoldIterable::Empty);
            }
            array.len()
        }
        v => return exec_err!(IncompatibleJValueType((*v).clone(), "array")),
    };

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);
    let iterable = FoldIterable::Scalar(foldable);

    Ok(iterable)
}

fn handle_instruction_json_path<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
    json_path: &str,
    should_flatten: bool,
) -> ExecutionResult<FoldIterable> {
    match exec_ctx.data_cache.get(variable_name) {
        Some(AValue::JValueRef(variable)) => {
            let jvalues = apply_json_path(&variable.result, json_path)?;
            from_jvalues(jvalues, variable.triplet.clone(), json_path, should_flatten)
        }
        // TODO: refactor this after switching to bexed value, stream and scalar should live in a separate fields
        Some(AValue::StreamRef(stream)) => exec_err!(ExecutionError::InternalError(format!(
            "scalar name points to stream: {:?}",
            stream
        ))),
        Some(AValue::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalues = iterable_value.apply_json_path(json_path)?;
            let triplet = as_triplet(&iterable_value);

            from_jvalues(jvalues, triplet, json_path, should_flatten)
        }
        _ => return exec_err!(ExecutionError::VariableNotFound(variable_name.to_string())),
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
    triplet: Rc<ResolvedTriplet>,
    json_path: &str,
    should_flatten: bool,
) -> ExecutionResult<FoldIterable> {
    let jvalues = construct_iterable_jvalues(jvalues, should_flatten)?;

    if jvalues.is_empty() {
        return Ok(FoldIterable::Empty);
    }

    let tetraplet = SecurityTetraplet {
        triplet,
        json_path: json_path.to_string(),
    };

    let foldable = IterableJsonPathResult::init(jvalues, tetraplet);
    let iterable = FoldIterable::Scalar(Box::new(foldable));
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

fn as_triplet(iterable: &IterableItem<'_>) -> Rc<ResolvedTriplet> {
    use IterableItem::*;

    let tetraplet = match iterable {
        RefRef((_, tetraplet, _)) => tetraplet,
        RefValue((_, tetraplet, _)) => tetraplet,
        RcValue((_, tetraplet, _)) => tetraplet,
    };

    // clone is cheap here, because triplet is under Rc
    Rc::clone(&tetraplet.triplet)
}
