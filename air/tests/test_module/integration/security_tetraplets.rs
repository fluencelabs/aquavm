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

use air::ResolvedTriplet;
use air::SecurityTetraplet;
use air_test_utils::checked_call_vm;
use air_test_utils::create_avm;
use air_test_utils::executed_state;
use air_test_utils::CallServiceClosure;
use air_test_utils::IValue;
use air_test_utils::NEVec;

use std::cell::RefCell;
use std::rc::Rc;

type ArgTetraplets = Vec<Vec<SecurityTetraplet>>;

fn arg_host_function() -> (CallServiceClosure, Rc<RefCell<ArgTetraplets>>) {
    let arg_tetraplets = Rc::new(RefCell::new(ArgTetraplets::new()));

    let arg_tetraplets_inner = arg_tetraplets.clone();
    let host_function: CallServiceClosure = Box::new(move |args| -> Option<IValue> {
        let tetraplets = match &args.function_args[3] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let de_tetraplets: ArgTetraplets =
            serde_json::from_str(tetraplets).expect("json deserialization shouldn't fail");
        *arg_tetraplets_inner.borrow_mut() = de_tetraplets;

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(tetraplets.clone())]).unwrap(),
        ))
    });

    (host_function, arg_tetraplets)
}

#[test]
fn simple_fold() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> Option<IValue> {
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

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_avm(return_numbers_call_service, set_variable_vm_peer_id.clone());

    let mut client_vms = Vec::new();
    for i in 1..=10 {
        let (arg_host_func, arg_tetraplets) = arg_host_function();
        let vm = create_avm(arg_host_func, i.to_string());
        client_vms.push((vm, arg_tetraplets))
    }

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = format!(
        r#"
        (seq
            (call "{}" ("{}" "{}") [] IterableResultPeer1)
            (fold IterableResultPeer1 i
                (par
                    (call i ("local_service_id" "local_fn_name") [i "some_text_literal"] $acc)
                    (next i)
                )
            )
        )
        "#,
        set_variable_vm_peer_id, service_id, function_name
    );

    let init_peer_id = String::from("some_init_peer_id");
    let result = checked_call_vm!(set_variable_vm, init_peer_id.clone(), script.clone(), "", "");
    let mut data = result.data;

    let first_arg_triplet = ResolvedTriplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
    };
    let first_arg_tetraplet = SecurityTetraplet {
        triplet: first_arg_triplet,
        json_path: String::new(),
    };

    let second_arg_triplet = ResolvedTriplet {
        peer_pk: init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
    };
    let second_arg_tetraplet = SecurityTetraplet {
        triplet: second_arg_triplet,
        json_path: String::new(),
    };

    let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));
    for i in 0..10 {
        let result = checked_call_vm!(client_vms[i].0, init_peer_id.clone(), script.clone(), "", data);
        data = result.data;

        assert_eq!(client_vms[i].1, expected_tetraplets);
    }
}

#[test]
fn fold_json_path() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(String::from(
                    "{\"arg\": [\"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\", \"10\"]}",
                )),
            ])
            .unwrap(),
        ))
    });

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_avm(return_numbers_call_service, set_variable_vm_peer_id.clone());

    let (arg_host_func, arg_tetraplets) = arg_host_function();
    let client_peer_id = String::from("client_id");
    let mut client_vm = create_avm(arg_host_func, client_peer_id.clone());

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = format!(
        r#"
        (seq
            (call "{}" ("{}" "{}") [] IterableResultPeer1)
            (fold IterableResultPeer1.$.arg i
                (par
                    (call "{}" ("local_service_id" "local_fn_name") [i "some_text_literal"] $acc)
                    (next i)
                )
            )
        )
        "#,
        set_variable_vm_peer_id, service_id, function_name, client_peer_id
    );

    let init_peer_id = String::from("some_init_peer_id");
    let result = checked_call_vm!(set_variable_vm, init_peer_id.clone(), script.clone(), "", "");

    let first_arg_triplet = ResolvedTriplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
    };
    let first_arg_tetraplet = SecurityTetraplet {
        triplet: first_arg_triplet,
        json_path: String::from("$.arg"),
    };

    let second_arg_triplet = ResolvedTriplet {
        peer_pk: init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
    };
    let second_arg_tetraplet = SecurityTetraplet {
        triplet: second_arg_triplet,
        json_path: String::new(),
    };

    let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));
    checked_call_vm!(client_vm, init_peer_id.clone(), script.clone(), "", result.data);
    assert_eq!(arg_tetraplets, expected_tetraplets);
}

use fluence_app_service::AppService;
use fluence_app_service::AppServiceConfig;
use fluence_app_service::FaaSConfig;
use fluence_app_service::ModuleDescriptor;

use air_test_utils::trace_from_result;
use std::path::PathBuf;

fn construct_service_config(module_name: impl Into<String>) -> AppServiceConfig {
    let module_name = module_name.into();
    let module_path = format!("./tests/security_tetraplets/{}/target/wasm32-wasi/debug/", module_name);

    let module_descriptor = ModuleDescriptor {
        file_name: module_name.clone() + ".wasm",
        import_name: module_name,
        ..<_>::default()
    };

    let faas_config = FaaSConfig {
        modules_dir: Some(PathBuf::from(module_path)),
        modules_config: vec![module_descriptor],
        default_modules_config: None,
    };

    let service_base_dir = std::env::temp_dir();

    let config = AppServiceConfig {
        service_base_dir,
        faas_config,
    };

    config
}

#[test]
#[ignore]
fn tetraplet_with_wasm_modules() {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::SecurityTetraplet as SDKTetraplet;

    let auth_module_name = String::from("auth_module");
    let auth_service_config = construct_service_config(auth_module_name.clone());
    let auth_service = AppService::new(auth_service_config, auth_module_name.clone(), <_>::default()).unwrap();

    let log_module_name = String::from("log_storage");
    let log_service_config = construct_service_config(log_module_name.clone());
    let log_service = AppService::new(log_service_config, log_module_name.clone(), <_>::default()).unwrap();

    let services = maplit::hashmap!(
      "auth" => auth_service,
      "log_storage" => log_service,
    );
    let services = Rc::new(RefCell::new(services));

    let services_inner = services.clone();
    const ADMIN_PEER_PK: &str = "12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE1";
    let host_func: CallServiceClosure = Box::new(move |args| -> Option<IValue> {
        let args = &args.function_args;

        let service_id = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let function_name = match &args[1] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let service_args = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let tetraplets = match &args[3] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let tetraplets: Vec<Vec<SDKTetraplet>> = serde_json::from_str(tetraplets).unwrap();

        let mut call_parameters = CallParameters::default();
        call_parameters.init_peer_id = ADMIN_PEER_PK.to_string();
        call_parameters.tetraplets = tetraplets;

        let service_args = serde_json::from_str(service_args).unwrap();
        let mut service = services_inner.borrow_mut();
        let service: &mut AppService = service.get_mut(service_id.as_str()).unwrap();

        let result = service.call(function_name, service_args, call_parameters).unwrap();

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(result.to_string())]).unwrap(),
        ))
    });

    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
        (seq
            (call "{0}" ("auth" "is_authorized") [] auth_result)
            (call "{0}" ("log_storage" "delete") [auth_result.$.is_authorized "1"])
        )
    "#,
        local_peer_id,
    );

    let mut vm = create_avm(host_func, local_peer_id);

    let result = checked_call_vm!(vm, ADMIN_PEER_PK, script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("Ok");

    assert_eq!(actual_trace[1], expected_state)
}
