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

use super::ExecutionContext;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

const CURRENT_PEER_ALIAS: &str = "%current_peer_id%";

/*
   (current)
   (pk $pk)
   (pk $pk $srv_id)
   PEER_PART: resolves to (peer_pk) \/ (peer_pk, pk_srv_id)

   (fn $name)
   (fn $name $srv_id)
   FN_PART: resolves to (fn_name) \/ (fn_srv_id, fn_name)
*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum PeerPart {
    PeerPk(String),
    PeerPkWithPkServiceId(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum FunctionPart {
    FuncName(String),
    ServiceIdWithFuncName(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Call(PeerPart, FunctionPart, Vec<String>, String);

impl super::ExecutableInstruction for Call {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("call {:?} is called with context {:?}", self, ctx);

        let peer_part = &self.0;
        let function_part = &self.1;
        let arguments = &self.2;
        let result_variable_name = &self.3;

        let (peer_pk, service_id, func_name) = parse_peer_fn_parts(peer_part, function_part)?;
        let function_args = parse_args(arguments, ctx)?;
        let function_args = serde_json::to_string(&function_args)
            .map_err(|e| AquamarineError::FuncArgsSerdeError(function_args, e))?;
        let result_variable_name = parse_result_variable_name(result_variable_name)?;

        if peer_pk == ctx.current_peer_id || peer_pk == CURRENT_PEER_ALIAS {
            let result = unsafe {
                crate::call_service(service_id.to_string(), func_name.to_string(), function_args)
            };
            if result.ret_code != crate::CALL_SERVICE_SUCCESS {
                return Err(AquamarineError::LocalServiceError(result.result));
            }

            let result: JValue = serde_json::from_str(&result.result)
                .map_err(|e| AquamarineError::CallServiceSerdeError(result, e))?;
            set_result(ctx, result_variable_name, result)?;
        } else {
            ctx.next_peer_pks.push(peer_pk.to_string());
        }

        Ok(())
    }
}

#[rustfmt::skip]
fn parse_peer_fn_parts<'a>(
    peer_part: &'a PeerPart,
    fn_part: &'a FunctionPart,
) -> Result<(&'a str, &'a str, &'a str)> {
    match (peer_part, fn_part) {
        (PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id), FunctionPart::ServiceIdWithFuncName(_service_id, func_name)) => {
            Ok((peer_pk, peer_service_id, func_name))
        },
        (PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id), FunctionPart::FuncName(func_name)) => {
            Ok((peer_pk, peer_service_id, func_name))
        },
        (PeerPart::PeerPk(peer_pk), FunctionPart::ServiceIdWithFuncName(service_id, func_name)) => {
            Ok((peer_pk, service_id, func_name))
        }
        (PeerPart::PeerPk(_), FunctionPart::FuncName(_)) => Err(AquamarineError::InstructionError(
            String::from("call should have service id specified by peer part or function part"),
        )),
    }
}

#[rustfmt::skip]
fn parse_args(args: &[String], ctx: &ExecutionContext) -> Result<JValue> {
    let mut result = Vec::with_capacity(args.len());

    for arg in args {
        let mut split_arg: Vec<&str> = arg.splitn(2, '.').collect();
        let variable_name = split_arg.remove(0);

        let value_by_key = match (ctx.data.get(variable_name), ctx.folds.get(variable_name)) {
            (_, Some(fold_state)) => match ctx.data.get(&fold_state.iterable_name) {
                Some(JValue::Array(values)) => &values[fold_state.cursor],
                Some(v) => return Err(AquamarineError::IncompatibleJValueType(v.clone(), String::from("array"))),
                None => return Err(AquamarineError::VariableNotFound(fold_state.iterable_name.clone())),
            },
            (Some(value), None) => value,
            (None, None) => return Err(AquamarineError::VariableNotFound(variable_name.to_string())),
        };

        let value = if !split_arg.is_empty() {
            let json_path = split_arg.remove(0);
            let values = jsonpath_lib::select(value_by_key, json_path)
                .map_err(|e| AquamarineError::VariableNotInJsonPath(String::from(json_path), e))?;

            if values.len() != 1 {
                return Err(AquamarineError::MultipleValuesInJsonPath(String::from(
                    json_path,
                )));
            }

            values[0].clone()
        } else {
            value_by_key.clone()
        };

        result.push(value);
    }

    Ok(JValue::Array(result))
}

fn parse_result_variable_name(result_name: &str) -> Result<&str> {
    if !result_name.is_empty() {
        Ok(result_name)
    } else {
        Err(AquamarineError::InstructionError(String::from(
            "result name of a call instruction must be non empty",
        )))
    }
}

fn set_result(
    ctx: &mut ExecutionContext,
    result_variable_name: &str,
    result: JValue,
) -> Result<()> {
    use std::collections::hash_map::Entry;

    let is_array = result_variable_name.ends_with("[]");
    if !is_array {
        if ctx
            .data
            .insert(result_variable_name.to_string(), result)
            .is_some()
        {
            return Err(AquamarineError::MultipleVariablesFound(
                result_variable_name.to_string(),
            ));
        }

        return Ok(());
    }

    match ctx
        .data
        // unwrap is safe because it's been checked for []
        .entry(result_variable_name.strip_suffix("[]").unwrap().to_string())
    {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            JValue::Array(values) => values.push(result),
            v => {
                return Err(AquamarineError::IncompatibleJValueType(
                    v.clone(),
                    String::from("Array"),
                ))
            }
        },
        Entry::Vacant(entry) => {
            entry.insert(JValue::Array(vec![result]));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;

    use serde_json::json;

    #[test]
    fn current_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service());

        let script = String::from(
            r#"
               (call (%current_peer_id% (local_service_id local_fn_name) (value) result_name))
            "#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));

        let script = String::from(
            r#"
               (call (test_peer_id (local_service_id local_fn_name) (value) result_name))
            "#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));
    }

    #[test]
    fn remote_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service());
        let remote_peer_id = String::from("some_remote_peer_id");

        let script = format!(
            "(call ({} (local_service_id local_fn_name) (value) result_name))",
            remote_peer_id
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }
}
