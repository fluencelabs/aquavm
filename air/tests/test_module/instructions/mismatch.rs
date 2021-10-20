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

use air_test_utils::prelude::*;

#[test]
fn mismatch_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_1"] value_2)
                )
                (xor
                    (mismatch value_1 value_2
                        (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_2");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2], expected_state);
}

#[test]
fn mismatch_not_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_2"] value_2)
                )
                (xor
                    (mismatch value_1 value_2
                        (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2], expected_state);
}

#[test]
fn mismatch_with_string() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{0}" ("" "") ["value_1"] value_1)
                (xor
                    (mismatch value_1 "value_1"
                        (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("result_2");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1], expected_state);
}

#[test]
fn mismatch_without_xor() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{0}" ("" "") ["value_1"] value_1)
                    (call "{0}" ("" "") ["value_1"] value_2)
                )
                (mismatch value_1 value_2
                    (call "{1}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                )
            )"#,
        set_variable_peer_id, local_peer_id
    );

    let result = call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = call_vm!(vm, "asd", &script, "", result.data);

    assert_eq!(result.ret_code, 1012);

    let result = call_vm!(vm, "asd", script, "", result.data);

    assert_eq!(result.ret_code, 1012);
}

#[test]
fn mismatch_with_two_xors() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(set_variable_call_service(serde_json::json!(false)), local_peer_id);

    let local_peer_id_2 = "local_peer_id_2";

    let script = format!(
        r#"
            (xor
                (seq
                    (seq
                        (call "{0}" ("getDataSrv" "condition") [] condition)
                        (call "{0}" ("getDataSrv" "relay") [] relay)
                    )
                    (xor
                        (mismatch condition true
                            (call "{1}" ("println" "print") ["it is false"])
                        )
                        (call "{0}" ("println" "print") ["it is true"])
                    )
                )
                (call "{0}" ("errorHandlingSrv" "error") [%last_error%])
            )
            "#,
        local_peer_id, local_peer_id_2
    );

    let result = checked_call_vm!(vm, "", script, "", "");

    let mut actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::request_sent_by(local_peer_id);

    assert_eq!(result.ret_code, 0);
    assert_eq!(actual_trace.pop().unwrap(), expected_executed_call_result);
}
