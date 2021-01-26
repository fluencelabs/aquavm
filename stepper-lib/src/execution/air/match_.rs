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
use super::ExecutionError;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use crate::log_instruction;

use air_parser::ast::Match;
use air_parser::ast::MatchableValue;

impl<'i> super::ExecutableInstruction<'i> for Match<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        log_instruction!(match_, exec_ctx, trace_ctx);

        let left_value = &self.0;
        let right_value = &self.1;
        let is_equal_values = compare_matchable(left_value, right_value, exec_ctx)?;

        if !is_equal_values {
            return Err(ExecutionError::MatchWithoutXorError);
        }

        self.2.execute(exec_ctx, trace_ctx)
    }
}

fn compare_matchable<'ctx>(
    left: &MatchableValue<'_>,
    right: &MatchableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    use crate::execution::utils::resolve_to_jvaluable;
    use MatchableValue::*;

    match (left, right) {
        (Literal(left_name), Literal(right_name)) => Ok(left_name == right_name),
        (Variable(left_name), Variable(right_name)) => {
            let left_jvaluable = resolve_to_jvaluable(left_name, exec_ctx)?;
            let left_value = left_jvaluable.as_jvalue();

            let right_jvaluable = resolve_to_jvaluable(right_name, exec_ctx)?;
            let right_value = right_jvaluable.as_jvalue();

            Ok(left_value == right_value)
        }
        (JsonPath { variable: lv, path: lp }, JsonPath { variable: rv, path: rp }) => {
            let left_jvaluable = resolve_to_jvaluable(lv, exec_ctx)?;
            let left_value = left_jvaluable.apply_json_path(lp)?;

            let right_jvaluable = resolve_to_jvaluable(rv, exec_ctx)?;
            let right_value = right_jvaluable.apply_json_path(rp)?;

            Ok(left_value == right_value)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::execution_trace::ExecutionTrace;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;

    use std::rc::Rc;

    #[test]
    fn match_equal() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let set_variable_peer_id = "set_variable_peer_id";
        let mut set_variable_vm = create_aqua_vm(echo_string_call_service(), set_variable_peer_id);

        let local_peer_id = "local_peer_id";
        let mut vm = create_aqua_vm(echo_string_call_service(), local_peer_id);

        let script = format!(
            r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_1"] value_2)
                )
                (xor
                    (match value_1 value_2
                        (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#,
            set_variable_peer_id, local_peer_id
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
        let res = call_vm!(vm, "asd", script, "", res.data);

        let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
        let expected_executed_call_result = Call(Executed(Rc::new(JValue::String(String::from("result_1")))));

        assert_eq!(actual_trace.len(), 3);
        assert_eq!(actual_trace[2], expected_executed_call_result);
    }

    #[test]
    fn match_not_equal() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let set_variable_peer_id = "set_variable_peer_id";
        let mut set_variable_vm = create_aqua_vm(echo_string_call_service(), set_variable_peer_id);

        let local_peer_id = "local_peer_id";
        let mut vm = create_aqua_vm(echo_string_call_service(), local_peer_id);

        let script = format!(
            r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_2"] value_2)
                )
                (xor
                    (match value_1 value_2
                        (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#,
            set_variable_peer_id, local_peer_id
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
        let res = call_vm!(vm, "asd", script, "", res.data);

        let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
        let expected_executed_call_result = Call(Executed(Rc::new(JValue::String(String::from("result_2")))));

        assert_eq!(actual_trace.len(), 3);
        assert_eq!(actual_trace[2], expected_executed_call_result);
    }

    #[test]
    fn match_without_xor() {
        let set_variable_peer_id = "set_variable_peer_id";
        let mut set_variable_vm = create_aqua_vm(echo_string_call_service(), set_variable_peer_id);

        let local_peer_id = "local_peer_id";
        let mut vm = create_aqua_vm(echo_string_call_service(), local_peer_id);

        let script = format!(
            r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_2"] value_2)
                )
                (match value_1 value_2
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                )
            )"#,
            set_variable_peer_id, local_peer_id
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
        let res = call_vm!(vm, "asd", script.clone(), "", res.data);

        assert_eq!(res.ret_code, 1015);

        let res = call_vm!(vm, "asd", script, "", res.data);

        assert_eq!(res.ret_code, 1015);
    }
}
