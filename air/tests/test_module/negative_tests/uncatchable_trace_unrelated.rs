/*
 * Copyright 2023 Fluence Labs Limited
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

use air::UncatchableError::*;
use air_interpreter_cid::value_to_json_cid;
use air_interpreter_data::ValueRef;
use air_test_utils::prelude::*;

#[test]
fn fold_state_not_found() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = f!(r#"
        (seq
            (seq
                (call "vm_peer_id_1" ("" "") [] some)
                (fold some i
                    (next i)
                )
            )
            (next i)
        )
    "#);

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = FoldStateNotFound(String::from("i"));
    assert!(check_error(&result, expected_error));
}

#[test]
fn iterable_shadowing() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = f!(r#"
        (seq
            (call "vm_peer_id_1" ("" "") [] some)
            (fold some i
                (call "vm_peer_id_1" ("" "") [] i)
            )
        )
    "#);

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = IterableShadowing(String::from("i"));
    assert!(check_error(&result, expected_error));
}

#[test]
fn call_result_not_correspond_to_instr() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);

    let script = f!(r#"
        (call "vm_peer_id_1" ("" "") [] $some)
        "#);

    let scalar_value = 42;
    let wrong_trace = vec![scalar_number(scalar_value)];
    let data = raw_data_from_trace(wrong_trace, <_>::default());

    let result = peer_vm_1.call(script, "", data, <_>::default()).unwrap();
    let value_ref = ValueRef::Scalar(value_to_json_cid(&json!(scalar_value)).unwrap().into());
    let expected_error = CallResultNotCorrespondToInstr(value_ref);
    assert!(check_error(&result, expected_error));
}

#[test]
fn shadowing_is_not_allowed() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let var_name = String::from("some");
    let script = f!(r#"
    (seq
        (ap 42 {var_name})
        (ap 42 {var_name})
    )
    "#);

    let result = peer_vm_1.call(script, "", "", <_>::default()).unwrap();
    let expected_error = ShadowingIsNotAllowed(var_name);
    assert!(check_error(&result, expected_error));
}

#[test]
fn value_for_cid_not_found() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([1, 2,]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);

    let script = f!(r#"
        (call "vm_peer_id_1" ("" "") [] some)
    "#);

    let wrong_trace = vec![scalar_number(42)];
    let data = raw_data_from_trace(wrong_trace, <_>::default());
    let result = peer_vm_1.call(script, "", data, <_>::default()).unwrap();
    println!("{:?}", result);
    let missing_cid = String::from("bagaaieraondvznakk2hi3kfaixhnceatpykz7cikytniqo3lc7ogkgz2qbeq");
    let expected_error = ValueForCidNotFound("value", missing_cid);
    assert!(check_error(&result, expected_error));
}
