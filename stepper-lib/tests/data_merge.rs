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
use aqua_test_utils::set_variable_call_service;
use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::IValue;

use pretty_assertions::assert_eq;
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

    let res1 = call_vm!(vm1, "asd", script.clone(), "[]", "[]");
    let res2 = call_vm!(vm2, "asd", script.clone(), "[]", "[]");
    let res3 = call_vm!(vm1, "asd", script.clone(), res1.data.clone(), res2.data.clone());
    let res4 = call_vm!(vm2, "asd", script, res1.data.clone(), res2.data.clone());

    let resulted_json1: JValue = serde_json::from_str(&res1.data).expect("stepper should return valid json");

    let right_json1 = json!( [
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "request_sent": "A" } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "request_sent": "A" } },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "request_sent": "A" } },
    ]);

    assert_eq!(resulted_json1, right_json1);
    assert_eq!(res1.next_peer_pks, vec![String::from("B")]);

    let resulted_json2: JValue = serde_json::from_str(&res2.data).expect("stepper should return valid json");

    let right_json2 = json!( [
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "request_sent": "B" } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "request_sent": "B" } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "request_sent": "B" } },
    ]);

    assert_eq!(resulted_json2, right_json2);
    assert_eq!(res2.next_peer_pks, vec![String::from("A")]);

    let resulted_json3: JValue = serde_json::from_str(&res3.data).expect("stepper should return valid json");

    let right_json3 = json!( [
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "request_sent": "A" } },
    ]);

    assert_eq!(resulted_json3, right_json3);
    assert!(res3.next_peer_pks.is_empty());

    let resulted_json4: JValue = serde_json::from_str(&res4.data).expect("stepper should return valid json");

    let right_json4 = json!( [
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,2] },
        { "call": { "executed": ["A", "B"] } },
        { "par": [1,0] },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "executed": ["A", "B"] } },
        { "call": { "executed": ["A", "B"] } },
    ]);

    assert_eq!(resulted_json4, right_json4);
    assert!(res4.next_peer_pks.is_empty());
}

#[test]
fn acc_merge() {
    env_logger::init();

    let neighborhood_call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
        let args_count = match &args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let args_count = (args_count.as_bytes()[0] - b'0') as usize;

        let args_json = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let args: Vec<JValue> = serde_json::from_str(args_json).expect("valid json");
        let args = match &args[0] {
            JValue::Array(args) => args,
            _ => unreachable!(),
        };

        assert_eq!(args.len(), args_count);

        Some(IValue::Record(
            Vec1::new(vec![IValue::S32(0), IValue::String(json!(args).to_string())]).unwrap(),
        ))
    });

    let mut vm1 = create_aqua_vm(set_variable_call_service(r#""peer_id""#), "A");
    let mut vm2 = create_aqua_vm(neighborhood_call_service, "B");

    let script = String::from(
        r#"
        (seq (
            (call ("A" ("add_provider" "") () void[]))
                (seq (
                    (call ("A" ("add_provider" "") () void[]))
                    (seq (
                        (call ("A" ("get_providers" "") () providers[]))
                        (seq (
                            (call ("A" ("get_providers" "") () providers[]))
                            (seq (
                                (call ("B" ("" "2") (providers) void[]))
                                (call ("B" ("" "3") (void) void[]))
                            ))
                        ))
                    ))
                ))
            ))
        "#,
    );

    let res = call_vm!(vm1, "asd", script.clone(), "[]", "[]");
    call_vm!(vm2, "asd", script, "[]", res.data);
}
