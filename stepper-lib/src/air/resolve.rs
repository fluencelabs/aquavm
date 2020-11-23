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
use super::JValuableResult;
use crate::AValue;
use crate::AquamarineError;
use crate::ExecutedCallResult;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::InstructionValue;

use std::borrow::Cow;
use std::rc::Rc;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_args<'i>(
    value: &InstructionValue<'i>,
    exec_ctx: &ExecutionCtx<'i>,
) -> Result<(Cow<'i, JValue>, Vec<SecurityTetraplet>)> {
    fn handle_string_arg(arg: &str) -> Result<(Cow<'i, JValue>, Vec<SecurityTetraplet>)> {
        let jvalue = JValue::String(arg.to_string());
        let tetraplet = SecurityTetraplet::initiator_tetraplet(exec_ctx);

        Ok((Cow::Owned(jvalue), vec![tetraplet]))
    }

    match value {
        InstructionValue::CurrentPeerId => handle_string_arg(exec_ctx.current_peer_id.as_str()),
        InstructionValue::InitPeerId => handle_string_arg(exec_ctx.init_peer_id.as_str()),
        InstructionValue::Literal(value) => handle_string_arg(value),
        InstructionValue::Variable(name) => {
            let resolved = resolve_to_call_result(name, ctx)?;
            let jvalue = resolved.as_jvalue();
            let tetraplets = resolved.as_tetraplets();

            Ok((jvalue, tetraplets))
        }
        InstructionValue::JsonPath { variable, path } => {
            let resolved = resolve_to_call_result(variable, ctx)?;
            let (jvalue, tetraplets) = resolved.apply_json_path_with_tetraplets(path)?;
            let jvalue = jvalue.iter().cloned().collect::<Vec<_>>();
            let jvalue = JValue::Array(jvalue);

            Ok((Cow::Owned(jvalue), tetraplets))
        }
    }
}

/// Takes variable's value from `ExecutionCtx::data_cache` by name.
pub(crate) fn resolve_to_call_result<'name, 'exec_ctx>(
    name: &'name str,
    ctx: &'exec_ctx ExecutionCtx<'name>,
) -> Result<impl JValuableResult> {
    use AquamarineError::VariableNotFound;

    let value = ctx
        .data_cache
        .get(name)
        .ok_or_else(|| VariableNotFound(name.to_string()))?;

    match value {
        AValue::JValueFoldCursor(fold_state) => match &fold_state.iterable {
            AValue::JValueRef(call_result) => match call_result.as_ref() {
                JValue::Array(_array) => unimplemented!(),
                _ => unreachable!("fold state must be well-formed because it is changed only by stepper"),
            },
            AValue::JValueAccumulatorRef(_acc) => unimplemented!(),
            _ => unreachable!("fold state must be well-formed because it is changed only by stepper"),
        },
        AValue::JValueRef(value) => Ok(value.clone()),
        AValue::JValueAccumulatorRef(acc) => Ok(acc.borrow()),
    }
}

pub(crate) fn apply_json_path(jvalue: JValue, json_path: &str) -> Result<JValue> {
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

/// Applies `json_path` to `jvalue`
fn find_by_json_path<'jvalue, 'json_path>(
    jvalue: &'jvalue JValue,
    json_path: &'json_path str,
) -> Result<Vec<&'jvalue JValue>> {
    use AquamarineError::VariableNotInJsonPath as JsonPathError;

    jsonpath_lib::select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))
}
