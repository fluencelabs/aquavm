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

use air::SecurityTetraplet;
use air_test_utils::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

type ArgTetraplets = Vec<Vec<SecurityTetraplet>>;

fn arg_host_function() -> (CallServiceClosure, Rc<RefCell<ArgTetraplets>>) {
    let arg_tetraplets = Rc::new(RefCell::new(ArgTetraplets::new()));

    let arg_tetraplets_inner = arg_tetraplets.clone();
    let host_function: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let result = json!(params.tetraplets);
        *arg_tetraplets_inner.borrow_mut() = params.tetraplets;

        CallServiceResult::ok(result)
    });

    (host_function, arg_tetraplets)
}

#[test]
fn fold_with_inner_call() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!(["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]))
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
    let script = f!(r#"
        (seq
            (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] IterableResultPeer1)
            (fold IterableResultPeer1 i
                (par
                    (call i ("local_service_id" "local_fn_name") [i "some_text_literal"] $acc)
                    (next i)
                )
            )
        )
        "#);

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), script.clone(), "", "");
    let mut data = result.data;

    let first_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
        json_path: String::new(),
    };

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: test_params.init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
        json_path: String::new(),
    };

    let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));
    for i in 0..10 {
        let result = checked_call_vm!(client_vms[i].0, test_params.clone(), script.clone(), "", data);
        data = result.data;

        assert_eq!(client_vms[i].1, expected_tetraplets);
    }
}

#[test]
fn fold_json_path() {
    let variable_numbers = json!({"args": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]});

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(variable_numbers),
        set_variable_vm_peer_id.clone(),
    );

    let (arg_host_func, arg_tetraplets) = arg_host_function();
    let client_peer_id = String::from("client_id");
    let mut client_vm = create_avm(arg_host_func, client_peer_id.clone());

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = f!(r#"
       (seq
            (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] IterableResultPeer1)
            (fold IterableResultPeer1.$.args i
                (seq
                    (fold IterableResultPeer1.$.args j
                        (seq
                            (call "{client_peer_id}" ("local_service_id" "local_fn_name") [i "some_text_literal"] $acc)
                            (next j)
                        )
                    )
                    (next i)
                )
            )
        )
        "#);

    let test_params = TestRunParameters::from_init_peer_id("some_init_peer_id");
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), script.clone(), "", "");

    let first_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
        json_path: String::from(".$.args"),
    };

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: test_params.init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
        json_path: String::new(),
    };

    let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));
    checked_call_vm!(client_vm, test_params, script, "", result.data);
    assert_eq!(arg_tetraplets, expected_tetraplets);
}

#[test]
fn check_tetraplet_works_correctly() {
    let return_numbers_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!({"args": ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]}))
    });

    let set_variable_vm_peer_id = String::from("some_peer_id_1");
    let mut set_variable_vm = create_avm(return_numbers_call_service, set_variable_vm_peer_id.clone());

    let (arg_host_func, arg_tetraplets) = arg_host_function();
    let client_peer_id = String::from("client_id");
    let mut client_vm = create_avm(arg_host_func, client_peer_id.clone());

    let service_id = String::from("some_service_id");
    let function_name = String::from("some_function_name");
    let script = f!(r#"
        (seq
            (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] value)
            (seq
                (call "{client_peer_id}" ("local_service_id" "local_fn_name") [value.$.args value.$.args.[0]])
                (call "{client_peer_id}" ("local_service_id" "local_fn_name") [value.$.args value.$.args.[0]])
            )
        )"#);

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script.clone(), "", "");

    let first_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id.clone(),
        service_id: service_id.clone(),
        function_name: function_name.clone(),
        json_path: String::from(".$.args"),
    };

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
        json_path: String::from(".$.args.[0]"),
    };

    let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet]];
    let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));
    checked_call_vm!(client_vm, <_>::default(), script, "", result.data);
    assert_eq!(arg_tetraplets, expected_tetraplets);
}

use fluence_app_service::AppService;
use fluence_app_service::AppServiceConfig;
use fluence_app_service::MarineConfig;
use fluence_app_service::ModuleDescriptor;

use air_test_utils::trace_from_result;
use std::path::PathBuf;

fn construct_service_config(module_name: impl Into<String>) -> AppServiceConfig {
    let module_name = module_name.into();
    let module_path = format!("./tests/security_tetraplets/{module_name}/target/wasm32-wasi/debug/");

    let module_descriptor = ModuleDescriptor {
        file_name: module_name.clone() + ".wasm",
        import_name: module_name,
        ..<_>::default()
    };

    let marine_config = MarineConfig {
        modules_dir: Some(PathBuf::from(module_path)),
        modules_config: vec![module_descriptor],
        default_modules_config: None,
    };

    let service_base_dir = std::env::temp_dir();

    AppServiceConfig {
        service_working_dir: service_base_dir.clone(),
        service_base_dir,
        marine_config,
    }
}

#[test]
#[ignore]
fn tetraplet_with_wasm_modules() {
    use marine_rs_sdk::CallParameters;
    use marine_rs_sdk::SecurityTetraplet as SDKTetraplet;

    let auth_module_name = String::from("auth_module");
    let auth_service_config = construct_service_config(auth_module_name.clone());
    let auth_service = AppService::new(auth_service_config, auth_module_name, <_>::default()).unwrap();

    let log_module_name = String::from("log_storage");
    let log_service_config = construct_service_config(log_module_name.clone());
    let log_service = AppService::new(log_service_config, log_module_name, <_>::default()).unwrap();

    let services = maplit::hashmap!(
      "auth" => auth_service,
      "log_storage" => log_service,
    );
    let services = Rc::new(RefCell::new(services));

    let services_inner = services.clone();
    const ADMIN_PEER_PK: &str = "12D3KooWEXNUbCXooUwHrHBbrmjsrpHXoEphPwbjQXEGyzbqKnE1";
    let host_func: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        let tetraplets = serde_json::to_vec(&params.tetraplets).expect("default serializer shouldn't fail");
        let tetraplets: Vec<Vec<SDKTetraplet>> =
            serde_json::from_slice(&tetraplets).expect("default deserializer shouldn't fail");

        let mut call_parameters = CallParameters::default();
        call_parameters.init_peer_id = ADMIN_PEER_PK.to_string();
        call_parameters.tetraplets = tetraplets;

        let mut service = services_inner.borrow_mut();
        let service = service.get_mut(params.service_id.as_str()).unwrap();

        let result = service
            .call(params.function_name, JValue::Array(params.arguments), call_parameters)
            .unwrap();

        CallServiceResult::ok(result)
    });

    let local_peer_id = "local_peer_id";
    let script = f!(r#"
        (seq
            (call "{local_peer_id}" ("auth" "is_authorized") [] auth_result)
            (call "{local_peer_id}" ("log_storage" "delete") [auth_result.$.is_authorized "1"])
        )
    "#);

    let mut vm = create_avm(host_func, local_peer_id);

    let test_params = TestRunParameters::from_init_peer_id(ADMIN_PEER_PK);
    let result = checked_call_vm!(vm, test_params, script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("Ok");

    assert_eq!(actual_trace[(1 as PosType).into()], expected_state)
}
