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

use super::CallEvidenceCtx;
use super::ExecutionCtx;
use super::Instruction;
use crate::AquamarineError;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Xor(Box<Instruction>, Box<Instruction>);

impl super::ExecutableInstruction for Xor {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log::info!("xor is called with contexts: {:?} {:?}", exec_ctx, call_ctx);

        match self.0.execute(exec_ctx, call_ctx) {
            Err(AquamarineError::LocalServiceError(_)) => self.1.execute(exec_ctx, call_ctx),
            res => res,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use serde_json::json;

    #[test]
    fn xor() {
        let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
            let builtin_service = match &args[0] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            if builtin_service == "service_id_1" {
                // return a error for service with id service_id_1
                Some(IValue::Record(
                    Vec1::new(vec![IValue::S32(1), IValue::String(String::from("{}"))]).unwrap(),
                ))
            } else {
                // return success for services with other ids
                Some(IValue::Record(
                    Vec1::new(vec![IValue::S32(0), IValue::String(String::from("\"res\""))]).unwrap(),
                ))
            }
        });

        let mut vm = create_aqua_vm(call_service, "");

        let script = String::from(
            r#"
            (xor (
                (call (%current_peer_id% ("service_id_1" "local_fn_name") () result_1))
                (call (%current_peer_id% ("service_id_2" "local_fn_name") () result_2))
            ))"#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", json!({"arg3": "arg3_value"}).to_string(),]))
            .expect("call should be successful");

        let jdata: JValue = serde_json::from_str(&res.data).expect("should be valid json");

        assert_eq!(jdata["result_2"], json!("res"));

        let script = String::from(
            r#"
            (xor (
                (call (%current_peer_id% ("service_id_2" "local_fn_name") () result_1))
                (call (%current_peer_id% ("service_id_1" "local_fn_name") () result_2))
            ))"#,
        );

        let res = vm
            .call(json!(["asd", script, "{}", json!({"arg3": "arg3_value"}).to_string(),]))
            .expect("call should be successful");

        let jdata: JValue = serde_json::from_str(&res.data).expect("should be valid json");

        assert_eq!(jdata["result_1"], json!("res"));
    }
}
