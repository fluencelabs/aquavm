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

use air::UncatchableError;
use air_test_utils::prelude::*;

#[test]
fn xor() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id);

    let script = format!(
        r#"
            (xor
                (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_1)
                (call "{local_peer_id}" ("service_id_2" "local_fn_name") [] result_2)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_call_result = scalar!(
        "success result from fallible_call_service",
        peer = local_peer_id,
        service = "service_id_2",
        function = "local_fn_name"
    );

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(
        actual_trace[0.into()],
        failed!(
            1,
            "failed result from fallible_call_service",
            peer = local_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        )
    );
    assert_eq!(actual_trace[1.into()], expected_call_result);

    let script = format!(
        r#"
            (xor
                (call "{local_peer_id}" ("service_id_2" "local_fn_name") [] result_1)
                (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_2)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0.into()], expected_call_result);
}

#[test]
fn xor_var_not_found() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (xor
                (par
                    (call "unknown_peer" ("service_id_1" "local_fn_name") [] lazy_defined_variable)
                    (call "{local_peer_id}" ("service_id_1" "local_fn_name") [lazy_defined_variable] result)
                )
                (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["expected"] result)
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace[0.into()], executed_state::par(1, 0));
    assert_eq!(actual_trace[1.into()], executed_state::request_sent_by(local_peer_id));
}

#[test]
fn xor_multiple_variables_found() {
    let set_variables_peer_id = "set_variables_peer_id";
    let mut set_variables_vm = create_avm(echo_call_service(), set_variables_peer_id);

    let local_peer_id = "local_peer_id";
    let some_string = "some_string";
    let expected_string = "expected_string";
    let variable_name = "result_1";
    let script = format!(
        r#"
            (seq
                (call "{set_variables_peer_id}" ("service_id_1" "local_fn_name") ["{some_string}"] {variable_name})
                (xor
                    (call "{local_peer_id}" ("service_id_1" "local_fn_name") [""] {variable_name})
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["{expected_string}"] result_2)
                )
            )"#
    );

    let result = call_vm!(set_variables_vm, <_>::default(), &script, "", "");

    let expected_error = UncatchableError::ShadowingIsNotAllowed(variable_name.to_string());
    assert!(check_error(&result, expected_error));
}

#[test]
fn xor_par() {
    use executed_state::*;

    let fallible_service_id = String::from("service_id_1");
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id);

    let script = format!(
        r#"
            (xor
                (par
                    (par
                        (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_1)
                        (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_2)
                    )
                    (par
                        (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_3)
                        (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_4)
                    )
                )
                (seq
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") [] result_4)
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") [] result_5)
                )
            )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let success_result = "success result from fallible_call_service";
    let failed_result = "failed result from fallible_call_service";
    let expected_trace = vec![
        par(3, 3),
        par(1, 1),
        failed!(
            1,
            failed_result,
            peer = local_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        ),
        failed!(
            1,
            failed_result,
            peer = local_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        ),
        par(1, 1),
        failed!(
            1,
            failed_result,
            peer = local_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        ),
        failed!(
            1,
            failed_result,
            peer = local_peer_id,
            service = "service_id_1",
            function = "local_fn_name"
        ),
        scalar!(
            success_result,
            peer = local_peer_id,
            service = "service_id_2",
            function = "local_fn_name"
        ),
        scalar!(
            success_result,
            peer = local_peer_id,
            service = "service_id_2",
            function = "local_fn_name"
        ),
    ];

    assert_eq!(actual_trace, expected_trace);

    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn last_error_with_xor() {
    let faillible_peer_id = "failible_peer_id";
    let mut faillible_vm = create_avm(fallible_call_service("service_id_1"), faillible_peer_id);
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id);

    let script = format!(
        r#"
            (xor
                (call "{faillible_peer_id}" ("service_id_1" "local_fn_name") [] result)
                (call "{local_peer_id}" ("service_id_2" "local_fn_name") [%last_error%.$.message] result)
            )"#
    );

    let result = checked_call_vm!(faillible_vm, <_>::default(), script.clone(), "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let msg = r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#;
    let expected_state = scalar!(
        msg,
        peer = local_peer_id,
        service = "service_id_2",
        function = "local_fn_name",
        args = [msg]
    );

    assert_eq!(actual_trace[1.into()], expected_state);
}
