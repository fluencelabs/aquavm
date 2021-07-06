use air_test_utils::create_avm;
use air_test_utils::unit_call_service;
use air_test_utils::AVMError;
use air_test_utils::InterpreterOutcome;
use air_test_utils::AVM;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static VM: RefCell<AVM> = RefCell::new(create_avm(unit_call_service(), "test_peer_id")));
thread_local!(static SCRIPT: String = String::from(
        r#"
            (call "test_peer_id" ("local_service_id" "local_fn_name") [] result_name)
        "#,
    )
);

fn current_peer_id_call() -> Result<InterpreterOutcome, AVMError> {
    VM.with(|vm| SCRIPT.with(|script| vm.borrow_mut().call_with_prev_data("", script.clone(), "", "")))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("current_peer_id_call", move |b| b.iter(move || current_peer_id_call()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
