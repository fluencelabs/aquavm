/*
 * Copyright 2021 Fluence Labs Limited
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

use air::CatchableError;
use air::ErrorObjectError;
use air::ExecutionError;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

#[tokio::test]
async fn fail_with_last_error() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id).await;

    let script = format!(
        r#"
            (xor
                (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_1)
                (fail %last_error%)
            )"#
    );

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!({
            "error_code": 10000i64,
            "instruction": r#"call "local_peer_id" ("service_id_1" "local_fn_name") [] result_1"#,
            "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
            "peer_id": "local_peer_id",
        })),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_error() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let mut vm = create_avm(fallible_call_service(fallible_service_id), local_peer_id).await;

    let script = format!(
        r#"
            (xor
                (call "{local_peer_id}" ("service_id_1" "local_fn_name") [] result_1)
                (fail :error:)
            )"#
    );

    let result = call_vm!(vm, <_>::default(), script, "", "");
    let err_message = r#""failed result from fallible_call_service""#.to_string();
    let expected_error = CatchableError::LocalServiceError(1i32, err_message.into());

    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_literals() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(echo_call_service(), local_peer_id).await;

    let script = r#"
            (xor
                (fail 1337 "error message")
                (fail %last_error%)
            )"#;

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let result = call_vm!(vm, test_params.clone(), script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!( {
        "error_code": 1337i64,
        "instruction": r#"fail 1337 "error message""#,
        "message": "error message",
        "peer_id": test_params.init_peer_id,
        })),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_last_error_tetraplets() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let (host_closure, tetraplet_anchor) = tetraplet_host_function(fallible_call_service(fallible_service_id));
    let mut vm = create_avm(host_closure, local_peer_id).await;

    let local_fn_name = "local_fn_name";
    let script = format!(
        r#"
        (xor
            (xor
                (call "{local_peer_id}" ("{fallible_service_id}" "{local_fn_name}") [] result_1)
                (fail %last_error%)
            )
            (call "{local_peer_id}" ("" "") [%last_error%])
        )
          "#
    );

    let test_params = TestRunParameters::from_init_peer_id(local_peer_id);
    let _ = checked_call_vm!(vm, test_params, script, "", "");
    assert_eq!(
        tetraplet_anchor.borrow()[0][0],
        SecurityTetraplet::new(local_peer_id, fallible_service_id, local_fn_name, "")
    );
}

#[tokio::test]
async fn fail_with_error_tetraplets() {
    let local_peer_id = "local_peer_id";
    let fallible_service_id = "service_id_1";
    let (host_closure, tetraplet_anchor) = tetraplet_host_function(fallible_call_service(fallible_service_id));
    let mut vm = create_avm(host_closure, local_peer_id).await;

    let local_fn_name = "local_fn_name";
    let script = format!(
        r#"
        (xor
            (xor
                (call "{local_peer_id}" ("{fallible_service_id}" "{local_fn_name}") [] result_1)
                (fail :error:)
            )
            (call "{local_peer_id}" ("" "") [%last_error%])
        )
          "#
    );

    let test_params = TestRunParameters::from_init_peer_id(local_peer_id);
    let _ = checked_call_vm!(vm, test_params, script, "", "");
    assert_eq!(
        tetraplet_anchor.borrow()[0][0],
        SecurityTetraplet::new(local_peer_id, fallible_service_id, local_fn_name, "")
    );
}

#[tokio::test]
async fn fail_with_literals_tetraplets() {
    let local_peer_id = "local_peer_id";
    let (host_closure, tetraplet_anchor) = tetraplet_host_function(echo_call_service());
    let mut vm = create_avm(host_closure, local_peer_id).await;

    let script = format!(
        r#"
            (xor
                (xor
                    (fail 1337 "error message")
                    (fail %last_error%)
                )
                (call "{local_peer_id}" ("" "") [%last_error%])
            )"#
    );

    let test_params = TestRunParameters::from_init_peer_id(local_peer_id);
    let _ = checked_call_vm!(vm, test_params, script, "", "");
    assert_eq!(
        tetraplet_anchor.borrow()[0][0],
        SecurityTetraplet::literal_tetraplet(local_peer_id)
    );
}

#[tokio::test]
async fn fail_with_canon_stream() {
    let vm_peer_id = "local_peer_id";
    let error_code = 1337i64;
    let error_message = "error message";
    let mut vm = create_avm(
        set_variable_call_service(json!({"error_code": error_code, "message": error_message})),
        vm_peer_id,
    ).await;

    let script = format!(
        r#"
            (seq
                (seq
                    (call "{vm_peer_id}" ("" "") [] $stream)
                    (canon "{vm_peer_id}" $stream #canon_stream)
                )
                (fail #canon_stream.$.[0])
            )"#
    );

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let result = call_vm!(vm, test_params, script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!( {
        "error_code": error_code,
        "message": error_message,
        })),
    };
    assert!(check_error(&result, expected_error));
}

async fn fail_to_fail_with_unsupported_errorcode(script: &str) {
    let local_peer_id = "local_peer_id";
    let script = script.to_string();

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let results = executor.execute_all(local_peer_id).await.unwrap();

    let expected_error = ExecutionError::Catchable(rc!(CatchableError::InvalidErrorObjectError(
        ErrorObjectError::ErrorCodeMustBeNonZero
    )));
    assert!(check_error(&results.last().unwrap(), expected_error));
}

#[tokio::test]
async fn fail_to_fail_with_unsupported_errorcode_in_scalar() {
    let script = r#"
        (seq
            (call "local_peer_id" ("m" "f1") [] scalar) ; ok = {"error_code": 0, "message": "some message"}
            (fail scalar)
        )
    "#;
    fail_to_fail_with_unsupported_errorcode(script);
}

#[tokio::test]
async fn fail_to_fail_with_unsupported_errorcode_in_scalar_wl() {
    let script = r#"
        (seq
            (call "local_peer_id" ("m" "f1") [] scalar) ; ok = {"key": {"error_code": 0, "message": "some message"} }
            (fail scalar.$.key)
        )
    "#;
    fail_to_fail_with_unsupported_errorcode(script);
}

#[tokio::test]
async fn fail_to_fail_with_unsupported_errorcode_in_canon() {
    let script = r#"
        (seq
            (call "local_peer_id" ("m" "f1") [] scalar) ; ok = [{"error_code": 0, "message": "some message"}]
            (fail scalar.$.[0])
        )
    "#;
    fail_to_fail_with_unsupported_errorcode(script);
}

#[tokio::test]
async fn fail_to_fail_with_unsupported_errorcode_in_error() {
    let script = r#"
        (fail :error:)
    "#;
    fail_to_fail_with_unsupported_errorcode(script);
}
