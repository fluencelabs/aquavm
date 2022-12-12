use air_test_utils::prelude::*;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use serde_json::json;

use std::cell::RefCell;

thread_local!(static RELAY_1_VM: RefCell<TestRunner> = RefCell::new(create_avm(unit_call_service(), "Relay1")));
thread_local!(static RELAY_2_VM: RefCell<TestRunner> = RefCell::new(create_avm(unit_call_service(), "Relay2")));
thread_local!(static REMOTE_VM: RefCell<TestRunner> = RefCell::new({
    let members_call_service: CallServiceClosure = Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!([["A", "Relay1"], ["B", "Relay2"]]))
    });

    create_avm(members_call_service, "Remote")
}));
thread_local!(static CLIENT_1_VM: RefCell<TestRunner> = RefCell::new(create_avm(unit_call_service(), "A")));
thread_local!(static CLIENT_2_VM: RefCell<TestRunner> = RefCell::new(create_avm(unit_call_service(), "B")));

fn chat_sent_message_benchmark() -> Result<RawAVMOutcome, String> {
    let script = r#"
            (seq 
                (call "Relay1" ("identity" "") [] $void1)
                (seq 
                    (call "Remote" ("552196ea-b9b2-4761-98d4-8e7dba77fac4" "add") [] $void2)
                    (seq 
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par 
                                (seq 
                                    (call m.$.[1]! ("identity" "") [] $void)
                                    (call m.$.[0]! ("fgemb3" "add") [] $void3)
                                )
                                (next m)
                            )
                        )
                    )
                )
            )
        "#;

    let run_parameters = TestRunParameters::new("A", 0, 1);
    let result = CLIENT_1_VM
        .with(|vm| vm.borrow_mut().call(script, "", "", run_parameters.clone()))
        .unwrap();
    let result = RELAY_1_VM
        .with(|vm| vm.borrow_mut().call(script, "", result.data, run_parameters.clone()))
        .unwrap();
    let result = REMOTE_VM
        .with(|vm| vm.borrow_mut().call(script, "", result.data, run_parameters.clone()))
        .unwrap();
    let res_data = result.data.clone();
    let res1 = RELAY_1_VM
        .with(|vm| vm.borrow_mut().call(script, "", res_data, run_parameters.clone()))
        .unwrap();
    CLIENT_1_VM
        .with(|vm| vm.borrow_mut().call(script, "", res1.data, run_parameters.clone()))
        .unwrap();
    let res2 = RELAY_2_VM
        .with(|vm| vm.borrow_mut().call(script, "", result.data, run_parameters.clone()))
        .unwrap();
    CLIENT_2_VM.with(|vm| vm.borrow_mut().call(script, "", res2.data, run_parameters.clone()))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chat_send_message", move |b| b.iter(chat_sent_message_benchmark));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
