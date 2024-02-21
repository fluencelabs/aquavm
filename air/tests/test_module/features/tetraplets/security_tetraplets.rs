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

use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;
use polyplets::SecurityTetraplet;
use pretty_assertions::assert_eq;

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
    let script = format!(
        r#"
        (seq
            (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] IterableResultPeer1)
            (fold IterableResultPeer1 i
                (par
                    (call i ("local_service_id" "local_fn_name") [i "some_text_literal"] $acc)
                    (next i)
                )
            )
        )
        "#
    );

    let test_params = TestRunParameters::from_init_peer_id("init_peer_id");
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), script.clone(), "", "");
    let mut data = result.data;

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: test_params.init_peer_id.clone(),
        ..Default::default()
    };

    for i in 0..10 {
        let result = checked_call_vm!(client_vms[i].0, test_params.clone(), script.clone(), "", data);
        data = result.data;

        let first_arg_tetraplet = SecurityTetraplet {
            peer_pk: set_variable_vm_peer_id.clone(),
            service_id: service_id.clone(),
            function_name: function_name.clone(),
            lens: format!(".$.[{}]", i),
        };

        let expected_tetraplets = vec![vec![first_arg_tetraplet], vec![second_arg_tetraplet.clone()]];
        let expected_tetraplets = Rc::new(RefCell::new(expected_tetraplets));

        assert_eq!(client_vms[i].1, expected_tetraplets);
    }
}

#[test]
fn fold_stream_with_inner_call() {
    let init_peer_name = "init_peer_id";
    let air_script = r#"
      (seq
         (seq
            (call "init_peer_id" ("" "") [] $stream) ; ok = 42
            (seq
               (call "init_peer_id" ("" "") [] var) ; ok = {"field": 43}
               (ap var.$.field $stream)))
         (fold $stream i
            (seq
               (call "init_peer_id" ("" "") [i] $s2) ; behaviour = tetraplet
               (next i))))
    "#;
    let executor = air_test_framework::AirScriptExecutor::from_annotated(
        TestRunParameters::from_init_peer_id(init_peer_name),
        &air_script,
    )
    .unwrap();

    let result = executor.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{}", result.error_message);
    let data = data_from_result(&result);

    let init_peer_id = at(init_peer_name);

    let expected_trace = vec![
        stream!(
            json!([[{"peer_pk": init_peer_id, "service_id": "..0", "function_name": "", "lambda": ""}]]),
            0,
            peer = &init_peer_id,
            service = "..2",
            args = [42]
        ),
        stream!(
            json!([[{"peer_pk": init_peer_id, "service_id": "..1", "function_name": "", "lambda": ".$.field"}]]),
            0,
            peer = init_peer_id,
            service = "..2",
            args = [43]
        ),
    ];
    assert_eq!(&(*data.trace)[4..], &expected_trace, "{:?}", data.cid_info);
}

