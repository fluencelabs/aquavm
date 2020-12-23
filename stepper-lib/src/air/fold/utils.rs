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
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::InstructionValue;

use std::ops::Deref;

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
        _ => unreachable!("it's statically checked that other types of iterable value aren't possible here"),
    }
}

fn handle_instruction_variable<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable_name: &str,
) -> Result<Option<IterableValue>> {
    use AquamarineError::IncompatibleJValueType;

    let iterable: IterableValue = match exec_ctx.data_cache.get(variable_name) {
        Some(AValue::JValueRef(call_result)) => {
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

            let foldable = IterableResolvedCall::init(call_result.clone(), len);
            Box::new(foldable)
        }
        Some(AValue::JValueAccumulatorRef(acc)) => {
            let acc = acc.borrow();
            if acc.is_empty() {
                return Ok(None);
            }

            let call_results = acc.iter().cloned().collect::<Vec<_>>();
            let foldable = IterableVecResolvedCall::init(call_results);
            Box::new(foldable)
        }
        _ => {
            return Err(AquamarineError::InstructionError(String::from(
                "At now, it isn't possible to use fold iterator in other folds",
            )))
        }
    };

    Ok(Some(iterable))
}

fn handle_instruction_json_path<'ctx>(
    exec_ctx: &ExecutionCtx<'ctx>,
    variable: &str,
    path: &str,
) -> Result<Option<IterableValue>> {
    use AquamarineError::JValueAccJsonPathError;
    use AquamarineError::JValueJsonPathError;

    let iterable: IterableValue = match exec_ctx.data_cache.get(variable) {
        Some(AValue::JValueRef(variable)) => {
            use jsonpath_lib::select;

            let jvalues = select(&variable.result, path)
                .map_err(|e| JValueJsonPathError(variable.result.deref().clone(), path.to_string(), e))?;

            let len = jvalues.len();
            if len == 0 {
                return Ok(None);
            }

            let jvalues = jvalues.into_iter().cloned().collect();

            let tetraplet = SecurityTetraplet {
                triplet: variable.triplet.clone(),
                json_path: path.to_string(),
            };

            let foldable = IterableJsonPathResult::init(jvalues, tetraplet);
            Box::new(foldable)
        }
        Some(AValue::JValueAccumulatorRef(acc)) => {
            use jsonpath_lib::select_with_iter;

            let acc = acc.borrow();
            if acc.is_empty() {
                return Ok(None);
            }

            let acc_iter = acc.iter().map(|v| v.result.deref());
            let (jvalues, tetraplet_indices) = select_with_iter(acc_iter, &path)
                .map_err(|e| JValueAccJsonPathError(acc.clone(), path.to_string(), e))?;

            let jvalues = jvalues.into_iter().cloned().collect();
            let tetraplets = tetraplet_indices
                .into_iter()
                .map(|id| SecurityTetraplet {
                    triplet: acc[id].triplet.clone(),
                    json_path: path.to_string(),
                })
                .collect::<Vec<_>>();

            let foldable = IterableVecJsonPathResult::init(jvalues, tetraplets);
            Box::new(foldable)
        }
        _ => {
            return Err(AquamarineError::InstructionError(String::from(
                "At now, it isn't possible to use fold iterator in other folds",
            )))
        }
    };

    Ok(Some(iterable))
}
