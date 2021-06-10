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
use air_test_utils::fallible_call_service;
use air_test_utils::ExecutionTrace;

#[test]
fn xor() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = String::from("service_id_1");
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id);

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
    let expected_call_result = executed_state::scalar_string("res");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[0], executed_state::service_failed(1, "error"));
    assert_eq!(actual_trace[1], expected_call_result);

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
    assert_eq!(actual_trace[0], expected_call_result);
}

#[test]
fn xor_var_not_found() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

    let script = format!(
        r#"
            (xor
                (par
                    (call "unknown_peer" ("service_id_1" "local_fn_name") [] lazy_defined_variable)
                    (call "{0}" ("service_id_1" "local_fn_name") [lazy_defined_variable] result)
                )
                (call "{0}" ("service_id_2" "local_fn_name") ["expected"] result)
            )"#,
        local_peer_id,
    );

    let res = call_vm!(vm, "asd", script, "[]", "[]");
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
    assert_eq!(actual_trace[0], executed_state::par(1, 0));
    assert_eq!(actual_trace[1], executed_state::request_sent_by(local_peer_id));
}

#[test]
fn xor_multiple_variables_found() {
    let set_variables_peer_id = "set_variables_peer_id";
    let mut set_variables_vm = create_avm(echo_string_call_service(), set_variables_peer_id);

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

    let some_string = String::from("some_string");
    let expected_string = String::from("expected_string");
    let script = format!(
        r#"
            (seq
                (call "{0}" ("service_id_1" "local_fn_name") ["{2}"] result_1)
                (xor
                    (call "{1}" ("service_id_1" "local_fn_name") [""] result_1)
                    (call "{1}" ("service_id_2" "local_fn_name") ["{3}"] result_2)
                )
            )"#,
        set_variables_peer_id, local_peer_id, some_string, expected_string
    );

    let res = call_vm!(set_variables_vm, "asd", script.clone(), "[]", "[]");
    let res = call_vm!(vm, "asd", script, "[]", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");
    let some_string_call_result = executed_state::scalar_string(some_string);
    let expected_string_call_result = executed_state::scalar_string(expected_string);

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[0], some_string_call_result);
    assert_eq!(actual_trace[1], expected_string_call_result);
}

#[test]
fn xor_par() {
    use executed_state::*;

    let fallible_service_id = String::from("service_id_1");
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(fallible_call_service(fallible_service_id.clone()), local_peer_id);

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

    let expected_trace = vec![
        par(2, 2),
        scalar_string(&res),
        scalar_string(&res),
        par(1, 0),
        service_failed(1, "error"),
        scalar_string(&res),
        scalar_string(&res),
    ];

    assert_eq!(actual_trace, expected_trace);

    let result = call_vm!(vm, "asd", script, "[]", result.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&result.data).expect("should be valid json");
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn last_error_with_xor() {
    use air_test_utils::echo_string_call_service;

    let faillible_peer_id = "failible_peer_id";
    let mut faillible_vm = create_avm(fallible_call_service("service_id_1"), faillible_peer_id);
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_string_call_service(), local_peer_id);

    let script = format!(
        r#"
            (xor
                (call "{0}" ("service_id_1" "local_fn_name") [] result)
                (call "{1}" ("service_id_2" "local_fn_name") [%last_error%.$.msg] result)
            )"#,
        faillible_peer_id, local_peer_id,
    );

    let res = call_vm!(faillible_vm, "asd", script.clone(), "", "");
    let res = call_vm!(vm, "asd", script, "", res.data);
    let actual_trace: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be valid json");

    let expected_state = executed_state::scalar_string("Local service error, ret_code is 1, error message is 'error'");

    assert_eq!(actual_trace[1], expected_state);
}