#[test]
fn fold_canon_with_inner_call() {
    let init_peer_name = "init_peer_id";
    let air_script = r#"
      (seq
         (seq
            (seq
               (call "init_peer_id" ("" "") [] $stream) ; ok = 42
               (call "init_peer_id" ("" "") [] var)) ; ok = {"field": 43}
            (ap var.$.field $stream))
         (seq
            (canon "init_peer_id" $stream #can)
            (fold #can x
              (seq
                (call "init_peer_id" ("" "") [x] $s2) ; behaviour=tetraplet
                (next x)))))
    "#;
    let executor = air_test_framework::AirScriptExecutor::from_annotated(
        TestRunParameters::from_init_peer_id(init_peer_name),
        &air_script,
    )
    .unwrap();

    let result = executor.execute_one(init_peer_name).unwrap();
    assert_eq!(result.ret_code, 0, "{}", result.error_message);
    let data = data_from_result(&result);

    let init_peer_id = at(init_peer_name);

    let expected_trace = vec![
        stream!(
            json!([[{"peer_pk": init_peer_id, "service_id": "..0", "function_name": "", "lambda": ""}]]),
            0,
            peer = &init_peer_id,
            service = "..2",
            args = [42]
        ),
        stream!(
            json!([[{"peer_pk": init_peer_id, "service_id": "..1", "function_name": "", "lambda": ".$.field"}]]),
            1,
            peer = init_peer_id,
            service = "..2",
            args = [43]
        ),
    ];
    assert_eq!(&(*data.trace)[4..], &expected_trace, "{:?}", data.cid_info);
}

#[test]
fn fold_lambda() {
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
    let script = format!(
        r#"
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
        "#
    );

    let test_params = TestRunParameters::from_init_peer_id("some_init_peer_id");
    let result = checked_call_vm!(set_variable_vm, test_params.clone(), script.clone(), "", "");

    let first_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
        lens: String::from(".$.args.$.[9]"),
    };

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: test_params.init_peer_id.clone(),
        service_id: String::new(),
        function_name: String::new(),
        lens: String::new(),
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
    let script = format!(
        r#"
        (seq
            (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] value)
            (seq
                (call "{client_peer_id}" ("local_service_id" "local_fn_name") [value.$.args value.$.args.[0]])
                (call "{client_peer_id}" ("local_service_id" "local_fn_name") [value.$.args value.$.args.[0]])
            )
        )"#
    );

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script.clone(), "", "");

    let first_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id.clone(),
        service_id: service_id.clone(),
        function_name: function_name.clone(),
        lens: String::from(".$.args"),
    };

    let second_arg_tetraplet = SecurityTetraplet {
        peer_pk: set_variable_vm_peer_id,
        service_id,
        function_name,
        lens: String::from(".$.args.[0]"),
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
        total_memory_limit: None,
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
        call_parameters.particle.init_peer_id = ADMIN_PEER_PK.to_string();
        call_parameters.tetraplets = tetraplets;

        let mut service = services_inner.borrow_mut();
        let service = service.get_mut(params.service_id.as_str()).unwrap();

        let result = service
            .call(
                params.function_name,
                json!(params.arguments),
                to_app_service_call_parameters(call_parameters),
            )
            .unwrap();

        CallServiceResult::ok(result)
    });

    let local_peer_id = "local_peer_id";
    let script = format!(
        r#"
        (seq
            (call "{local_peer_id}" ("auth" "is_authorized") [] auth_result)
            (call "{local_peer_id}" ("log_storage" "delete") [auth_result.$.is_authorized "1"])
        )
    "#
    );

    let mut vm = create_avm(host_func, local_peer_id);

    let test_params = TestRunParameters::from_init_peer_id(ADMIN_PEER_PK);
    let result = checked_call_vm!(vm, test_params, script, "", "");
    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!("Ok");

    assert_eq!(actual_trace[1.into()], expected_state)
}

fn to_app_service_call_parameters(
    call_parameters: marine_rs_sdk::CallParameters,
) -> fluence_app_service::CallParameters {
    fluence_app_service::CallParameters {
        particle: to_app_service_particle_parameters(call_parameters.particle),
        service_id: call_parameters.service_id,
        service_creator_peer_id: call_parameters.service_creator_peer_id,
        host_id: call_parameters.host_id,
        worker_id: call_parameters.worker_id,
        tetraplets: call_parameters
            .tetraplets
            .into_iter()
            .map(to_app_service_tetraplets)
            .collect(),
    }
}

fn to_app_service_particle_parameters(
    particle: marine_rs_sdk::ParticleParameters,
) -> fluence_app_service::ParticleParameters {
    fluence_app_service::ParticleParameters {
        id: particle.id,
        init_peer_id: particle.init_peer_id,
        timestamp: particle.timestamp,
        ttl: particle.ttl,
        script: particle.script,
        signature: particle.signature,
        token: particle.token,
    }
}

fn to_app_service_tetraplets(
    tetraplets: Vec<marine_rs_sdk::SecurityTetraplet>,
) -> Vec<fluence_app_service::SecurityTetraplet> {
    tetraplets.into_iter().map(to_app_service_tetraplet).collect()
}

fn to_app_service_tetraplet(tetraplet: marine_rs_sdk::SecurityTetraplet) -> fluence_app_service::SecurityTetraplet {
    fluence_app_service::SecurityTetraplet {
        peer_pk: tetraplet.peer_pk,
        service_id: tetraplet.service_id,
        function_name: tetraplet.function_name,
        lens: tetraplet.lens,
    }
}
