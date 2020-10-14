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
fn evidence_seq_par_call() {
    let mut vm = create_aqua_vm(unit_call_service(), "");

    let script = String::from(
        r#"
        (seq (
            (par (
                (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_1))
                (call ("remote_peer_id" ("service_id" "fn_name") () g))
            ))
            (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_2))
        ))"#,
    );

    let res = vm
        .call(json!([
            String::from("asd"),
            script,
            json!({
                "__call": [
                    { "par": [1,1] },
                    { "call": "executed" },
                    { "call": "executed" },
                ]
            })
            .to_string(),
        ]))
        .expect("should be successful");

    let resulted_json: JValue =
        serde_json::from_str(&res.data).expect("stepper should return valid json");

    let right_json = json!( {
        "result_2": "test",
        "__call": [
            { "par": [1,1] },
            { "call": "executed" },
            { "call": "executed" },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_par_par_call() {
    let mut vm = create_aqua_vm(unit_call_service(), "some_peer_id");

    let script = String::from(
        r#"
        (par (
            (par (
                (call ("some_peer_id" ("local_service_id" "local_fn_name") () result_1))
                (call ("remote_peer_id" ("service_id" "fn_name") () g))
            ))
            (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_2))
        ))"#,
    );

    let res = vm
        .call(json!([
            String::from("asd"),
            script,
            json!({
                "__call": [
                    { "par": [3,0] },
                    { "par": [1,1] },
                    { "call": "request_sent" },
                    { "call": "executed" },
                ]
            })
            .to_string(),
        ]))
        .expect("should be successful");

    let resulted_json: JValue =
        serde_json::from_str(&res.data).expect("stepper should return valid json");

    let right_json = json!( {
        "result_1" : "test",
        "result_2" : "test",
        "__call": [
            { "par": [3,1] },
            { "par": [1,1] },
            { "call": "executed" },
            { "call": "executed" },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_seq_seq() {
    let peer_id_1 = String::from("12D3KooWHk9BjDQBUqnavciRPhAYFvqKBe4ZiPPvde7vDaqgn5er");
    let peer_id_2 = String::from("12D3KooWAzJcYitiZrerycVB4Wryrx22CFKdDGx7c4u31PFdfTbR");
    let mut vm1 = create_aqua_vm(unit_call_service(), peer_id_1.clone());
    let mut vm2 = create_aqua_vm(unit_call_service(), peer_id_2.clone());

    let script = format!(
        r#"
        (seq (
            (call ("{}" ("identity" "") () void0))
            (seq (
                (call ("{}" ("add_blueprint" "") () blueprint_id))
                (call ("{}" ("addBlueprint-14d8488e-d10d-474d-96b2-878f6a7d74c8" "") () void1))
            ))
        ))
        "#,
        peer_id_1, peer_id_1, peer_id_2
    );

    let res1 = vm2
        .call(json!([String::from("asd"), script, String::from("{}")]))
        .expect("should be successful");

    assert_eq!(res1.next_peer_pks, vec![peer_id_1.clone()]);

    let res2 = vm1
        .call(json!([String::from("asd"), script, res1.data]))
        .expect("should be successful");

    assert_eq!(res2.next_peer_pks, vec![peer_id_2.clone()]);

    let res3 = vm2
        .call(json!([String::from("asd"), script, res2.data]))
        .expect("should be successful");

    let resulted_json: JValue =
        serde_json::from_str(&res3.data).expect("stepper should return valid json");

    let right_json = json!( {
        "void0": "test",
        "void1": "test",
        "blueprint_id": "test",
        "__call": [
            { "call": "executed" },
            { "call": "executed" },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert!(res3.next_peer_pks.is_empty());
}

#[test]
fn evidence_create_service() {
    let module = "greeting";
    let config = json!(
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
    let mut data_value = json!({
        "module_bytes": vec![1,2],
        "module_config": config,
        "blueprint": { "name": "blueprint", "dependencies": [module] },
        "__call": [
            { "call": "executed" },
            { "call": "executed" },
            { "call": "executed" },
            { "call": "executed" },
        ]
    });
    let data = data_value.to_string();

    let script = String::from(
        r#"(seq (
            (call (%current_peer_id% ("add_module" "") (module_bytes module_config) module))
            (seq (
                (call (%current_peer_id% ("add_blueprint" "") (blueprint) blueprint_id))
                (seq (
                    (call (%current_peer_id% ("create" "") (blueprint_id) service_id))
                    (call ("remote_peer_id" ("" "") (service_id) client_result))
                ))
            ))
        ))"#,
    );

    let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
        let builtin_service = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let response = match builtin_service.as_str() {
            "add_module" => String::from("add_module response"),
            "add_blueprint" => String::from("add_blueprint response"),
            "create" => String::from("create response"),
            _ => String::from("unknown response"),
        };

        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(format!("\"{}\"", response)),
            ])
            .unwrap(),
        ))
    });

    let mut vm = create_aqua_vm(call_service, "");

    let res = vm
        .call(json!([String::from("init_user_pk"), script, data,]))
        .expect("should be successful");

    let resulted_data: JValue = serde_json::from_str(&res.data).expect("should be correct json");

    data_value.as_object_mut().unwrap().insert(
        String::from("__call"),
        json!([{"call": "executed"}, {"call": "executed"}, {"call": "executed"}, {"call": "executed"}]),
    );

    assert_eq!(resulted_data, data_value);
    assert!(res.next_peer_pks.is_empty());
}

#[test]
fn evidence_par_seq_fold_call() {
    let return_numbers_call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![
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
        (par (
            (seq (
                (call ("some_peer_id_1" ("local_service_id" "local_fn_name") () IterableResultPeer1))
                (fold (IterableResultPeer1 i
                    (par (
                        (call ("some_peer_id_2" ("local_service_id" "local_fn_name") (i) acc[]))
                        (next i)
                    ))
                ))
            ))
            (call ("some_peer_id_3" ("local_service_id" "local_fn_name") () result_2))
        ))"#,
    );

    let res1 = vm2
        .call(json!([
            String::from("asd"),
            script,
            json!({
                "__call": []
            })
            .to_string(),
        ]))
        .expect("should be successful");

    let res2 = vm1
        .call(json!([String::from("asd"), script, res1.data]))
        .expect("should be successful");

    let mut data = res2.data;

    for _ in 0..100 {
        let res3 = vm2
            .call(json!([String::from("asd"), script, data]))
            .expect("should be successful");

        data = res3.data;
    }

    let res4 = vm3
        .call(json!([String::from("asd"), script, data]))
        .expect("should be successful");

    let resulted_json: JValue =
        serde_json::from_str(&res4.data).expect("stepper should return valid json");

    let right_json = json!( {
        "result_2": "test",
        "IterableResultPeer1": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        "acc": [1,2,3,4,5,6,7,8,9,10],
        "__call": [
            { "par": [21,1] },
            { "call": "executed" },
            { "par": [1,18] },
            { "call": "executed" },
            { "par": [1,16] },
            { "call": "executed" },
            { "par": [1,14] },
            { "call": "executed" },
            { "par": [1,12] },
            { "call": "executed" },
            { "par": [1,10] },
            { "call": "executed" },
            { "par": [1,8] },
            { "call": "executed" },
            { "par": [1,6] },
            { "call": "executed" },
            { "par": [1,4] },
            { "call": "executed" },
            { "par": [1,2] },
            { "call": "executed" },
            { "par": [1,0] },
            { "call": "executed" },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert!(res4.next_peer_pks.is_empty());
}
