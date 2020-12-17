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
use crate::log_instruction;
use crate::AquamarineError::LocalServiceError;
use crate::Result;

use air_parser::ast::Xor;

impl<'i> super::ExecutableInstruction<'i> for Xor<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log_instruction!(xor, exec_ctx, call_ctx);

        exec_ctx.subtree_complete = true;
        match self.0.execute(exec_ctx, call_ctx) {
            Err(LocalServiceError(_)) => {
                exec_ctx.subtree_complete = true;
                self.1.execute(exec_ctx, call_ctx)
            }
            res => res,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::call_evidence::CallEvidencePath;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::HostExportedFunc;
    use aqua_test_utils::IValue;
    use aqua_test_utils::Vec1;

    use std::rc::Rc;

    fn fallible_call_service(fallible_service_id: String) -> HostExportedFunc {
        Box::new(move |_, args| -> Option<IValue> {
            let builtin_service = match &args[0] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            // return a error for service with such id
            if builtin_service == &fallible_service_id {
                Some(IValue::Record(
                    Vec1::new(vec![IValue::S32(1), IValue::String(String::from(r#""error""#))]).unwrap(),
                ))
            } else {
                // return success for services with other ids
                Some(IValue::Record(
                    Vec1::new(vec![IValue::S32(0), IValue::String(String::from(r#""res""#))]).unwrap(),
                ))
            }
        })
    }

    #[test]
    fn xor() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let fallible_service_id = String::from("service_id_1");
        let mut vm = create_aqua_vm(fallible_call_service(fallible_service_id), "");

        let script = String::from(
            r#"
            (xor
                (call %current_peer_id% ("service_id_1" "local_fn_name") [] result_1)
                (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_2)
            )"#,
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be valid json");

        assert_eq!(call_path.len(), 2);
        assert_eq!(call_path[0], Call(CallServiceFailed(String::from(r#""error""#))));
        assert_eq!(
            call_path[1],
            Call(Executed(Rc::new(JValue::String(String::from("res")))))
        );

        let script = String::from(
            r#"
            (xor
                (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_1)
                (call %current_peer_id% ("service_id_1" "local_fn_name") [] result_2)
            )"#,
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let call_path: CallEvidencePath = serde_json::from_slice(&res.data).expect("should be valid json");

        assert_eq!(call_path.len(), 1);
        assert_eq!(
            call_path[0],
            Call(Executed(Rc::new(JValue::String(String::from("res")))))
        );
    }

    #[test]
    fn xor_par() {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        let fallible_service_id = String::from("service_id_1");
        let mut vm = create_aqua_vm(fallible_call_service(fallible_service_id), "");

        let script = String::from(
            r#"
            (xor
                (par
                    (seq
                        (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_1)
                        (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_2)
                    )
                    (par
                        (call %current_peer_id% ("service_id_1" "local_fn_name") [] result_3)
                        (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_4)
                    )
                )
                (seq
                    (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_4)
                    (call %current_peer_id% ("service_id_2" "local_fn_name") [] result_5)
                )
            )"#,
        );

        let result = call_vm!(vm, "asd", script.clone(), "[]", "[]");
        let result_path: CallEvidencePath = serde_json::from_slice(&result.data).expect("should be valid json");

        let res = String::from("res");

        let right_path = vec![
            Par(2, 2),
            Call(Executed(Rc::new(JValue::String(res.clone())))),
            Call(Executed(Rc::new(JValue::String(res.clone())))),
            Par(1, 0),
            Call(CallServiceFailed(String::from(r#""error""#))),
            Call(Executed(Rc::new(JValue::String(res.clone())))),
            Call(Executed(Rc::new(JValue::String(res)))),
        ];

        assert_eq!(result_path, right_path);

        let result = call_vm!(vm, "asd", script, "[]", result.data);
        let result_path: CallEvidencePath = serde_json::from_slice(&result.data).expect("should be valid json");
        assert_eq!(result_path, right_path);
    }
}
