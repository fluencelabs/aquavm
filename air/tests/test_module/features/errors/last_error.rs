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

use air::CatchableError;
use air::ExecutionError;
use air::LambdaError;
use air::LastErrorObjectError;
use air::SecurityTetraplet;
use air_test_utils::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

type ArgToCheck<T> = Rc<RefCell<Option<T>>>;

fn create_check_service_closure(
    args_to_check: ArgToCheck<JValue>,
    tetraplets_to_check: ArgToCheck<Vec<Vec<SecurityTetraplet>>>,
) -> CallServiceClosure {
    Box::new(move |params| -> CallServiceResult {
        let mut call_args: Vec<JValue> =
            serde_json::from_value(JValue::Array(params.arguments)).expect("json deserialization shouldn't fail");

        let result = json!(params.tetraplets);
        *args_to_check.borrow_mut() = Some(call_args.remove(0));
        *tetraplets_to_check.borrow_mut() = Some(params.tetraplets);

        CallServiceResult::ok(result)
    })
}

#[test]
fn last_error_tetraplets() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id);

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = format!(
        include_str!("scripts/create_service_with_xor.clj"),
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
    assert_eq!(&(*tetraplets.borrow()).as_ref().unwrap()[0][0].json_path, "");
}

#[test]
fn not_clear_last_error_in_match() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = f!(r#"
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
    "#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value, JValue::Null);
}

#[test]
fn not_clear_last_error_in_mismatch() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(unit_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = f!(r#"
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
    "#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), &script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value, JValue::Null);
}

#[test]
fn track_current_peer_id() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = f!(r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (call "{local_peer_id}" ("" "") [%last_error%])
        )
    "#);

    let result = checked_call_vm!(fallible_vm, <_>::default(), &script, "", "");
    let _ = checked_call_vm!(local_vm, <_>::default(), script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    let last_error = actual_value.as_object().unwrap();
    assert_eq!(last_error.get("peer_id").unwrap(), fallible_peer_id);
}

#[test]
fn variable_names_shown_in_error() {
    let set_variable_vm_peer_id = "set_variable_vm_peer_id";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(1u32)), set_variable_vm_peer_id);

    let echo_vm_peer_id = "echo_vm_peer_id";
    let mut echo_vm = create_avm(echo_call_service(), echo_vm_peer_id);

    let script = f!(r#"
        (xor
            (seq
                (call "{set_variable_vm_peer_id}" ("" "") [""] -relay-)
                (call -relay- ("" "") [])
            )
            (call "{echo_vm_peer_id}" ("" "") [%last_error%.$.message])
        )
    "#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(echo_vm, <_>::default(), script, "", result.data);
    let trace = trace_from_result(&result);

    assert_eq!(
        trace.as_ref()[1],
        executed_state::scalar(json!(
            "expected JValue type 'string' for the variable `-relay-`, but got '1'"
        ))
    );
}

#[test]
fn non_initialized_last_error() {
    let vm_peer_id = "vm_peer_id";
    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        vm_peer_id,
    );

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [%last_error%])
            (null)
        )
    "#);

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let _ = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value, JValue::Null);

    let actual_tetraplets = (*tetraplets.borrow()).as_ref().unwrap().clone();
    assert_eq!(
        actual_tetraplets,
        vec![vec![SecurityTetraplet::new(test_params.init_peer_id, "", "", "")]]
    );
}

#[test]
fn access_last_error_by_not_exists_field() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id);

    let local_peer_id = "local_peer_id";

    let non_exists_field_name = "non_exists_field";
    let script = f!(r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (call "{local_peer_id}" ("" "") [%last_error%.$.{non_exists_field_name}])
        )
    "#);

    let result = call_vm!(fallible_vm, <_>::default(), &script, "", "");

    let expected_error = ExecutionError::Catchable(rc!(CatchableError::LambdaApplierError(
        LambdaError::ValueNotContainSuchField {
            value: json!({
                "error_code": 10000i64,
                "instruction": r#"call "fallible_peer_id" ("fallible_call_service" "") [""] "#,
                "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
                "peer_id": "fallible_peer_id",
            }),
            field_name: non_exists_field_name.to_string()
        }
    )));
    assert!(check_error(&result, expected_error));
}

