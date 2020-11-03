use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::unit_call_service;
use aquamarine_vm::vec1::Vec1;
use aquamarine_vm::AquamarineVM;
use aquamarine_vm::AquamarineVMError;
use aquamarine_vm::HostExportedFunc;
use aquamarine_vm::IValue;
use aquamarine_vm::StepperOutcome;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use std::cell::RefCell;

thread_local!(static RELAY_1_VM: RefCell<AquamarineVM> = RefCell::new(create_aqua_vm(unit_call_service(), "Relay1")));
thread_local!(static RELAY_2_VM: RefCell<AquamarineVM> = RefCell::new(create_aqua_vm(unit_call_service(), "Relay2")));
thread_local!(static REMOTE_VM: RefCell<AquamarineVM> = RefCell::new({
    let members_call_service: HostExportedFunc = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            Vec1::new(vec![
                IValue::S32(0),
                IValue::String(String::from(r#"[["A", "Relay1"], ["B", "Relay2"]]"#)),
            ])
            .unwrap(),
        ))
    });

    create_aqua_vm(members_call_service, "Remote")
}));
thread_local!(static CLIENT_1_VM: RefCell<AquamarineVM> = RefCell::new(create_aqua_vm(unit_call_service(), "A")));
thread_local!(static CLIENT_2_VM: RefCell<AquamarineVM> = RefCell::new(create_aqua_vm(unit_call_service(), "B")));

fn chat_sent_message_benchmark() -> Result<StepperOutcome, AquamarineVMError> {
    let script = String::from(
        r#"
            (seq 
                (call "Relay1" ("identity" "") [] void1[])
                (seq 
                    (call "Remote" ("552196ea-b9b2-4761-98d4-8e7dba77fac4" "add") [] void2[])
                    (seq 
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par 
                                (seq 
                                    (call m.$.[1] ("identity" "") [] void[])
                                    (call m.$.[0] ("fgemb3" "add") [] void3[])
                                )
                                (next m)
                            )
                        )
                    )
                )
            )
        "#,
    );

    let res = CLIENT_1_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", "[]"))
        .unwrap();
    let res = RELAY_1_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res.data))
        .unwrap();
    let res = REMOTE_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res.data))
        .unwrap();
    let res_data = res.data.clone();
    let res1 = RELAY_1_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res_data))
        .unwrap();
    CLIENT_1_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res1.data))
        .unwrap();
    let res2 = RELAY_2_VM
        .with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res.data))
        .unwrap();
    CLIENT_2_VM.with(|vm| vm.borrow_mut().call_with_prev_data("", script.clone(), "[]", res2.data))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chat_send_message", move |b| {
        b.iter(move || chat_sent_message_benchmark())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
