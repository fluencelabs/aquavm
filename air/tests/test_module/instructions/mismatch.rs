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

use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::echo_string_call_service;
use air_test_utils::executed_state;
use air_test_utils::ExecutionTrace;

#[test]
fn mismatch_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_string_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

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

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(vm, "asd", script, "", res.data);

    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
    let expected_state = executed_state::scalar_string("result_2");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2], expected_state);
}

#[test]
fn mismatch_not_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_string_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

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

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(vm, "asd", script, "", res.data);

    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
    let expected_state = executed_state::scalar_string("result_1");

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2], expected_state);
}

#[test]
fn mismatch_with_string() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_string_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

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

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(vm, "asd", script, "", res.data);

    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
    let expected_state = executed_state::scalar_string("result_2");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1], expected_state);
}

#[test]
fn mismatch_without_xor() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_string_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

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

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(vm, "asd", script.clone(), "", res.data);

    assert_eq!(res.ret_code, 1016);

    let res = call_vm!(vm, "asd", script, "", res.data);

    assert_eq!(res.ret_code, 1016);
}

#[test]
fn mismatch_with_two_xors() {
    use air_test_utils::set_variable_call_service;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(
        set_variable_call_service(serde_json::json!(false).to_string()),
        local_peer_id,
    );

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

    let res = call_vm!(vm, "", script, "", "");
    let mut trace: ExecutionTrace =
        serde_json::from_slice(&res.data).expect("the interpreter should provide correct trace");

    let expected_executed_call_result = executed_state::request_sent_by(local_peer_id);
    assert_eq!(res.ret_code, 0);
    assert_eq!(trace.pop_back().unwrap(), expected_executed_call_result);
}
