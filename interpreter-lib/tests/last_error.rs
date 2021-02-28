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

use aqua_test_utils::call_vm;
use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::fallible_call_service;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;
use interpreter_lib::SecurityTetraplet;

use std::cell::RefCell;
use std::rc::Rc;

type ArgToCheck<T> = Rc<RefCell<Option<T>>>;

fn create_check_service_closure(
    args_to_check: ArgToCheck<Vec<String>>,
    tetraplets_to_check: ArgToCheck<Vec<Vec<SecurityTetraplet>>>,
) -> CallServiceClosure {
    Box::new(move |_, args| -> Option<IValue> {
        let call_args = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let call_args: Vec<String> = serde_json::from_str(call_args).expect("json deserialization shouldn't fail");

        let tetraplets = match &args[3] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let de_tetraplets: Vec<Vec<SecurityTetraplet>> =
            serde_json::from_str(tetraplets).expect("json deserialization shouldn't fail");

        *args_to_check.borrow_mut() = Some(call_args);
        *tetraplets_to_check.borrow_mut() = Some(de_tetraplets);

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(tetraplets.clone())]).unwrap(),
        ))
    })
}

#[test]
fn last_error_tetraplets() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_aqua_vm(unit_call_service(), set_variable_peer_id);

    let fallible_peer_id = "fallible_peer_id";
    let mut fallible_vm = create_aqua_vm(fallible_call_service("fallible_call_service"), fallible_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_aqua_vm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = format!(
        include_str!("scripts/create_service_with_xor.clj"),
        set_variable_peer_id, fallible_peer_id, local_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(fallible_vm, "asd", script.clone(), "", res.data);
    let _ = call_vm!(local_vm, "asd", script, "", res.data);

    assert_eq!(
        (*args.borrow()).as_ref().unwrap()[0],
        r#"{"error":"Local service error: ret_code is 1, error message is 'error'","instruction":"call \"fallible_peer_id\" (\"fallible_call_service\" \"\") [service_id] client_result"}"#
    );

    let triplet = (*tetraplets.borrow()).as_ref().unwrap()[0][0].triplet.clone();
    assert_eq!(triplet.peer_pk, fallible_peer_id);
    assert_eq!(triplet.service_id, "fallible_call_service");
    assert_eq!(triplet.function_name, "");
    assert_eq!(&(*tetraplets.borrow()).as_ref().unwrap()[0][0].json_path, "");
}

#[test]
fn not_clear_last_error_in_match() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_aqua_vm(unit_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_aqua_vm(
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
                    (call "{1}" ("op" "identity") [])
                    (call "{1}" ("" "") [%last_error%])
                )
            )
        )
    "#,
        set_variable_peer_id, local_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", &script, "", "");
    let _ = call_vm!(local_vm, "asd", &script, "", res.data);

    assert_eq!((*args.borrow()).as_ref().unwrap()[0], "");
}

#[test]
fn not_clear_last_error_in_mismatch() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_aqua_vm(unit_call_service(), set_variable_peer_id);

    let local_peer_id = "local_peer_id";

    let args = Rc::new(RefCell::new(None));
    let tetraplets = Rc::new(RefCell::new(None));
    let mut local_vm = create_aqua_vm(
        create_check_service_closure(args.clone(), tetraplets.clone()),
        local_peer_id,
    );

    let script = format!(
        r#"
        (seq
            (call "{0}" ("" "") [] relayVariableName)
            (xor
                (mismatch relayVariableName "test"
                    (call "unknown_peer" ("" "") [%last_error%])
                )
                (seq
                    (call "{1}" ("op" "identity") [])
                    (call "{1}" ("" "") [%last_error%])
                )
            )
        )
    "#,
        set_variable_peer_id, local_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", &script, "", "");
    let _ = call_vm!(local_vm, "asd", &script, "", res.data);

    assert_eq!((*args.borrow()).as_ref().unwrap()[0], "");
}
