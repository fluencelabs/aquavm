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

use air::no_error_object;
use air::CatchableError;
use air::ErrorObjectError;
use air::ExecutionCidState;
use air::ExecutionError;
use air::LambdaError;
use air::SecurityTetraplet;
use air::NO_ERROR_ERROR_CODE;
use air::NO_ERROR_MESSAGE;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use futures::FutureExt;

use std::cell::RefCell;
use std::rc::Rc;

type ArgToCheck<T> = Rc<RefCell<Option<T>>>;

fn create_check_service_closure(
    args_to_check: ArgToCheck<serde_json::Value>,
    tetraplets_to_check: ArgToCheck<Vec<Vec<SecurityTetraplet>>>,
) -> CallServiceClosure<'static> {
    Box::new(move |params| {
        let args_to_check = args_to_check.clone();
        let tetraplets_to_check = tetraplets_to_check.clone();
        async move {
            let mut call_args: Vec<serde_json::Value> = params.arguments;

            let result = json!(params.tetraplets);
            *args_to_check.borrow_mut() = Some(call_args.remove(0));
            *tetraplets_to_check.borrow_mut() = Some(params.tetraplets);

            CallServiceResult::ok(result)
        }
        .boxed_local()
    })
}

#[tokio::test]
async fn last_error_tetraplets() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id).await;

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id).await;

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    )
    .await;

    let script = format!(
        include_str!("scripts/create_service_with_xor.air"),
        set_variable_peer_id, fallible_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(fallible_vm, <_>::default(), &script, "", result.data);
    let _ = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    let last_error = actual_value.as_object().unwrap();
    assert_eq!(
        last_error.get("instruction").unwrap(),
        r#"call "fallible_peer_id" ("fallible_call_service" "") [service_id] client_result"#
    );

    assert_eq!(
        last_error.get("message").unwrap(),
        r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#
    );

    let tetraplet = (*tetraplets.borrow()).as_ref().unwrap()[0][0].clone();
    assert_eq!(tetraplet.peer_pk, fallible_peer_id);
    assert_eq!(tetraplet.service_id, "fallible_call_service");
    assert_eq!(tetraplet.function_name, "");
    assert_eq!(&(*tetraplets.borrow()).as_ref().unwrap()[0][0].lens, "");
}

#[tokio::test]
async fn not_clear_last_error_in_match() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(create_check_service_closure(args.clone(), tetraplets), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") [] relayVariableName)
            (xor
                (match relayVariableName ""
                    (call "unknown_peer" ("" "") [%last_error%])
                )
                (seq
                    (call "{local_peer_id}" ("" "") [%last_error%])
                    (null)
                )
            )
        )
    "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);

    let actual_value: JValue = (*args.borrow()).as_ref().unwrap().clone().into();
    assert_eq!(actual_value, no_error_object());
}

#[tokio::test]
async fn not_clear_last_error_in_mismatch() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id).await;

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(create_check_service_closure(args.clone(), tetraplets), local_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{set_variable_peer_id}" ("" "") [] relayVariableName)
            (xor
                (mismatch relayVariableName "result from unit_call_service"
                    (call "unknown_peer" ("" "") [%last_error%])
                )
                (seq
                    (null)
                    (call "{local_peer_id}" ("" "") [%last_error%])
                )
            )
        )
    "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);

    let actual_value: JValue = (*args.borrow()).as_ref().unwrap().into();
    assert_eq!(actual_value, no_error_object());
}

#[tokio::test]
async fn track_current_peer_id() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id).await;

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(create_check_service_closure(args.clone(), tetraplets), local_peer_id).await;

    let script = format!(
        r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (call "{local_peer_id}" ("" "") [%last_error%])
        )
    "#
    );

    let result = checked_call_vm!(fallible_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);

    let actual_value: JValue = (*args.borrow()).as_ref().unwrap().into();
    let last_error = actual_value.as_object().unwrap();
    assert_eq!(last_error.get("peer_id").unwrap(), fallible_peer_id);
}

#[tokio::test]
async fn variable_names_shown_in_error() {
    let set_variable_vm_peer_id = "set_variable_vm_peer_id";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(1u32)), set_variable_vm_peer_id).await;

    let echo_vm_peer_id = "echo_vm_peer_id";
    let mut echo_vm = create_avm(echo_call_service(), echo_vm_peer_id).await;

    let script = format!(
        r#"
        (xor
            (seq
                (call "{set_variable_vm_peer_id}" ("" "") [""] -relay-)
                (call -relay- ("" "") [])
            )
            (call "{echo_vm_peer_id}" ("" "") [%last_error%.$.message])
        )
    "#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(echo_vm, <_>::default(), script, "", result.data);
    let trace = trace_from_result(&result);

    let msg = "call cannot resolve non-String triplet variable part `-relay-` with value '1'";
    assert_eq!(trace[1.into()], unused!(msg, peer = echo_vm_peer_id, args = vec![msg]));
}

#[tokio::test]
async fn non_initialized_last_error() {
    let vm_peer_id = "vm_peer_id";
    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        vm_peer_id,
    )
    .await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [%last_error%])
            (null)
        )
    "#
    );

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let _ = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_value: JValue = (*args.borrow()).as_ref().unwrap().into();
    assert_eq!(actual_value, no_error_object(),);

    let actual_tetraplets = (*tetraplets.borrow()).as_ref().unwrap().clone();
    assert_eq!(
        actual_tetraplets,
        vec![vec![SecurityTetraplet::new(test_params.init_peer_id, "", "", "")]]
    );
}

