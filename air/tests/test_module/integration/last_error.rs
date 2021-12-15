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

use air::LastError;
use air::SecurityTetraplet;
use air_test_utils::prelude::*;

use fstrings::f;
use fstrings::format_args_f;

use std::cell::RefCell;
use std::rc::Rc;

type ArgToCheck<T> = Rc<RefCell<Option<T>>>;

fn create_check_service_closure(
    args_to_check: ArgToCheck<LastError>,
    tetraplets_to_check: ArgToCheck<Vec<Vec<SecurityTetraplet>>>,
) -> CallServiceClosure {
    Box::new(move |params| -> CallServiceResult {
        let mut call_args: Vec<LastError> =
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

    let result = checked_call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let result = checked_call_vm!(fallible_vm, "asd", script.clone(), "", result.data);
    let _ = checked_call_vm!(local_vm, "asd", script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(
        actual_value.instruction,
        r#"call "fallible_peer_id" ("fallible_call_service" "") [service_id] client_result"#
    );

    assert_eq!(
        actual_value.msg,
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

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] relayVariableName)
            (xor
                (match relayVariableName ""
                    (call "unknown_peer" ("" "") [%last_error%])
                )
                (seq
                    (null)
                    (call "{1}" ("" "") [%last_error%])
                )
            )
        )
    "#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let _ = checked_call_vm!(local_vm, "asd", &script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value.instruction, "");
    assert_eq!(actual_value.msg, "");
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

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] relayVariableName)
            (xor
                (mismatch relayVariableName "result from unit_call_service"
                    (call "unknown_peer" ("" "") [%last_error%])
                )
                (seq
                    (null)
                    (call "{1}" ("" "") [%last_error%])
                )
            )
        )
    "#,
        set_variable_peer_id, local_peer_id
    );

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let _ = checked_call_vm!(local_vm, "asd", &script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value.instruction, "");
    assert_eq!(actual_value.msg, "");
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

    let script = format!(
        r#"
        (xor
            (call "{0}" ("fallible_call_service" "") [""])
            (call "{1}" ("" "") [%last_error%])
        )
    "#,
        fallible_peer_id, local_peer_id
    );

    let result = checked_call_vm!(fallible_vm, "asd", &script, "", "");
    let _ = checked_call_vm!(local_vm, "asd", script, "", result.data);

    let actual_value = (*args.borrow()).as_ref().unwrap().clone();
    assert_eq!(actual_value.peer_id, fallible_peer_id);
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
            (call "{echo_vm_peer_id}" ("" "") [%last_error%.$.msg])
        )
    "#);

    let result = checked_call_vm!(set_variable_vm, "", &script, "", "");
    let result = checked_call_vm!(echo_vm, "", script, "", result.data);
    let trace = trace_from_result(&result);

    assert_eq!(
        trace[1],
        executed_state::scalar(json!(
            "expected JValue type 'string' for the variable `-relay-`, but got '1'"
        ))
    );
}
