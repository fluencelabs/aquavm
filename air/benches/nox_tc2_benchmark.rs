use air_test_utils::prelude::*;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;
use std::time::Duration;

thread_local!(static VM: RefCell<TestRunner<ReleaseWasmAirRunner>> = RefCell::new(
    create_custom_avm(unit_call_service(), "test_peer_id"))
);
thread_local!(static SCRIPT: String = String::from(
r#"
    (seq
        (ap "123" $stream)
        (canon "test_peer_id" $stream #stream-canon)
    )
"#,
    )
);

fn nox_tc2_benchmark() -> Result<RawAVMOutcome, String> {
    let run_parameters = TestRunParameters::new("test_peer_id", 0, 1, "");
    VM.with(|vm| SCRIPT.with(|script| vm.borrow_mut().call(script, "", "", run_parameters)))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nox_tc2_benchmark", move |b: &mut criterion::Bencher<'_>| {
        b.iter(nox_tc2_benchmark)
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
