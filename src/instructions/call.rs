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
use crate::AquaData;
use crate::AquamarineError;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

const CURRENT_PEER_ALIAS: &str = "%current_peer_id%";
const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

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
        log::info!("call is called with context: {:?}", ctx);

        let (peer_pk, service_id, func_name) = parse_peer_fn_parts(&self.0, &self.1)?;
        let function_args = parse_args(&self.2, &ctx.data)?;
        let function_args = serde_json::to_string(&function_args)?;
        let result_name = parse_result_name(&self.3)?;

        let current_peer_id = std::env::var(CURRENT_PEER_ID_ENV_NAME)
            .map_err(|e| AquamarineError::CurrentPeerIdNotSet(e))?;

        if peer_pk == current_peer_id || peer_pk == CURRENT_PEER_ALIAS {
            let result = unsafe {
                crate::call_service(service_id.to_string(), func_name.to_string(), function_args)
            };
            if result.ret_code != crate::CALL_SERVICE_SUCCESS {
                return Err(AquamarineError::LocalServiceError(result.result));
            }

            let result: serde_json::Value = serde_json::from_str(&result.result)?;
            ctx.data.insert(result_name.to_string(), result);
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

fn parse_args(args: &[String], data: &AquaData) -> Result<serde_json::Value> {
    let mut result = Vec::with_capacity(args.len());

    for arg in args {
        let mut split_arg: Vec<&str> = arg.splitn(2, '.').collect();
        let variable_name = split_arg.remove(0);

        let value_by_key = data
            .get(variable_name)
            .ok_or_else(|| AquamarineError::VariableNotFound(String::from(variable_name)))?;

        let value = if !split_arg.is_empty() {
            let json_path = split_arg.remove(0);
            let values = jsonpath_lib::select(&value_by_key, json_path)
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

    Ok(serde_json::Value::Array(result))
}

fn parse_result_name(result_name: &str) -> Result<&str> {
    if !result_name.is_empty() {
        Ok(result_name)
    } else {
        Err(AquamarineError::InstructionError(String::from(
            "result name of a call instruction must be non empty",
        )))
    }
}
