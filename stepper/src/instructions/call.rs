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
use crate::AValue;
use crate::AquamarineError;
use crate::Result;
use crate::SerdeValue;

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

            let result: SerdeValue = serde_json::from_str(&result.result)
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

fn parse_args(args: &[String], ctx: &ExecutionContext) -> Result<SerdeValue> {
    let mut result = Vec::with_capacity(args.len());

    for arg in args {
        let mut split_arg: Vec<&str> = arg.splitn(2, '.').collect();
        let variable_name = split_arg.remove(0);

        let value_from_data = ctx
            .data
            .get(variable_name)
            .ok_or_else(|| AquamarineError::VariableNotFound(variable_name.to_string()))?;
        let value_by_key = match value_from_data {
            AValue::SerdeValue(value) => value,
            AValue::Iterator(values, cursor) => &values[*cursor],
            v => {
                return Err(AquamarineError::IncompatibleAValueType(
                    v.clone(),
                    String::from("ServeValue or Iterator"),
                ))
            }
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

    Ok(SerdeValue::Array(result))
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
    result: SerdeValue,
) -> Result<()> {
    use std::collections::hash_map::Entry;

    let is_array = result_variable_name.ends_with("[]");
    if is_array {
        match ctx
            .data
            .entry(result_variable_name.strip_suffix("[]").unwrap().to_string())
        {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                AValue::Accumulator(values) => values.push_back(result),
                v => {
                    return Err(AquamarineError::IncompatibleAValueType(
                        v.clone(),
                        String::from("Accumulator"),
                    ))
                }
            },
            Entry::Vacant(entry) => {
                let mut list = std::collections::LinkedList::new();
                list.push_back(result);
                entry.insert(AValue::Accumulator(list));
            }
        }
    } else {
        // TODO: check that value already present
        ctx.data
            .insert(result_variable_name.to_string(), AValue::SerdeValue(result));
    }

    Ok(())
}
