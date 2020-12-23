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

use super::iterable::*;
use super::Iterable;
use super::IterableItemType;
use crate::air::ExecutionCtx;
use crate::air::JValuable;
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::ResolvedCallResult;
use crate::ResolvedTriplet;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::InstructionValue;
use jsonpath_lib::select;
use jsonpath_lib::select_with_iter;

use std::ops::Deref;
use std::rc::Rc;

pub(super) type IterableValue = Box<dyn for<'ctx> Iterable<'ctx, Item = IterableItemType<'ctx>>>;

/// Constructs iterable value for given instruction value,
/// return Some if iterable isn't empty and None otherwise.
pub(super) fn construct_iterable_value<'ctx>(
    value: &InstructionValue<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
) -> Result<Option<IterableValue>> {
    match value {
        InstructionValue::Variable(name) => handle_instruction_variable(exec_ctx, name),
        InstructionValue::JsonPath { variable, path } => handle_instruction_json_path(exec_ctx, variable, path),
        _ => unreachable!("it will be statically checked that other types of iterable value aren't possible here"),
    }
}

fn handle_instruction_variable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
) -> Result<Option<IterableValue>> {
    let iterable: Option<IterableValue> = match exec_ctx.data_cache.get(variable_name) {
        Some(AValue::JValueRef(call_result)) => from_call_result(call_result.clone())?,
        Some(AValue::JValueAccumulatorRef(acc)) => {
            let acc = acc.borrow();
            if acc.is_empty() {
                return Ok(None);
            }

            let call_results = acc.iter().cloned().collect::<Vec<_>>();
            let foldable = IterableVecResolvedCall::init(call_results);
            Some(Box::new(foldable))
        }
        Some(AValue::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let jvalue = iterable_value.as_jvalue();
            let result = Rc::new(jvalue.into_owned());
            let triplet = iterable_value.as_tetraplets().remove(0).triplet;

            let call_result = ResolvedCallResult { result, triplet };
            from_call_result(call_result)?
        }
        _ => return Err(AquamarineError::VariableNotFound(variable_name.to_string())),
    };

    Ok(iterable)
}

/// Constructs iterable value from resolved call result.
fn from_call_result(call_result: ResolvedCallResult) -> Result<Option<IterableValue>> {
    use AquamarineError::IncompatibleJValueType;

    let len = match &call_result.result.deref() {
        JValue::Array(array) => {
            if array.is_empty() {
                // skip fold if array is empty
                return Ok(None);
            }
            array.len()
        }
        v => return Err(IncompatibleJValueType((*v).clone(), "array")),
    };

    let foldable = IterableResolvedCall::init(call_result, len);
    let foldable = Box::new(foldable);

    Ok(Some(foldable))
}

fn handle_instruction_json_path<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
    json_path: &str,
) -> Result<Option<IterableValue>> {
    use AquamarineError::JValueAccJsonPathError;

    let iterable: Option<IterableValue> = match exec_ctx.data_cache.get(variable_name) {
        Some(AValue::JValueRef(variable)) => {
            let jvalues = apply_json_path(&variable.result, json_path)?;
            from_jvalues(jvalues, variable.triplet.clone(), json_path)
        }
        Some(AValue::JValueAccumulatorRef(acc)) => {
            let acc = acc.borrow();
            if acc.is_empty() {
                return Ok(None);
            }

            let acc_iter = acc.iter().map(|v| v.result.deref());
            let (jvalues, tetraplet_indices) = select_with_iter(acc_iter, &json_path)
                .map_err(|e| JValueAccJsonPathError(acc.clone(), json_path.to_string(), e))?;

            let jvalues = jvalues.into_iter().cloned().collect();
            let tetraplets = tetraplet_indices
                .into_iter()
                .map(|id| SecurityTetraplet {
                    triplet: acc[id].triplet.clone(),
                    json_path: json_path.to_string(),
                })
                .collect::<Vec<_>>();

            let foldable = IterableVecJsonPathResult::init(jvalues, tetraplets);
            Some(Box::new(foldable))
        }
        Some(AValue::JValueFoldCursor(fold_state)) => {
            let iterable_value = fold_state.iterable.peek().unwrap();
            let (jvalues, mut tetraplets) = iterable_value.apply_json_path_with_tetraplets(json_path)?;
            let triplet = tetraplets.remove(0).triplet;

            from_jvalues(jvalues, triplet, json_path)
        }
        _ => return Err(AquamarineError::VariableNotFound(variable_name.to_string())),
    };

    Ok(iterable)
}

fn apply_json_path<'jvalue, 'str>(jvalue: &'jvalue JValue, json_path: &'str str) -> Result<Vec<&'jvalue JValue>> {
    use AquamarineError::JValueJsonPathError;

    select(jvalue, json_path).map_err(|e| JValueJsonPathError(jvalue.clone(), json_path.to_string(), e))
}

/// Applies json_path to provided jvalues and construct IterableValue from the result and given triplet.
fn from_jvalues(jvalues: Vec<&JValue>, triplet: Rc<ResolvedTriplet>, json_path: &str) -> Option<IterableValue> {
    if jvalues.is_empty() {
        return None;
    }

    let jvalues = jvalues.into_iter().cloned().collect();

    let tetraplet = SecurityTetraplet {
        triplet,
        json_path: json_path.to_string(),
    };

    let foldable = IterableJsonPathResult::init(jvalues, tetraplet);
    Some(Box::new(foldable))
}
