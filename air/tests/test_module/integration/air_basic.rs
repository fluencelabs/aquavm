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

use air::PreparationError;
use air_test_utils::prelude::*;

#[test]
fn seq_par_call() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

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

    let result = checked_call_vm!(vm, "asd", script, "", "");
    let actual_trace = trace_from_result(&result);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace = vec![
        executed_state::par(1, 1),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::request_sent_by(vm_peer_id),
        executed_state::scalar_string(unit_call_service_result),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn par_par_call() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

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

    let result = checked_call_vm!(vm, "asd", script, "", "");
    let actual_trace = trace_from_result(&result);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace = vec![
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::request_sent_by(vm_peer_id),
        executed_state::scalar_string(unit_call_service_result),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn create_service() {
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
        "module_bytes".to_string() => module_bytes.clone(),
        "module_config".to_string() => module_config.clone(),
        "blueprint".to_string() => blueprint.clone(),
    );

    let mut set_variables_vm = create_avm(
        set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)),
        "set_variables",
    );

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let call_service: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let response = match params.service_id.as_str() {
            "add_module" => add_module_response,
            "add_blueprint" => add_blueprint_response,
            "create" => create_response,
            _ => "unknown response",
        };

        CallServiceResult::ok(json!(response))
    });

    let mut vm = create_avm(call_service, "A");

    let script = include_str!("./scripts/create_service.clj");

    let result = checked_call_vm!(set_variables_vm, "init_peer_id", script.clone(), "", "");
    let result = checked_call_vm!(vm, "init_peer_id", script, "", result.data);

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        executed_state::scalar(module_bytes),
        executed_state::scalar(module_config),
        executed_state::scalar(blueprint),
        executed_state::scalar_string(add_module_response),
        executed_state::scalar_string(add_blueprint_response),
        executed_state::scalar_string(create_response),
        executed_state::request_sent_by("A"),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[test]
fn invalid_air() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = r#"(seq )"#;

    let result = call_vm!(vm, "", &script, "", "");

    let error_message = air_parser::parse(script).into_err();
    let expected_error = PreparationError::AIRParseError(error_message);
    assert!(check_error(&result, expected_error));
}
