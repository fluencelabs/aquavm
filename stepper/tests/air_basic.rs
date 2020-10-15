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
use aqua_test_utils::unit_call_service;
use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::IValue;

use serde_json::json;

type JValue = serde_json::Value;

#[test]
fn seq_par_call() {
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
        .call(json!([String::from("asd"), script, String::from("{}"),]))
        .expect("should be successful");

    let resulted_json: JValue =
        serde_json::from_str(&res.data).expect("stepper should return valid json");

    let right_json = json!( {
        "result_1" : "test",
        "__call": [
            { "par": [1,1] },
            { "call": "executed" },
            { "call": "request_sent" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn par_par_call() {
    let mut vm = create_aqua_vm(unit_call_service(), "");

    let script = String::from(
        r#"
        (par (
            (par (
                (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_1))
                (call ("remote_peer_id" ("service_id" "fn_name") () g))
            ))
            (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_2))
        ))"#,
    );

    let res = vm
        .call(json!([String::from("asd"), script, String::from("{}"),]))
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
            { "call": "request_sent" },
            { "call": "executed" },
        ]
    });

    assert_eq!(resulted_json, right_json);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn create_service() {
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
        String::from("module"),
        JValue::String(String::from("add_module response")),
    );
    data_value.as_object_mut().unwrap().insert(
        String::from("blueprint_id"),
        JValue::String(String::from("add_blueprint response")),
    );
    data_value.as_object_mut().unwrap().insert(
        String::from("service_id"),
        JValue::String(String::from("create response")),
    );
    data_value.as_object_mut().unwrap().insert(
        String::from("__call"),
        json!([{"call": "executed"}, {"call": "executed"}, {"call": "executed"}, {"call": "request_sent"}]),
    );

    assert_eq!(resulted_data, data_value);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}