#[tokio::test]
async fn access_last_error_by_not_exists_field() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id).await;

    let local_peer_id = "local_peer_id";

    let non_exists_field_name = "non_exists_field";
    let script = format!(
        r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (call "{local_peer_id}" ("" "") [%last_error%.$.{non_exists_field_name}])
        )
    "#
    );

    let result = call_vm!(fallible_vm, <_>::default(), &script, "", "");

    let expected_error = ExecutionError::Catchable(rc!(CatchableError::LambdaApplierError(
        LambdaError::ValueNotContainSuchField {
            value: json!({
                "error_code": 10000i64,
                "instruction": r#"call "fallible_peer_id" ("fallible_call_service" "") [""] "#,
                "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
                "peer_id": "fallible_peer_id",
            }).into(),
            field_name: non_exists_field_name.to_string()
        }
    )));
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn last_error_with_par_one_subgraph_failed() {
    let fallible_peer_id = "fallible_peer_id";
    let fallible_call_service_name = "fallible_call_service";
    let mut fallible_vm = create_avm(fallible_call_service(fallible_call_service_name), fallible_peer_id).await;

    let vm_peer_id = "local_peer_id";
    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut vm = create_avm(create_check_service_closure(args.clone(), tetraplets), vm_peer_id).await;
    let script = format!(
        r#"
        (seq
            (par
                (call "{fallible_peer_id}" ("{fallible_call_service_name}" "") [""])
                (call "{fallible_peer_id}" ("non_fallible_call_service" "") [""])
            )
            (call "{vm_peer_id}" ("" "") [%last_error%])
        )
    "#
    );

    let result = checked_call_vm!(fallible_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    let expected_value = json!({
        "error_code": 10000i64,
        "instruction": r#"call "fallible_peer_id" ("fallible_call_service" "") [""] "#,
        "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
        "peer_id": fallible_peer_id
    });
    assert_eq!(actual_value, expected_value);
}

#[tokio::test]
async fn fail_with_scalar_rebubble_error() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id).await;

    let script = format!(
        r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (seq
                (ap %last_error% scalar)
                (fail scalar)
            )
        )
    "#
    );

    let result = call_vm!(fallible_vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: json!({
            "error_code": 10000i64,
            "instruction": r#"call "fallible_peer_id" ("fallible_call_service" "") [""] "#,
            "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
            "peer_id": "fallible_peer_id",
        }).into(),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_scalar_from_call() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let error_message = "error message";
    let service_result = json!({"error_code": error_code, "message": error_message});
    let mut vm = create_avm(set_variable_call_service(service_result), vm_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#
    );

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: json!({
            "error_code": error_code,
            "message": error_message,
        })
        .into(),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_scalar_with_lambda_from_call() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let error_message = "error message";
    let service_result = json!({"error": {"error_code": error_code, "message": error_message}});
    let mut vm = create_avm(set_variable_call_service(service_result), vm_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar.$.error)
        )
    "#
    );

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: json!({
            "error_code": error_code,
            "message": error_message,
        })
        .into(),
    };
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_scalar_from_call_not_enough_fields() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let service_result = json!({ "error_code": error_code });
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#
    );

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::InvalidErrorObjectError(ErrorObjectError::ScalarMustContainField {
        scalar: service_result.into(),
        field_name: "message",
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_scalar_from_call_not_right_type() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#
    );

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error =
        CatchableError::InvalidErrorObjectError(ErrorObjectError::ScalarMustBeObject(service_result.into()));
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn fail_with_scalar_from_call_field_not_right_type() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!({"error_code": "error_code", "message": "error message"});
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id).await;

    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#
    );

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::InvalidErrorObjectError(ErrorObjectError::ScalarFieldIsWrongType {
        scalar: service_result.into(),
        field_name: "error_code",
        expected_type: "integer",
    });
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn last_error_with_match() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(fallible_call_service("fallible_call_service"), vm_peer_id).await;

    let script = format!(
        r#"
        (xor
            (call "{vm_peer_id}" ("fallible_call_service" "") [""])
            (match %last_error%.$.error_code 10000
                (call "{vm_peer_id}" ("" "") [%last_error%])
            )
        )
    "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");

    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2); // if match works there will be 2 calls in a resulted trace
}

#[tokio::test]
async fn undefined_last_error_errcode() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
        (call "{local_peer_id}" ("test" "error_code") [%last_error%.$.error_code] scalar) ; behaviour = echo
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let actual_trace = trace_from_result(&result.last().unwrap());
    let mut cid_state = ExecutionCidState::new();
    let errcode_lambda_output = json!(NO_ERROR_ERROR_CODE);

    let expected_trace = ExecutionTrace::from(vec![scalar_tracked!(
        errcode_lambda_output.clone(),
        cid_state,
        peer_name = local_peer_id,
        service = "test..0",
        function = "error_code",
        args = vec![errcode_lambda_output]
    )]);
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn undefined_last_error_msg_errcode() {
    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
        (call "{local_peer_id}" ("test" "message") [%last_error%.$.message] scalar1) ; behaviour = echo
    "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(local_peer_id), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(local_peer_id).await.unwrap();

    let actual_trace = trace_from_result(&result.last().unwrap());
    let mut cid_state = ExecutionCidState::new();
    let message_lambda_output = json!(NO_ERROR_MESSAGE);

    let expected_trace = ExecutionTrace::from(vec![scalar_tracked!(
        message_lambda_output.clone(),
        cid_state,
        peer_name = local_peer_id,
        service = "test..0",
        function = "message",
        args = vec![message_lambda_output]
    )]);
    assert_eq!(actual_trace, expected_trace);
}
