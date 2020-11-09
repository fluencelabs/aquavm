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

use super::ExecutionCtx;
use crate::call_evidence::CallEvidenceCtx;
use crate::call_evidence::CallResult;
use crate::call_evidence::EvidenceState;
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use air_parser::ast::{CallOutput, Value};

use std::{borrow::Cow, cell::RefCell, rc::Rc};

/// Writes result of a local `Call` instruction to `ExecutionCtx` at `output`
pub(super) fn set_local_call_result<'i>(
    output: CallOutput<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    result: Rc<JValue>,
) -> Result<()> {
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use AquamarineError::*;

    match output {
        CallOutput::Scalar(name) => {
            match exec_ctx.data_cache.entry(name.to_string()) {
                Vacant(entry) => entry.insert(AValue::JValueRef(result)),
                Occupied(entry) => return Err(MultipleVariablesFound(entry.key().clone())),
            };
        }
        CallOutput::Accumulator(name) => {
            match exec_ctx.data_cache.entry(name.to_string()) {
                Occupied(mut entry) => match entry.get_mut() {
                    // if result is an array, insert result to the end of the array
                    AValue::JValueAccumulatorRef(values) => values.borrow_mut().push(result),
                    v => return Err(IncompatibleAValueType(format!("{:?}", v), String::from("Array"))),
                },
                Vacant(entry) => {
                    entry.insert(AValue::JValueAccumulatorRef(RefCell::new(vec![result])));
                }
            };
        }
        CallOutput::None => {}
    }

    Ok(())
}

/// Writes evidence of a particle being sent to remote node
pub(super) fn set_remote_call_result<'i>(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx<'i>,
    call_ctx: &mut CallEvidenceCtx,
) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent(exec_ctx.current_peer_id.clone()));
    log::info!(
        target: EVIDENCE_CHANGING,
        "  adding new call evidence state {:?}",
        new_evidence_state
    );
    call_ctx.new_path.push_back(new_evidence_state);
}

/// Applies `json_path` to `jvalue`
pub(super) fn find_by_json_path<'jvalue, 'json_path>(
    jvalue: &'jvalue JValue,
    json_path: &'json_path str,
) -> Result<Vec<&'jvalue JValue>> {
    use AquamarineError::VariableNotInJsonPath as JsonPathError;

    jsonpath_lib::select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))
}

/// Takes variable's value from `ExecutionCtx::data_cache`
/// TODO: maybe return &'i JValue?
pub(super) fn resolve_variable<'exec_ctx, 'i>(variable: &'i str, ctx: &'exec_ctx ExecutionCtx<'i>) -> Result<JValue> {
    use AquamarineError::VariableNotFound;

    let value = ctx
        .data_cache
        .get(variable)
        .ok_or_else(|| VariableNotFound(variable.to_string()))?;

    match value {
        AValue::JValueFoldCursor(fold_state) => {
            if let JValue::Array(array) = fold_state.iterable.as_ref() {
                Ok(array[fold_state.cursor].clone())
            } else {
                unreachable!("fold state must be well-formed because it is changed only by stepper")
            }
        }
        AValue::JValueRef(value) => Ok(value.as_ref().clone()),
        AValue::JValueAccumulatorRef(acc) => {
            let owned_acc = acc.borrow().iter().map(|v| v.as_ref()).cloned().collect::<Vec<_>>();
            Ok(JValue::Array(owned_acc))
        }
    }
}

pub(super) fn apply_json_path<'i>(jvalue: JValue, json_path: &'i str) -> Result<JValue> {
    let values = find_by_json_path(&jvalue, json_path)?;
    if values.is_empty() {
        return Err(AquamarineError::VariableNotFound(json_path.to_string()));
    }

    if values.len() != 1 {
        return Err(AquamarineError::MultipleValuesInJsonPath(json_path.to_string()));
    }

    // TODO: sure need this clone?
    Ok(values[0].clone())
}

pub(super) fn require_string(value: JValue) -> Result<String> {
    if let JValue::String(s) = value {
        Ok(s)
    } else {
        Err(AquamarineError::IncompatibleJValueType(value, "string".to_string()))
    }
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc
pub(super) fn resolve_value<'i, 'a: 'i>(value: &'a Value<'i>, ctx: &'a ExecutionCtx<'i>) -> Result<Cow<'i, str>> {
    let resolved = match value {
        Value::CurrentPeerId => Cow::Borrowed(ctx.current_peer_id.as_str()),
        Value::Literal(value) => Cow::Borrowed(*value),
        Value::Variable(name) => {
            let resolved = resolve_variable(name, ctx)?;
            let resolved = require_string(resolved)?;
            Cow::Owned(resolved)
        }
        Value::JsonPath { variable, path } => {
            let resolved = resolve_variable(variable, ctx)?;
            let resolved = apply_json_path(resolved, path)?;
            let resolved = require_string(resolved)?;
            Cow::Owned(resolved)
        }
    };

    Ok(resolved)
}

/// Resolve value to JValue, similar to `resolve_value`
pub(super) fn resolve_jvalue<'i>(value: &Value<'i>, ctx: &ExecutionCtx<'i>) -> Result<JValue> {
    let value = match value {
        Value::CurrentPeerId => JValue::String(ctx.current_peer_id.clone()),
        Value::Literal(value) => JValue::String(value.to_string()),
        Value::Variable(name) => resolve_variable(name, ctx)?,
        Value::JsonPath { variable, path } => {
            let value = resolve_variable(variable, ctx)?;
            apply_json_path(value, path)?
        }
    };

    Ok(value)
}
