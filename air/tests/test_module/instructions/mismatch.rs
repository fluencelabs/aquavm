/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air::CatchableError;
use air::ToErrorCode;
use air_test_utils::prelude::*;

#[tokio::test]
async fn mismatch_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_2)
                )
                (xor
                    (mismatch value_1 value_2
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(
        "result_2",
        peer = local_peer_id,
        service = "service_id_2",
        function = "local_fn_name",
        args = ["result_2"]
    );

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2.into()], expected_state);
}

#[tokio::test]
async fn mismatch_not_equal() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_2"] value_2)
                )
                (xor
                    (mismatch value_1 value_2
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(
        "result_1",
        peer = local_peer_id,
        service = "service_id_2",
        function = "local_fn_name",
        args = ["result_1"]
    );

    assert_eq!(actual_trace.len(), 3);
    assert_eq!(actual_trace[2.into()], expected_state);
}

#[tokio::test]
async fn mismatch_with_string() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (seq
                (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                (xor
                    (mismatch value_1 "value_1"
                        (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                    )
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_2"] result_2)
                )
            )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(
        "result_2",
        peer = local_peer_id,
        service = "service_id_2",
        function = "local_fn_name",
        args = ["result_2"]
    );

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_state);
}

#[tokio::test]
async fn mismatch_without_xor() {
    let set_variable_peer_id = "set_variable_peer_id";
    let mut set_variable_vm = create_avm(echo_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_1)
                    (call "{set_variable_peer_id}" ("" "") ["value_1"] value_2)
                )
                (mismatch value_1 value_2
                    (call "{local_peer_id}" ("service_id_2" "local_fn_name") ["result_1"] result_1)
                )
            )"#
    );

    let result = call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = call_vm!(vm, <_>::default(), &script, "", result.data);

    let expected_error = CatchableError::MismatchValuesEqual;
    assert_eq!(expected_error.to_error_code(), 10002);
    assert!(check_error(&result, expected_error));

    let result = call_vm!(vm, <_>::default(), script, "", result.data);

    let expected_error = CatchableError::MismatchValuesEqual;
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn mismatch_with_two_xors() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(set_variable_call_service(serde_json::json!(false)), local_peer_id).await;

    let local_peer_id_2 = "local_peer_id_2";

    let script = format!(
        r#"
            (xor
                (seq
                    (seq
                        (call "{local_peer_id}" ("getDataSrv" "condition") [] condition)
                        (call "{local_peer_id}" ("getDataSrv" "relay") [] relay)
                    )
                    (xor
                        (mismatch condition true
                            (call "{local_peer_id_2}" ("println" "print") ["it is false"])
                        )
                        (call "{local_peer_id}" ("println" "print") ["it is true"])
                    )
                )
                (call "{local_peer_id}" ("errorHandlingSrv" "error") [%last_error%])
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let mut actual_trace = trace_from_result(&result);
    let expected_executed_call_result = executed_state::request_sent_by(local_peer_id);

    assert_eq!(actual_trace.pop().unwrap(), expected_executed_call_result);
}

#[tokio::test]
async fn mismatch_with_error() {
    use air::ExecutionCidState;

    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = format!(
        r#"
        (xor
            (mismatch 42 42 (null))
            (call "local_peer_id" ("test" "error_code") [:error:.$.error_code] scalar)
        )
    "#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);

    let mut cid_state = ExecutionCidState::new();
    let errcode_lambda_output = json!(10002);

    let expected_trace = ExecutionTrace::from(vec![scalar_tracked!(
        errcode_lambda_output.clone(),
        cid_state,
        peer = local_peer_id,
        service = "test",
        function = "error_code",
        args = vec![errcode_lambda_output]
    )]);
    assert_eq!(actual_trace, expected_trace);
}
