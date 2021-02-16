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
use stepper_lib::SecurityTetraplet;

fn create_check_service_closure() -> CallServiceClosure {
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

        assert_eq!(
            call_args[0],
            r#"{"error":"Local service error: ret_code is 1, error message is 'error'","instruction":"call \"failible_peer_id\" ("falliable_call_service" "") [service_id] client_result"}"#
        );

        let triplet = &de_tetraplets[0][0].triplet;
        assert_eq!(triplet.peer_pk, "failible_peer_id");
        assert_eq!(triplet.service_id, "failiable_call_service");
        assert_eq!(triplet.function_name, "");
        assert_eq!(de_tetraplets[0][0].json_path, "");

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(tetraplets.clone())]).unwrap(),
        ))
    })
}

#[test]
fn last_error_tetraplets() {
    let set_variable_peer_id = "set_variable";
    let mut set_variable_vm = create_aqua_vm(unit_call_service(), set_variable_peer_id);

    let faillible_peer_id = "failible_peer_id";
    let mut faillible_vm = create_aqua_vm(fallible_call_service("falliable_call_service"), faillible_peer_id);

    let local_peer_id = "local_peer_id";
    let mut local_vm = create_aqua_vm(create_check_service_closure(), local_peer_id);

    let script = format!(
        include_str!("scripts/create_service_with_xor.clj"),
        set_variable_peer_id, faillible_peer_id, local_peer_id
    );

    let res = call_vm!(set_variable_vm, "asd", script.clone(), "", "");
    let res = call_vm!(faillible_vm, "asd", script.clone(), "", res.data);

    // assert is done on the 'create_check_service_closure' call service closure
    let _ = call_vm!(local_vm, "asd", script, "", res.data);
}
