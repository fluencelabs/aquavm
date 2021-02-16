use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::set_variables_call_service;
use aqua_test_utils::AquamarineVM;
use aqua_test_utils::AquamarineVMError;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;
use aqua_test_utils::StepperOutcome;

use serde_json::json;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static VM: RefCell<AquamarineVM> = RefCell::new({
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

    create_aqua_vm(call_service, "A")
}));

thread_local!(static SET_VARIABLES_VM: RefCell<AquamarineVM> = RefCell::new({
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

    create_aqua_vm(set_variables_call_service(variables_mapping), "set_variables")
}));

fn create_service_benchmark() -> Result<StepperOutcome, AquamarineVMError> {
    let script = String::from(
        r#"
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
        )"#,
    );

    let res = SET_VARIABLES_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", "[]"))
        .unwrap();
    VM.with(|vm| vm.borrow_mut().call_with_prev_data("", script, "[]", res.data))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_service", move |b| b.iter(move || create_service_benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
