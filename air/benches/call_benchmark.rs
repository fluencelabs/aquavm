use air_test_utils::prelude::*;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static VM: RefCell<TestRunner<ReleaseWasmAirRunner>> = RefCell::new(
    create_custom_avm(unit_call_service(), "test_peer_id"))
);
thread_local!(static SCRIPT: String = String::from(
        r#"
            (call "test_peer_id" ("local_service_id" "local_fn_name") [] result_name)
        "#,
    )
);

fn current_peer_id_call() -> Result<RawAVMOutcome, String> {
    let run_parameters = TestRunParameters::new("test_peer_id", 0, 1, "");
    VM.with(|vm| SCRIPT.with(|script| vm.borrow_mut().call(script, "", "", run_parameters)))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("current_peer_id_call", move |b| b.iter(current_peer_id_call));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
