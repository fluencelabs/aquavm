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
mod triplet;
mod utils;

use parsed_call::ParsedCall;

use super::CallEvidenceCtx;
use super::ExecutionCtx;
use crate::log_instruction;
use crate::AquamarineError::VariableNotFound;
use crate::AquamarineError::VariableNotInJsonPath;
use crate::Result;

use air_parser::ast::Call;

/*
   (current)
   (pk $pk)
   (pk $pk $srv_id)
   PEER_PART: resolves to (peer_pk) \/ (peer_pk, pk_srv_id)

   (fn $name)
   (fn $name $srv_id)
   FN_PART: resolves to (fn_name) \/ (fn_srv_id, fn_name)
*/

impl<'i> super::ExecutableInstruction<'i> for Call<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log_instruction!(call, exec_ctx, call_ctx);

        let parsed_call = match ParsedCall::new(self, exec_ctx) {
            Ok(parsed_call) => parsed_call,
            // to support lazy variable evaluation
            Err(VariableNotFound(variable_name)) => {
                log::trace!(r#"variable with name "{}" not found, waiting"#, variable_name);
                exec_ctx.subtree_complete = false;
                return Ok(());
            }
            Err(VariableNotInJsonPath(variable, json_path, json_path_err)) => {
                log::trace!(
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
    use crate::call_evidence::CallEvidencePath;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;
    use aqua_test_utils::set_variable_call_service;
    use aqua_test_utils::unit_call_service;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use std::rc::Rc;

    #[test]
    fn current_peer_id_call() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let mut vm = create_aqua_vm(unit_call_service(), "test_peer_id");

        let script = String::from(
            r#"
               (call %current_peer_id% ("local_service_id" "local_fn_name") [] result_name)
            "#,
        );

        let res = call_vm!(vm, "asd", script.clone(), "[]", "[]");
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 1);
        assert_eq!(
            call_path[0],
            Call(Executed(Rc::new(JValue::String(String::from("test")))))
        );
        assert!(res.next_peer_pks.is_empty());

        // test that empty string for data works
        let res_with_empty_string = call_vm!(vm, "asd", script, "", "");
        assert_eq!(res_with_empty_string, res);

        let script = String::from(
            r#"
               (call "test_peer_id" ("local_service_id" "local_fn_name") [] result_name)
            "#,
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 1);
        assert_eq!(
            call_path[0],
            Call(Executed(Rc::new(JValue::String(String::from("test")))))
        );
        assert!(res.next_peer_pks.is_empty());
    }

    #[test]
    fn remote_peer_id_call() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let some_local_peer_id = String::from("some_local_peer_id");
        let mut vm = create_aqua_vm(echo_string_call_service(), some_local_peer_id.clone());

        let remote_peer_id = String::from("some_remote_peer_id");
        let script = format!(
            r#"(call "{}" ("local_service_id" "local_fn_name") [value] result_name)"#,
            remote_peer_id
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 1);
        assert_eq!(call_path[0], Call(RequestSent(some_local_peer_id)));
        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }

    #[test]
    fn variables() {
        let mut vm = create_aqua_vm(unit_call_service(), "remote_peer_id");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#""remote_peer_id""#), "set_variable");

        let script = format!(
            r#"
            (seq 
                (call "set_variable" ("some_service_id" "local_fn_name") [] remote_peer_id)
                (call remote_peer_id ("some_service_id" "local_fn_name") [] result_name)
            )
        "#,
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "[]", "[]");
        let res = call_vm!(vm, "asd", script, "[]", res.data);

        assert!(res.next_peer_pks.is_empty());
    }

    #[test]
    fn string_parameters() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            Some(IValue::Record(
                Vec1::new(vec![IValue::S32(0), IValue::String(arg.clone())]).unwrap(),
            ))
        });

        let mut vm = create_aqua_vm(call_service, "A");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#""arg3_value""#), "set_variable");

        let script = String::from(
            r#"
            (seq 
                (call "set_variable" ("some_service_id" "local_fn_name") [] arg3)
                (call "A" ("some_service_id" "local_fn_name") ["arg1" "arg2" arg3] result)
            )
        "#,
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "[]", "[]");
        let res = call_vm!(vm, "asd", script, "[]", res.data);
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 2);
        assert_eq!(
            call_path[1],
            Call(Executed(Rc::new(JValue::Array(vec![
                JValue::String(String::from("arg1")),
                JValue::String(String::from("arg2")),
                JValue::String(String::from("arg3_value")),
            ]))))
        );
    }
}
