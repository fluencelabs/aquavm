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

use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::echo_number_call_service;
use aqua_test_utils::unit_call_service;
use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::IValue;

use serde_json::json;

type JValue = serde_json::Value;

#[test]
fn data_merge() {
    let neighborhood_call_service1: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(String::from("[\"A\", \"B\"]"))]).unwrap(),
        ))
    });

    let neighborhood_call_service2: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(String::from("[\"A\", \"B\"]"))]).unwrap(),
        ))
    });

    let mut vm1 = create_aqua_vm(neighborhood_call_service1, "A");
    let mut vm2 = create_aqua_vm(neighborhood_call_service2, "B");

    let script = String::from(
        r#"
        (seq (
            (call (%current_peer_id% ("neighborhood" "") () neighborhood))
            (seq (
                (seq (
                    (fold (neighborhood i
                        (par (
                            (call (i ("add_provider" "") () void[]))
                            (next i)
                        ))
                    ))
                    (fold (neighborhood i
                        (par (
                            (call (i ("get_providers" "") () providers[]))
                            (next i)
                        ))
                    ))
                ))
                (seq (
                    (call ("A" ("identity" "") () void[]))
                    (call ("B" ("" "") () none))
                ))
            ))
        ))
        "#,
    );

    let res1 = vm1
        .call(json!(["asd", script, "{}", "{}"]))
        .expect("should be successful");

    let res2 = vm2
        .call(json!(["asd", script, "{}", "{}"]))
        .expect("should be successful");

    let res3 = vm2
        .call(json!(["asd", script, res1.data, res2.data]))
        .expect("should be successful");

    let res4 = vm1
        .call(json!(["asd", script, res1.data, res2.data]))
        .expect("should be successful");

    let res5 = vm2
        .call(json!(["asd", script, res3.data, res4.data]))
        .expect("should be successful");

    let res6 = vm1
        .call(json!(["asd", script, res3.data, res4.data]))
        .expect("should be successful");

    let resulted_json3: JValue = serde_json::from_str(&res3.data).expect("stepper should return valid json");

    let right_json3 = json!( {
        "void": [["A", "B"]],
        "neighborhood": ["A", "B"],
        "providers": [["A", "B"]],
        "__call": [
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "request_sent" },
            { "par": [1,0] },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json3, right_json3);
    assert_eq!(res3.next_peer_pks, vec![String::from("A")]);

    let resulted_json4: JValue = serde_json::from_str(&res4.data).expect("stepper should return valid json");

    let right_json4 = json!( {
        "void": [["A", "B"]],
        "neighborhood": ["A", "B"],
        "providers": [["A", "B"]],
        "__call": [
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "request_sent" },
        ]
    });

    assert_eq!(resulted_json4, right_json4);
    assert_eq!(res4.next_peer_pks, vec![String::from("B")]);

    let resulted_json5: JValue = serde_json::from_str(&res5.data).expect("stepper should return valid json");

    let right_json5 = json!( {
        "void": [["A", "B"]],
        "neighborhood": ["A", "B"],
        "providers": [["A", "B"]],
        "__call": [
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "call": "request_sent" },
        ]
    });

    assert_eq!(resulted_json5, right_json5);
    assert_eq!(res5.next_peer_pks, vec![String::from("A")]);

    let resulted_json6: JValue = serde_json::from_str(&res6.data).expect("stepper should return valid json");

    let right_json6 = json!( {
        "void": [["A", "B"], ["A", "B"]],
        "neighborhood": ["A", "B"],
        "providers": [["A", "B"]],
        "__call": [
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "call": "executed" },
            { "call": "request_sent" }
        ]
    });

    assert_eq!(resulted_json6, right_json6);
    assert_eq!(res6.next_peer_pks, vec![String::from("B")]);
}
