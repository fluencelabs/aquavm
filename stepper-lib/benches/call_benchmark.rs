use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::unit_call_service;
use aquamarine_vm::AquamarineVM;
use aquamarine_vm::AquamarineVMError;
use aquamarine_vm::StepperOutcome;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static VM: RefCell<AquamarineVM> = RefCell::new(create_aqua_vm(unit_call_service(), "test_peer_id")));
thread_local!(static SCRIPT: String = String::from(
        r#"
            (call (%current_peer_id% ("local_service_id" "local_fn_name") () result_name))
        "#,
    )
);

fn current_peer_id_call() -> Result<StepperOutcome, AquamarineVMError> {
    VM.with(|vm| SCRIPT.with(|script| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", "[]")))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("current_peer_id_call", move |b| b.iter(move || current_peer_id_call()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
