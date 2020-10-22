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

mod parsed_call;
mod utils;

use parsed_call::ParsedCall;

use super::CallEvidenceCtx;
use super::ExecutionCtx;
use crate::AquamarineError::VariableNotFound;
use crate::AquamarineError::VariableNotInJsonPath;
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
pub(self) enum PeerPart {
    PeerPk(String),
    PeerPkWithServiceId(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub(self) enum FunctionPart {
    FuncName(String),
    ServiceIdWithFuncName(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Call(PeerPart, FunctionPart, Vec<String>, String);

impl super::ExecutableInstruction for Call {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log::info!("call {:?} is called with contexts: {:?} {:?}", self, exec_ctx, call_ctx);

        let parsed_call = match ParsedCall::new(self, exec_ctx) {
            Ok(parsed_call) => parsed_call,
            // to support lazy variable evaluation
            Err(VariableNotFound(variable_name)) => {
                log::info!(r#"variable with name "{}" not found, waiting"#, variable_name);
                exec_ctx.subtree_complete = false;
                return Ok(());
            }
            Err(VariableNotInJsonPath(variable, json_path, json_path_err)) => {
                log::info!(
                    r#"variable not found with json path "{}" in {:?} with error "{:?}", waiting"#,
                    json_path,
                    variable,
                    json_path_err
                );
                exec_ctx.subtree_complete = false;
                return Ok(());
            }
            Err(err) => return Err(err),
        };

        parsed_call.execute(exec_ctx, call_ctx)
    }
}

#[cfg(test)]
mod tests {
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use serde_json::json;

    #[test]
    fn current_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "test_peer_id");

        let script = String::from(
            r#"
               (call (%current_peer_id% ("local_service_id" "local_fn_name") (value) result_name))
            "#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", "{\"value\": \"test\"}",]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));

        let script = String::from(
            r#"
               (call ("test_peer_id" ("local_service_id" "local_fn_name") (value) result_name))
            "#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", "{\"value\": \"test\"}",]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));
    }

    #[test]
    fn remote_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "");
        let remote_peer_id = String::from("some_remote_peer_id");

        let script = format!(
            r#"(call ("{}" ("local_service_id" "local_fn_name") (value) result_name))"#,
            remote_peer_id
        );

        let res = vm
            .call(json!(["asd", script, "{}", "{\"value\": \"test\"}",]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }

    #[test]
    fn variables() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "");

        let script = format!(r#"(call (remote_peer_id ("some_service_id" "local_fn_name") ("param") result_name))"#,);

        let res = vm
            .call(json!(["asd", script, "{}", "{\"remote_peer_id\": \"some_peer_id\"}",]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("some_peer_id")]);
    }

    #[test]
    fn string_parameters() {
        let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            Some(IValue::Record(
                Vec1::new(vec![IValue::S32(0), IValue::String(arg.clone())]).unwrap(),
            ))
        });

        let mut vm = create_aqua_vm(call_service, "");

        let script = format!(
            r#"(call (%current_peer_id% ("some_service_id" "local_fn_name") ("arg1" "arg2" arg3) result_name))"#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", json!({"arg3": "arg3_value"}).to_string(),]))
            .expect("call should be successful");

        let jdata: JValue = serde_json::from_str(&res.data).expect("should be valid json");

        assert_eq!(jdata["result_name"], json!(["arg1", "arg2", "arg3_value"]));
    }
}