#[test]
fn last_error_with_par_one_subgraph_failed() {
    let fallible_peer_id = "fallible_peer_id";
    let fallible_call_service_name = "fallible_call_service";
    let mut fallible_vm = create_avm(fallible_call_service(fallible_call_service_name), fallible_peer_id);

    let vm_peer_id = "local_peer_id";
    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut vm = create_avm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        vm_peer_id,
    );
    let script = f!(r#"
        (seq
            (par
                (call "{fallible_peer_id}" ("{fallible_call_service_name}" "") [""])
                (call "{fallible_peer_id}" ("non_fallible_call_service" "") [""])
            )
            (call "{vm_peer_id}" ("" "") [%last_error%])
        )
    "#);

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

#[test]
fn fail_with_scalar_rebubble_error() {
    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_avm(fallible_call_service("fallible_call_service"), fallible_peer_id);

    let script = f!(r#"
        (xor
            (call "{fallible_peer_id}" ("fallible_call_service" "") [""])
            (seq
                (ap %last_error% scalar)
                (fail scalar)
            )
        )
    "#);

    let result = call_vm!(fallible_vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!({
            "error_code": 10000i64,
            "instruction": r#"call "fallible_peer_id" ("fallible_call_service" "") [""] "#,
            "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
            "peer_id": "fallible_peer_id",
        })),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn fail_with_scalar_from_call() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let error_message = "error message";
    let service_result = json!({"error_code": error_code, "message": error_message});
    let mut vm = create_avm(set_variable_call_service(service_result), vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#);

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!({
            "error_code": error_code,
            "message": error_message,
        })),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn fail_with_scalar_with_lambda_from_call() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let error_message = "error message";
    let service_result = json!({"error": {"error_code": error_code, "message": error_message}});
    let mut vm = create_avm(set_variable_call_service(service_result), vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar.$.error)
        )
    "#);

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::UserError {
        error: rc!(json!({
            "error_code": error_code,
            "message": error_message,
        })),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn fail_with_scalar_from_call_not_enough_fields() {
    let vm_peer_id = "vm_peer_id";
    let error_code = 1337;
    let service_result = json!({ "error_code": error_code });
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#);

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::InvalidLastErrorObjectError(LastErrorObjectError::ScalarMustContainField {
        scalar: service_result,
        field_name: "message",
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn fail_with_scalar_from_call_not_right_type() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#);

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error =
        CatchableError::InvalidLastErrorObjectError(LastErrorObjectError::ScalarMustBeObject(service_result));
    assert!(check_error(&result, expected_error));
}

#[test]
fn fail_with_scalar_from_call_field_not_right_type() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!({"error_code": "error_code", "message": "error message"});
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = f!(r#"
        (seq
            (call "{vm_peer_id}" ("" "") [""] scalar)
            (fail scalar)
        )
    "#);

    let result = call_vm!(vm, <_>::default(), &script, "", "");

    let expected_error = CatchableError::InvalidLastErrorObjectError(LastErrorObjectError::ScalarFieldIsWrongType {
        scalar: service_result.clone(),
        field_name: "error_code",
        expected_type: "integer",
    });
    assert!(check_error(&result, expected_error));
}

#[test]
fn last_error_with_match() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(fallible_call_service("fallible_call_service"), vm_peer_id);

    let script = f!(r#"
        (xor
            (call "{vm_peer_id}" ("fallible_call_service" "") [""])
            (match %last_error%.$.error_code 10000
                (call "{vm_peer_id}" ("" "") [%last_error%])
            )
        )
    "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");

    let trace = trace_from_result(&result);
    assert_eq!(trace.len(), 2); // if match works there will be 2 calls in a resulted trace
}
