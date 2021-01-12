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
use aqua_test_utils::echo_number_call_service;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;

use serde_json::json;

use std::rc::Rc;

type JValue = serde_json::Value;

#[test]
fn evidence_seq_par_call() {
    use stepper_lib::CallResult::*;
    use stepper_lib::EvidenceState::{self, *};

    let local_peer_id = "local_peer_id";
    let mut vm = create_aqua_vm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
        (seq
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        local_peer_id
    );

    let initial_state = json!([
        { "par": [1,1] },
        { "call": {"executed": "test"} },
        { "call": {"executed": "test"} },
    ])
    .to_string();

    let res = call_vm!(vm, "asd", script, "[]", initial_state);
    let resulted_path: Vec<EvidenceState> =
        serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_path = vec![
        Par(1, 1),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string)))),
    ];

    assert_eq!(resulted_path, expected_path);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_par_par_call() {
    use stepper_lib::CallResult::*;
    use stepper_lib::EvidenceState::{self, *};

    let local_peer_id = "local_peer_id";
    let mut vm = create_aqua_vm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
        (par
            (par
                (call "local_peer_id" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        local_peer_id,
    );

    let initial_state = json!([
        { "par": [3,0] },
        { "par": [1,0] },
        { "call": {"request_sent": "peer_id_1"} },
        { "call": {"executed": "test"} },
    ])
    .to_string();

    let res = call_vm!(vm, "asd", script, "[]", initial_state);
    let resulted_path: Vec<EvidenceState> =
        serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_path = vec![
        Par(3, 1),
        Par(1, 1),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(RequestSent(local_peer_id.to_string())),
        Call(Executed(Rc::new(JValue::String(test_string)))),
    ];

    assert_eq!(resulted_path, expected_path);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn evidence_seq_seq() {
    use stepper_lib::CallResult::*;
    use stepper_lib::EvidenceState::{self, *};

    let peer_id_1 = String::from("12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er");
    let peer_id_2 = String::from("12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR");
    let mut vm1 = create_aqua_vm(unit_call_service(), peer_id_1.clone());
    let mut vm2 = create_aqua_vm(unit_call_service(), peer_id_2.clone());

    let script = format!(
        r#"
        (seq
            (call "{}" ("identity" "") [] void0)
            (seq
                (call "{}" ("add_blueprint" "") [] blueprint_id)
                (call "{}" ("addBlueprint-14d8488e-d10d-474d-96b2-878f6a7d74c8" "") [] void1)
            )
        )
        "#,
        peer_id_1, peer_id_1, peer_id_2
    );

    let res = call_vm!(vm2, "asd", script.clone(), "[]", "[]");
    assert_eq!(res.next_peer_pks, vec![peer_id_1.clone()]);

    let res = call_vm!(vm1, "asd", script.clone(), "[]", res.data);
    assert_eq!(res.next_peer_pks, vec![peer_id_2.clone()]);

    let res = call_vm!(vm2, "asd", script, "[]", res.data);

    let resulted_path: Vec<EvidenceState> =
        serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_path = vec![
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(Executed(Rc::new(JValue::String(test_string)))),
    ];

    assert_eq!(resulted_path, expected_path);
}

#[test]
fn evidence_create_service() {
    use stepper_lib::CallResult::*;
    use stepper_lib::EvidenceState::{self, *};

    let module = "greeting";
    let module_config = json!(
        {
            "name": module,
            "mem_pages_count": 100,
            "logger_enabled": true,
            "wasi": {
                "envs": json!({}),
                "preopened_files": vec!["/tmp"],
                "mapped_dirs": json!({}),
            }
        }
    );

    let module_bytes = json!([1, 2]);
    let blueprint = json!({ "name": "blueprint", "dependencies": [module]});

    let add_module_response = String::from("add_module response");
    let add_blueprint_response = String::from("add_blueprint response");
    let create_response = String::from("create response");

    let call_service: CallServiceClosure = Box::new(move |_, args| -> Option<IValue> {
        let builtin_service = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let response = match builtin_service.as_str() {
            "add_module" => add_module_response.clone(),
            "add_blueprint" => add_blueprint_response.clone(),
            "create" => create_response.clone(),
            _ => String::from("unknown response"),
        };

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(format!("\"{}\"", response))]).unwrap(),
        ))
    });

    let mut vm = create_aqua_vm(call_service, "A");

    let script = include_str!("./scripts/create_service.clj");

    let add_module_response = String::from("add_module response");
    let add_blueprint_response = String::from("add_blueprint response");
    let create_response = String::from("create response");
    let path = vec![
        Call(Executed(Rc::new(module_bytes))),
        Call(Executed(Rc::new(module_config))),
        Call(Executed(Rc::new(blueprint))),
        Call(Executed(Rc::new(JValue::String(add_module_response)))),
        Call(Executed(Rc::new(JValue::String(add_blueprint_response)))),
        Call(Executed(Rc::new(JValue::String(create_response)))),
        Call(Executed(Rc::new(JValue::String(String::from("test"))))),
    ];

    let res = call_vm!(vm, "init_peer_id", script, "[]", json!(path).to_string());

    let resulted_path: Vec<EvidenceState> = serde_json::from_slice(&res.data).expect("should be a correct json");

    assert_eq!(resulted_path, path);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_par_seq_fold_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(String::from(
                    "[\"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\", \"10\"]",
                )),
            ])
            .unwrap(),
        ))
    });

    let mut vm1 = create_aqua_vm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_aqua_vm(echo_number_call_service(), "some_peer_id_2");
    let mut vm3 = create_aqua_vm(unit_call_service(), "some_peer_id_3");

    let script = String::from(
        r#"
        (par
            (seq
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] acc[])
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
    );

    let res = call_vm!(vm2, "asd", script.clone(), "[]", "[]");
    let res = call_vm!(vm1, "asd", script.clone(), "[]", res.data);
    let mut data = res.data;

    for _ in 0..100 {
        let res = call_vm!(vm2, "asd", script.clone(), "[]", data);
        data = res.data;
    }

    let res = call_vm!(vm3, "asd", script, "[]", data);
    let resulted_path: JValue = serde_json::from_slice(&res.data).expect("a valid json");

    let expected_json = json!( [
        { "par": [21,1] },
        { "call": { "executed": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"] } },
        { "par": [1,18] },
        { "call": { "executed": 1 } },
        { "par": [1,16] },
        { "call": { "executed": 2 } },
        { "par": [1,14] },
        { "call": { "executed": 3 } },
        { "par": [1,12] },
        { "call": { "executed": 4 } },
        { "par": [1,10] },
        { "call": { "executed": 5 } },
        { "par": [1,8] },
        { "call": { "executed": 6 } },
        { "par": [1,6] },
        { "call": { "executed": 7 } },
        { "par": [1,4] },
        { "call": { "executed": 8 } },
        { "par": [1,2] },
        { "call": { "executed": 9 } },
        { "par": [1,0] },
        { "call": { "executed": 10 } },
        { "call": { "executed": "test" } },
    ]);

    assert_eq!(resulted_path, expected_json);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_par_seq_fold_in_cycle_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(String::from(
                    "[\"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\", \"10\"]",
                )),
            ])
            .unwrap(),
        ))
    });

    let mut vm1 = create_aqua_vm(return_numbers_call_service, "some_peer_id_1");
    let mut vm2 = create_aqua_vm(echo_number_call_service(), "some_peer_id_2");
    let mut vm3 = create_aqua_vm(unit_call_service(), "some_peer_id_3");

    let script = String::from(
        r#"
        (par 
            (seq 
                (call "some_peer_id_1" ("local_service_id" "local_fn_name") [] IterableResultPeer1)
                (fold IterableResultPeer1 i
                    (par 
                        (call "some_peer_id_2" ("local_service_id" "local_fn_name") [i] acc[])
                        (next i)
                    )
                )
            )
            (call "some_peer_id_3" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
    );

    let mut data = vec![];

    for _ in 0..100 {
        let res = call_vm!(vm1, "asd", script.clone(), "[]", data);
        let res = call_vm!(vm2, "asd", script.clone(), "[]", res.data);
        let res = call_vm!(vm3, "asd", script.clone(), "[]", res.data);
        data = res.data;
    }

    let resulted_json: JValue = serde_json::from_slice(&data).expect("stepper should return valid json");

    let expected_json = json!( [
        { "par": [21,1] },
        { "call": { "executed": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"] } },
        { "par": [1,18] },
        { "call": { "executed": 1 } },
        { "par": [1,16] },
        { "call": { "executed": 2 } },
        { "par": [1,14] },
        { "call": { "executed": 3 } },
        { "par": [1,12] },
        { "call": { "executed": 4 } },
        { "par": [1,10] },
        { "call": { "executed": 5 } },
        { "par": [1,8] },
        { "call": { "executed": 6 } },
        { "par": [1,6] },
        { "call": { "executed": 7 } },
        { "par": [1,4] },
        { "call": { "executed": 8 } },
        { "par": [1,2] },
        { "call": { "executed": 9 } },
        { "par": [1,0] },
        { "call": { "executed": 10 } },
        { "call": { "executed": "test" } },
    ]);

    assert_eq!(resulted_json, expected_json);
}

#[test]
fn evidence_seq_par_seq_seq() {
    let peer_id_1 = String::from("12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er");
    let peer_id_2 = String::from("12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR");
    let mut vm1 = create_aqua_vm(unit_call_service(), peer_id_1.clone());
    let mut vm2 = create_aqua_vm(unit_call_service(), peer_id_2.clone());
    let script = format!(
        r#"
        (seq 
            (par 
                (seq 
                    (call "{}" ("" "") [] result_1)
                    (call "{}" ("" "") [] result_2)
                )
                (seq 
                    (call "{}" ("" "") [] result_3)
                    (call "{}" ("" "") [] result_4)
                )
            )
            (call "{}" ("" "") [] result_5)
        )
        "#,
        peer_id_1, peer_id_2, peer_id_2, peer_id_1, peer_id_2
    );

    let res = call_vm!(vm2, "asd", script.clone(), "[]", "[]");
    assert_eq!(res.next_peer_pks, vec![peer_id_1.clone()]);

    let res = call_vm!(vm1, "asd", script.clone(), "[]", res.data);
    assert_eq!(res.next_peer_pks, vec![peer_id_2.clone()]);

    let res = call_vm!(vm2, "asd", script, "[]", res.data);

    let resulted_json: JValue = serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let expected_json = json!( [
        { "par": [2,2] },
        { "call": {"executed" : "test" } },
        { "call": {"executed" : "test" } },
        { "call": {"executed" : "test" } },
        { "call": {"executed" : "test" } },
        { "call": {"executed" : "test" } },
    ]);

    assert_eq!(resulted_json, expected_json);
    assert!(res.next_peer_pks.is_empty());
}
