use air_test_utils::prelude::*;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use serde_json::Value;

use std::cell::RefCell;

thread_local!(static VM: RefCell<TestRunner<ReleaseWasmAirRunner>> = RefCell::new(
    create_custom_avm(unit_call_service(), "test_peer_id")
));
// effectively, we measure just loading time
const SCRIPT: &str = "(par (null) (null))";

// this is the data with large number of smaller values
const VALUES_DATA: &str = include_str!("data/anomaly_long.json");

fn data_long_calls(prev_data: &str, current_data: &str) -> Result<RawAVMOutcome, String> {
    let run_parameters = TestRunParameters::new("test_peer_id", 0, 1);
    VM.with(|vm| vm.borrow_mut().call(SCRIPT, prev_data, current_data, run_parameters))
}

fn build_par_data(data: &mut Value, hangs_left: bool) {
    let trace = data.get_mut("trace").unwrap().as_array_mut().unwrap();
    let trace_len = trace.len();
    let par = if hangs_left {
        json!({"par": [trace_len, 0]})
    } else {
        json!({"par": [0, trace_len]})
    };
    trace.insert(0, par);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut data_right = serde_json::from_str::<Value>(VALUES_DATA).unwrap();
    let mut data_left = data_right.clone();

    build_par_data(&mut data_right, false);
    build_par_data(&mut data_left, true);

    let json_data_right = serde_json::to_string(&data_right).unwrap();
    std::mem::drop(data_right);
    let json_data_left = serde_json::to_string(&data_left).unwrap();
    std::mem::drop(data_left);

    // the traces contain different par branches
    c.bench_function("data_long_calls wo merge", |b| {
        b.iter(|| data_long_calls(&json_data_right, &json_data_left).unwrap())
    });
    // the traces contain same par branch and arge merged
    c.bench_function("data_long_calls with merge", |b| {
        b.iter(|| data_long_calls(&json_data_right, &json_data_right).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
