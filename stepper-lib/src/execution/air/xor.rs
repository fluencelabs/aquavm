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
use super::ExecutionError::LocalServiceError;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use crate::log_instruction;

use air_parser::ast::Xor;

impl<'i> super::ExecutableInstruction<'i> for Xor<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        log_instruction!(xor, exec_ctx, trace_ctx);

        exec_ctx.subtree_complete = true;
        match self.0.execute(exec_ctx, trace_ctx) {
            Err(LocalServiceError(_)) => {
                exec_ctx.subtree_complete = true;
                self.1.execute(exec_ctx, trace_ctx)
            }
            res => res,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::execution_trace::ExecutionTrace;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::CallServiceClosure;
    use aqua_test_utils::IValue;
    use aqua_test_utils::NEVec;

    use std::rc::Rc;

    fn fallible_call_service(fallible_service_id: String) -> CallServiceClosure {
        Box::new(move |_, args| -> Option<IValue> {
            let builtin_service = match &args[0] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            // return a error for service with such id
            if builtin_service == &fallible_service_id {
                Some(IValue::Record(
                    NEVec::new(vec![IValue::S32(1), IValue::String(String::from(r#""error""#))]).unwrap(),
                ))
            } else {
                // return success for services with other ids
                Some(IValue::Record(
                    NEVec::new(vec![IValue::S32(0), IValue::String(String::from(r#""res""#))]).unwrap(),
                ))
            }
        })
    }

    #[test]
    fn xor() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let local_peer_id = "local_peer_id";
        let fallible_service_id = String::from("service_id_1");
        let mut vm = create_aqua_vm(fallible_call_service(fallible_service_id), local_peer_id);

        let script = format!(
            r#"
            (xor
                (call "{0}" ("service_id_1" "local_fn_name") [] result_1)
                (call "{0}" ("service_id_2" "local_fn_name") [] result_2)
            )"#,
            local_peer_id,
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
        let executed_call_result = Call(Executed(Rc::new(JValue::String(String::from("res")))));

        assert_eq!(actual_trace.len(), 2);
        assert_eq!(actual_trace[0], Call(CallServiceFailed(String::from(r#""error""#))));
        assert_eq!(actual_trace[1], executed_call_result);

        let script = format!(
            r#"
            (xor
                (call "{0}" ("service_id_2" "local_fn_name") [] result_1)
                (call "{0}" ("service_id_1" "local_fn_name") [] result_2)
            )"#,
            local_peer_id
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");

        assert_eq!(actual_trace.len(), 1);
        assert_eq!(actual_trace[0], executed_call_result);
    }

    #[test]
    fn xor_par() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let fallible_service_id = String::from("service_id_1");
        let local_peer_id = "local_peer_id";
        let mut vm = create_aqua_vm(fallible_call_service(fallible_service_id.clone()), local_peer_id);

        let script = format!(
            r#"
            (xor
                (par
                    (seq
                        (call "{0}" ("service_id_2" "local_fn_name") [] result_1)
                        (call "{0}" ("service_id_2" "local_fn_name") [] result_2)
                    )
                    (par
                        (call "{0}" ("service_id_1" "local_fn_name") [] result_3)
                        (call "{0}" ("service_id_2" "local_fn_name") [] result_4)
                    )
                )
                (seq
                    (call "{0}" ("service_id_2" "local_fn_name") [] result_4)
                    (call "{0}" ("service_id_2" "local_fn_name") [] result_5)
                )
            )"#,
            local_peer_id
        );

        let result = call_vm!(vm, "asd", script.clone(), "[]", "[]");
        let actual_trace: ExecutionTrace = serde_json::from_slice(&result.data).expect("should be valid json");

        let res = String::from("res");
        let executed_call_result = Rc::new(JValue::String(res));

        let expected_trace = vec![
            Par(2, 2),
            Call(Executed(executed_call_result.clone())),
            Call(Executed(executed_call_result.clone())),
            Par(1, 0),
            Call(CallServiceFailed(String::from(r#""error""#))),
            Call(Executed(executed_call_result.clone())),
            Call(Executed(executed_call_result.clone())),
        ];

        assert_eq!(actual_trace, expected_trace);

        let result = call_vm!(vm, "asd", script, "[]", result.data);
        let actual_trace: ExecutionTrace = serde_json::from_slice(&result.data).expect("should be valid json");
        assert_eq!(actual_trace, expected_trace);
    }
}
