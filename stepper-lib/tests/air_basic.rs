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
use aqua_test_utils::set_variables_call_service;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;

use serde_json::json;

use std::rc::Rc;

type JValue = serde_json::Value;

#[test]
fn seq_par_call() {
    use stepper_lib::execution_trace::CallResult::*;
    use stepper_lib::execution_trace::ExecutedState::{self, *};

    let vm_peer_id = String::from("some_peer_id");
    let mut vm = create_aqua_vm(unit_call_service(), vm_peer_id.clone());

    let script = format!(
        r#"
        (seq 
            (par 
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        vm_peer_id
    );

    let res = call_vm!(vm, "asd", script, "[]", "[]");
    let resulted_path: Vec<ExecutedState> =
        serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_path = vec![
        Par(1, 1),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(RequestSentBy(vm_peer_id)),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
    ];

    assert_eq!(resulted_path, expected_path);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn par_par_call() {
    use stepper_lib::execution_trace::CallResult::*;
    use stepper_lib::execution_trace::ExecutedState::{self, *};

    let vm_peer_id = String::from("some_peer_id");
    let mut vm = create_aqua_vm(unit_call_service(), vm_peer_id.clone());

    let script = format!(
        r#"
        (par
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        vm_peer_id,
    );

    let res = call_vm!(vm, "asd", script, "[]", "[]");
    let resulted_path: Vec<ExecutedState> =
        serde_json::from_slice(&res.data).expect("stepper should return valid json");

    let test_string = String::from("test");
    let expected_path = vec![
        Par(3, 1),
        Par(1, 1),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
        Call(RequestSentBy(vm_peer_id)),
        Call(Executed(Rc::new(JValue::String(test_string.clone())))),
    ];

    assert_eq!(resulted_path, expected_path);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn create_service() {
    use stepper_lib::execution_trace::CallResult::*;
    use stepper_lib::execution_trace::ExecutedState::{self, *};

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

    let variables_mapping = maplit::hashmap!(
        String::from("module_bytes") => module_bytes.to_string(),
        String::from("module_config") => module_config.to_string(),
        String::from("blueprint") => blueprint.to_string(),
    );

    let mut set_variables_vm = create_aqua_vm(set_variables_call_service(variables_mapping), "set_variables");

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

    let res = call_vm!(set_variables_vm, "init_peer_id", script.clone(), "[]", "[]");
    let res = call_vm!(vm, "init_peer_id", script, "[]", res.data);

    let add_module_response = String::from("add_module response");
    let add_blueprint_response = String::from("add_blueprint response");
    let create_response = String::from("create response");
    let resulted_path: Vec<ExecutedState> = serde_json::from_slice(&res.data).expect("should be a correct json");
    let expected_path = vec![
        Call(Executed(Rc::new(module_bytes))),
        Call(Executed(Rc::new(module_config))),
        Call(Executed(Rc::new(blueprint))),
        Call(Executed(Rc::new(JValue::String(add_module_response)))),
        Call(Executed(Rc::new(JValue::String(add_blueprint_response)))),
        Call(Executed(Rc::new(JValue::String(create_response)))),
        Call(RequestSentBy(String::from("A"))),
    ];

    assert_eq!(resulted_path, expected_path);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id")]);
}
