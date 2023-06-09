use air_test_utils::prelude::*;

use serde_json::json;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static VM: RefCell<TestRunner> = RefCell::new({
    let add_module_response = String::from("add_module response");
    let add_blueprint_response = String::from("add_blueprint response");
    let create_response = String::from("create response");

    let call_service: CallServiceClosure = Box::new(move |args| -> CallServiceResult {
        let response = match args.service_id.as_str() {
            "add_module" => add_module_response.clone(),
            "add_blueprint" => add_blueprint_response.clone(),
            "create" => create_response.clone(),
            _ => String::from("unknown response"),
        };

        CallServiceResult::ok(json!(response))
    });

    create_avm(call_service, "A")
}));

thread_local!(static SET_VARIABLES_VM: RefCell<TestRunner> = RefCell::new({
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
        String::from("module_bytes") => json!(module_bytes),
        String::from("module_config") => json!(module_config),
        String::from("blueprint") => json!(blueprint),
    );

    create_avm(set_variables_call_service(variables_mapping, VariableOptionSource::Argument(0)), "set_variables")
}));

fn create_service_benchmark() -> Result<RawAVMOutcome, String> {
    let script = r#"
        (seq 
            (seq 
                (seq 
                    (call "set_variables" ("" "") ["module_bytes"] module_bytes)
                    (call "set_variables" ("" "") ["module_config"] module_config)
                )
                (call "set_variables" ("" "") ["blueprint"] blueprint)
            )
            (seq 
                (call "A" ("add_module" "") [module_bytes module_config] module)
                (seq 
                    (call "A" ("add_blueprint" "") [blueprint] blueprint_id)
                    (seq 
                        (call "A" ("create" "") [blueprint_id] service_id)
                        (call "remote_peer_id" ("" "") [service_id] client_result)
                    )
                )
            )
        )"#;

    let run_parameters1 = TestRunParameters::new("set_variables", 0, 1, "");
    let run_parameters2 = run_parameters1.clone();
    let result = SET_VARIABLES_VM
        .with(|vm| vm.borrow_mut().call(script, "", "", run_parameters1))
        .unwrap();
    VM.with(|vm| vm.borrow_mut().call(script, "", result.data, run_parameters2))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_service", move |b| b.iter(create_service_benchmark));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
