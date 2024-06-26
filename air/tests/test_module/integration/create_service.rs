/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air_test_utils::prelude::*;
use futures::FutureExt;

#[tokio::test]
async fn create_service() {
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
    )
    .await;

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let call_service: CallServiceClosure = Box::new(move |params| {
        let response = match params.service_id.as_str() {
            "add_module" => add_module_response,
            "add_blueprint" => add_blueprint_response,
            "create" => create_response,
            _ => "unknown response",
        };

        let result = CallServiceResult::ok(json!(response));
        async move { result }.boxed_local()
    });

    let mut vm = create_avm(call_service, "A").await;

    let script = include_str!("./scripts/create_service.air");

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let result = checked_call_vm!(set_variables_vm, test_params.clone(), script, "", "");
    let result = checked_call_vm!(vm, test_params, script, "", result.data);

    let add_module_response = "add_module response";
    let add_blueprint_response = "add_blueprint response";
    let create_response = "create response";

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![
        scalar!(
            module_bytes.clone(),
            peer = "set_variables",
            service = "add_module",
            args = ["module_bytes"]
        ),
        scalar!(
            module_config.clone(),
            peer = "set_variables",
            service = "add_module",
            args = ["module_config"]
        ),
        scalar!(
            blueprint.clone(),
            peer = "set_variables",
            service = "add_module",
            args = ["blueprint"]
        ),
        scalar!(
            add_module_response,
            peer = "A",
            service = "add_module",
            args = [module_bytes, module_config]
        ),
        scalar!(
            add_blueprint_response,
            peer = "A",
            service = "add_blueprint",
            args = [blueprint]
        ),
        scalar!(
            create_response,
            peer = "A",
            service = "create",
            args = [add_blueprint_response]
        ),
        executed_state::request_sent_by("A"),
    ];

    assert_eq!(actual_trace, expected_trace);
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}
