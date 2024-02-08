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

use air::ExecutionCidState;
use air::PreparationError;
use air::ToErrorCode;
use air_interpreter_data::ExecutionTrace;
use air_test_framework::AirScriptExecutor;
use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;
use std::cell::RefCell;
use std::rc::Rc;

#[tokio::test]
fn lfold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let lfold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), lfold, "", "");
    let result = checked_call_vm!(vm, <_>::default(), lfold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(json!(["1", "2", "3", "4", "5"]), peer = "set_variable");

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let val = format!("{i}");
        let expected_state = stream!(val.as_str(), i as u32 - 1, peer = "A", args = [val.as_str()]);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[tokio::test]
fn rfold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let rfold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (next i)
                        (call "A" ("" "") [i] $acc)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), rfold, "", "");
    let result = checked_call_vm!(vm, <_>::default(), rfold, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 6);

    let expected_state = scalar!(json!(["1", "2", "3", "4", "5"]), peer = "set_variable");
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let val = format!("{}", 6 - i);
        let expected_state = stream!(val.as_str(), i as u32 - 1, peer = "A", args = [val.as_str()]);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[tokio::test]
fn inner_fold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 j
                            (seq
                                (call "A" ("" "") [i] $acc)
                                (next j)
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 27);

    let expected_state = scalar!(json!(["1", "2", "3", "4", "5"]), peer = "set_variable");
    assert_eq!(actual_trace[0.into()], expected_state);
    assert_eq!(actual_trace[1.into()], expected_state);

    for i in 1..=5 {
        for j in 1..=5 {
            let state_id = 1 + 5 * (i - 1) + j;
            let val = i.to_string();
            let expected_state = stream!(val.as_str(), state_id as u32 - 2, peer = "A", args = [val]);
            assert_eq!(actual_trace[state_id.into()], expected_state);
        }
    }
}

#[tokio::test]
fn inner_fold_with_same_iterator() {
    let mut vm = create_avm(
        set_variable_call_service(json!(["1", "2", "3", "4", "5"])),
        "set_variable",
    );

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] Iterable1)
                    (call "set_variable" ("" "") [] Iterable2)
                )
                (fold Iterable1 i
                    (seq
                        (fold Iterable2 i
                            (seq
                                (call "A" ("" "") [i] $acc)
                                (next i)
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let expected_error = PreparationError::AIRParseError("".to_string());
    assert_eq!(result.ret_code, expected_error.to_error_code());
}

#[tokio::test]
fn empty_iterable_fold() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!([])), "set_variable");

    let empty_fold = r#"
            (seq
                (call "set_variable" ("" "") [] Iterable)
                (fold Iterable i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), empty_fold, "", "");
    let result = checked_call_vm!(vm, <_>::default(), empty_fold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(json!([]), peer = "set_variable");

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0.into()], expected_state);
}

#[tokio::test]
fn empty_literal_array_fold() {
    let mut vm = create_avm(echo_call_service(), "A");

    let empty_fold = r#"
        (fold [] i
            (seq
                (call "A" ("" "") [i] $acc)
                (next i)
            )
        )"#;

    let result = checked_call_vm!(vm, <_>::default(), empty_fold, "", "");
    let actual_trace = trace_from_result(&result);

    assert!(actual_trace.is_empty());
}

#[tokio::test]
fn empty_fold_json_path() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!({ "messages": [] })), "set_variable");

    let empty_fold = r#"
            (seq
                (call "set_variable" ("" "") [] messages)
                (fold messages.$.messages! i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), empty_fold, "", "");
    let result = checked_call_vm!(vm, <_>::default(), empty_fold, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(json!({ "messages": [] }), peer = "set_variable")];

    assert_eq!(actual_trace, expected_trace);
}

// Check that fold works with the join behaviour without hanging up.
#[tokio::test]
fn fold_with_join() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");

    let fold_with_join = r#"
            (seq
                (call "set_variable" ("" "") [] iterable)
                (par
                    (call "unknown_peer" ("" "") [] lazy_def_variable)
                    (fold iterable i
                        (seq
                            (call "A" ("" "") [lazy_def_variable.$.hash!] $acc)
                            (next i)
                        )
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), fold_with_join, "", "");
    let result = checked_call_vm!(vm, <_>::default(), fold_with_join, "", result.data);

    let actual_trace = trace_from_result(&result);
    assert_eq!(actual_trace.len(), 4);
}

#[tokio::test]
fn lambda() {
    let mut vm = create_avm(echo_call_service(), "A");
    let mut set_variable_vm = create_avm(
        set_variable_call_service(json!({ "array": ["1","2","3","4","5"] })),
        "set_variable",
    );

    let script = r#"
            (seq
                (call "set_variable" ("" "") [] iterable)
                (fold iterable.$.array! i
                    (seq
                        (call "A" ("" "") [i] $acc)
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(json!({ "array": ["1", "2", "3", "4", "5"] }), peer = "set_variable");

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let val = format!("{i}");
        let expected_state = stream!(val.as_str(), i as u32 - 1, peer = "A", args = [val]);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[tokio::test]
fn shadowing() {
    use executed_state::*;

    let mut set_variables_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");
    let mut vm_a = create_avm(echo_call_service(), "A");
    let mut vm_b = create_avm(echo_call_service(), "B");

    let script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] iterable1)
                    (call "set_variable" ("" "") [] iterable2)
                )
                (fold iterable1 i
                    (seq
                        (seq
                            (fold iterable2 j
                                (seq
                                    (seq
                                        (call "A" ("" "") [i] local_j)
                                        (call "B" ("" "") [local_j])
                                    )
                                    (next j)
                                )
                            )
                            (par
                                (call "A" ("" "") [i] local_i)
                                (call "B" ("" "") [i])
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = checked_call_vm!(set_variables_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm_a, <_>::default(), script, "", result.data);
    let result = checked_call_vm!(vm_b, <_>::default(), script, "", result.data);
    let result = checked_call_vm!(vm_a, <_>::default(), script, "", result.data);
    let result = checked_call_vm!(vm_b, <_>::default(), script, "", result.data);
    let result = checked_call_vm!(vm_a, <_>::default(), script, "", result.data);
    let result = checked_call_vm!(vm_b, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_trace = ExecutionTrace::from(vec![
        scalar!(json!(["1", "2"]), peer = "set_variable"),
        scalar!(json!(["1", "2"]), peer = "set_variable"),
        scalar!("1", peer = "A", args = ["1"]),
        unused!("1", peer = "B", args = ["1"]),
        scalar!("1", peer = "A", args = ["1"]),
        unused!("1", peer = "B", args = ["1"]),
        par(1, 1),
        scalar!("1", peer = "A", args = ["1"]),
        unused!("1", peer = "B", args = ["1"]),
        scalar!("2", peer = "A", args = ["2"]),
        unused!("2", peer = "B", args = ["2"]),
        request_sent_by("B"),
    ]);

    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn shadowing_scope() {
    use executed_state::*;

    fn execute_script(script: String) -> Result<RawAVMOutcome, String> {
        let mut set_variables_vm = create_avm(set_variable_call_service(json!(["1", "2"])), "set_variable");
        let mut vm_a = create_avm(echo_call_service(), "A");
        let mut vm_b = create_avm(echo_call_service(), "B");

        let result = checked_call_vm!(set_variables_vm, <_>::default(), script.clone(), "", "");
        let result = checked_call_vm!(vm_a, <_>::default(), script.clone(), "", result.data);
        let result = checked_call_vm!(vm_b, <_>::default(), script.clone(), "", result.data);
        let result = checked_call_vm!(vm_a, <_>::default(), script.clone(), "", result.data);
        let result = checked_call_vm!(vm_b, <_>::default(), script.clone(), "", result.data);

        vm_a.call(script, "", result.data, <_>::default())
    }

    let variable_shadowing_script = r#"
            (seq
                (seq
                    (call "set_variable" ("" "") [] iterable1)
                    (call "set_variable" ("" "") [] iterable2)
                )
                (fold iterable1 i
                    (seq
                        (seq
                            (call "A" ("" "") ["value"] local_j)
                            (seq
                                (fold iterable2 j
                                    (seq
                                        (seq
                                            (call "A" ("" "") [i] local_j)
                                            (call "B" ("" "") [local_j])
                                        )
                                        (next j)
                                    )
                                )
                                (call "A" ("" "") [local_j])
                            )
                        )
                        (next i)
                    )
                )
            )"#;

    let result = execute_script(String::from(variable_shadowing_script)).unwrap();

    let actual_trace = trace_from_result(&result);
    let expected_trace = ExecutionTrace::from(vec![
        scalar!(vec!["1", "2"], peer = "set_variable"),
        scalar!(vec!["1", "2"], peer = "set_variable"),
        scalar!("value", peer = "A", args = ["value"]),
        scalar!("1", peer = "A", args = ["1"]),
        unused!("1", peer = "B", args = ["1"]),
        scalar!("1", peer = "A", args = ["1"]),
        unused!("1", peer = "B", args = ["1"]),
        unused!("value", peer = "A", args = ["value"]),
        scalar!("value", peer = "A", args = ["value"]),
        scalar!("2", peer = "A", args = ["2"]),
        request_sent_by("A"),
    ]);

    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_waits_on_empty_stream() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
            (par
                (call "" ("" "") [] $stream)
                (fold $stream iterator
                    (seq
                        (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                        (next iterator))))
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![executed_state::par(1, 0), executed_state::request_sent_by(vm_peer_id)];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_stream_seq_next_never_completes() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] $stream)
                (seq
                    (fold $stream iterator
                        (seq
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)))
                    (call "{vm_peer_id}" ("" "") [])))
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        stream!(1, 0, peer = vm_peer_id),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        stream!(1, 0, peer = vm_peer_id, args = [1]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_stream_seq_next_never_completes_with_never() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] $stream)
                (seq
                    (fold $stream iterator
                        (seq
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)
                        )
                        (never)
                    )
                    (call "{vm_peer_id}" ("" "") [])
                )
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        stream!(1, 0, peer = vm_peer_id),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        stream!(1, 0, peer = vm_peer_id, args = [1]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_stream_seq_next_completes_with_null() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] $stream)
                (seq
                    (fold $stream iterator
                        (seq
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)
                        )
                        (null)
                    )
                    (call "{vm_peer_id}" ("" "") [])
                )
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        stream!(1, 0, peer = vm_peer_id),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        stream!(1, 0, peer = vm_peer_id, args = [1]),
        unused!(1, peer = vm_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_scalar_seq_next_completes_with_null() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([1, 2]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] iterable)
                (seq
                    (fold iterable iterator
                        (par
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)
                        )
                        (null)
                    )
                    (seq
                        (canon "{vm_peer_id}" $new_stream #canon_stream)
                        (call "{vm_peer_id}" ("" "") [#canon_stream])
                    )
                )
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = ExecutionTrace::from(vec![
        scalar!(service_result.clone(), peer = vm_peer_id),
        executed_state::par(1, 2),
        stream!(service_result.clone(), 0, peer = vm_peer_id, args = [1]),
        executed_state::par(1, 0),
        stream!(service_result.clone(), 0, peer = vm_peer_id, args = [2]),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_peer_id", "service_id": ""}, "values": []}),
        ),
        unused!(service_result, peer = vm_peer_id, args = [json!([])]),
    ]);
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_scalar_seq_next_not_completes_with_never() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([1, 2]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] iterable)
                (seq
                    (fold iterable iterator
                        (par
                            (call "unknwon_peer_id" ("" "") [iterator] $new_stream)
                            (next iterator)
                        )
                        (never)
                    )
                    (seq
                        (canon "{vm_peer_id}" $new_stream #canon_stream)
                        (call "{vm_peer_id}" ("" "") [#canon_stream])
                    )
                )
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        scalar!(service_result, peer = vm_peer_id),
        executed_state::par(1, 2),
        executed_state::request_sent_by(vm_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(vm_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_stream_seq_next_saves_call_result() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (ap 1 $stream)
                    (ap 2 $stream)
                )
                (seq
                    (fold $stream iterator
                        (seq
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)
                        )
                        (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                    )
                    (call "{vm_peer_id}" ("" "") [0])
                )
            )
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(3.into(), 1), SubTraceDesc::new(6.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(4.into(), 1), SubTraceDesc::new(5.into(), 1)),
        ]),
        stream!(1, 0, peer = vm_peer_id, args = [1]),
        stream!(2, 1, peer = vm_peer_id, args = [2]),
        stream!(2, 2, peer = vm_peer_id, args = [2]),
        unused!(0, peer = vm_peer_id, args = [0]),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_par_next_completes() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(set_variable_call_service(json!(1)), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(set_variable_call_service(json!(1)), vm_2_peer_id);

    let vm_3_peer_id = "vm_3_peer_id";
    let mut vm_3 = create_avm(set_variable_call_service(json!(1)), vm_3_peer_id);

    let vm_4_peer_id = "vm_4_peer_id";
    let mut vm_4 = create_avm(set_variable_call_service(json!(1)), vm_4_peer_id);

    let script = format!(
        r#"
            (seq
                (seq
                    (seq
                        (ap "{vm_2_peer_id}" $stream)
                        (ap "{vm_3_peer_id}" $stream))
                    (ap "{vm_4_peer_id}" $stream))
                (seq
                    (fold $stream peer_id
                        (par
                            (call peer_id ("" "") [] $new_stream)
                            (next peer_id)))
                    (call "{vm_1_peer_id}" ("" "") []) ; this call should be executed if any of these three peers is reached
                )
            )
            "#
    );

    let result_1 = checked_call_vm!(vm_1, <_>::default(), &script, "", "");

    let result_2 = checked_call_vm!(vm_2, <_>::default(), &script, "", result_1.data.clone());
    let actual_trace = trace_from_result(&result_2);
    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(4.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(6.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(2, SubTraceDesc::new(8.into(), 2), SubTraceDesc::new(10.into(), 0)),
        ]),
        executed_state::par(1, 4),
        stream!(1, 0, peer = vm_2_peer_id),
        executed_state::par(1, 2),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::request_sent_by(vm_2_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);

    let result_3 = checked_call_vm!(vm_3, <_>::default(), &script, "", result_1.data.clone());
    let actual_trace = trace_from_result(&result_3);
    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(4.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(6.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(2, SubTraceDesc::new(8.into(), 2), SubTraceDesc::new(10.into(), 0)),
        ]),
        executed_state::par(1, 4),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::par(1, 2),
        stream!(1, 0, peer = vm_3_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::request_sent_by(vm_3_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);

    let result_4 = checked_call_vm!(vm_4, <_>::default(), &script, "", result_1.data);
    let actual_trace = trace_from_result(&result_4);
    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(4.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(6.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(2, SubTraceDesc::new(8.into(), 2), SubTraceDesc::new(10.into(), 0)),
        ]),
        executed_state::par(1, 4),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::par(1, 2),
        executed_state::request_sent_by(vm_1_peer_id),
        executed_state::par(1, 0),
        stream!(1, 0, peer = vm_4_peer_id),
        executed_state::request_sent_by(vm_4_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
fn fold_stream_map() {
    let vm_1_peer_id = "vm_1_peer_id";
    let k1 = 42;
    let k2 = "some";
    let arg_tetraplets = Rc::new(RefCell::new(vec![]));
    let arg_tetraplets_inner = arg_tetraplets.clone();

    let set_variable_call_service: CallServiceClosure = Box::new(move |params| -> CallServiceResult {
        arg_tetraplets_inner.borrow_mut().push(params.tetraplets.clone());
        CallServiceResult::ok(json!({"keyo": k1, "keyu": k2}))
    });

    let mut vm_1 = create_avm(set_variable_call_service, vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (seq
                        (call "{vm_1_peer_id}" ("m1" "f1") [1] scalar)
                        (call "{vm_1_peer_id}" ("m2" "f2") [1] $stream)
                    )
                    (canon "{vm_1_peer_id}" $stream #canon)
                )
                (seq
                    (ap (#canon.$.[0].keyo #canon.$.[0].keyo) %map)
                    (ap (scalar.$.keyu scalar.$.keyu) %map)
                )
            )
            (fold %map i
                (seq
                    (seq
                        (call "{vm_1_peer_id}" ("m3" "f3") [i.$.key] u)
                        (call "{vm_1_peer_id}" ("m4" "f4") [i.$.value] un)
                    )
                    (next i)
                )
            )
        )
        "#
    );

    let test_params = TestRunParameters::from_init_peer_id(vm_1_peer_id);
    let result = checked_call_vm!(vm_1, test_params, &script, "", "");
    let actual_trace = trace_from_result(&result);

    let generation_idx = 0;
    let mut cid_tracker = ExecutionCidState::new();
    let service_result = json!({"keyo": k1, "keyu": k2});

    let stream_1 = stream_tracked!(
        service_result.clone(),
        0,
        cid_tracker,
        peer = vm_1_peer_id,
        service = "m2",
        function = "f2",
        args = [1]
    );
    let cid_1 = extract_service_result_cid(&stream_1);

    let expected_state = ExecutionTrace::from(vec![
        scalar_tracked!(
            service_result.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m1",
            function = "f1",
            args = [1]
        ),
        stream_1,
        executed_state::canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""},
                "values": [{
                    "result": service_result.clone(),
                    "tetraplet": {"function_name": "f2", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": "m2"},
                    "provenance": Provenance::service_result(cid_1),
                }]
            }),
            &mut cid_tracker,
        ),
        executed_state::ap(generation_idx),
        executed_state::ap(generation_idx),
        executed_state::fold(vec![
            subtrace_lore(3, SubTraceDesc::new(6.into(), 2), SubTraceDesc::new(10.into(), 0)),
            subtrace_lore(4, SubTraceDesc::new(8.into(), 2), SubTraceDesc::new(10.into(), 0)),
        ]),
        scalar_tracked!(
            service_result.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m3",
            function = "f3",
            args = [k1]
        ),
        scalar_tracked!(
            service_result.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m4",
            function = "f4",
            args = [k1]
        ),
        scalar_tracked!(
            service_result.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m3",
            function = "f3",
            args = [k2]
        ),
        scalar_tracked!(
            service_result,
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m4",
            function = "f4",
            args = [k2]
        ),
    ]);
    assert_eq!(actual_trace, expected_state);

    let expected_tetraplets = vec![
        vec![vec![SecurityTetraplet::new(vm_1_peer_id, "m2", "f2", ".$.key")]],
        vec![vec![SecurityTetraplet::new(vm_1_peer_id, "m2", "f2", ".$.value")]],
        vec![vec![SecurityTetraplet::new(vm_1_peer_id, "m1", "f1", ".$.keyu.$.key")]],
        vec![vec![SecurityTetraplet::new(
            vm_1_peer_id,
            "m1",
            "f1",
            ".$.keyu.$.value",
        )]],
    ];
    let tetraplates_len = arg_tetraplets.borrow().len();
    assert_eq!(&arg_tetraplets.borrow()[tetraplates_len - 4..], &expected_tetraplets);
}

#[tokio::test]
fn fold_canon_stream_map() {
    let vm_1_peer_name = "vm_1_peer_id";
    let vm_1_peer_id = at(vm_1_peer_name);

    let script = format!(
        r#"
        (seq
            (seq
                (ap ("key" "value1") %map)
                (ap (-42 "value2") %map)
            )
            (seq
                (canon "{vm_1_peer_name}" %map #%canon_map)
                (fold #%canon_map iter
                    (seq
                        (call "{vm_1_peer_name}" ("m" "f") [iter] scalar) ; behaviour = echo
                        (next iter)
                    )
                )
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_name).unwrap();

    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let map_value_1 = json!({"key": "key", "value": "value1"});
    let map_value_2 = json!({"key": -42, "value": "value2"});

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
                {
                "result": map_value_1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_2,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            map_value_1.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_1]
        ),
        scalar_tracked!(
            map_value_2.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_2]
        ),
    ];

    assert_eq!(&*actual_trace, expected_trace,);
}

/// This test checks that fold over map and fold over canon map both produce
/// the same kvpairs sequences. Please note that call results produced by
/// the folds mentioned differ in their tetraplets b/c testing framework
/// increments service name index for each call used.
#[tokio::test]
fn fold_map_and_canon_map_orders_are_same() {
    let vm_1_peer_name = "vm_1_peer_id";
    let vm_1_peer_id = at(vm_1_peer_name);

    let script = format!(
        r#"
        (seq
            (seq
                (seq
                    (ap ("key" "value1") %map)
                    (ap (-42 "value2") %map)
                )
                (seq
                    (ap (42 "value3") %map)
                    (ap ("other" "value4") %map)
                )
            )
            (seq
                (seq
                    (canon "{vm_1_peer_name}" %map #%canon_map)
                    (fold #%canon_map iter
                        (seq
                            (call "{vm_1_peer_name}" ("m" "f") [iter] scalar) ; behaviour = echo
                            (next iter)
                        )
                    )
                )
                (fold %map iter
                    (seq
                        (call "{vm_1_peer_name}" ("m" "f") [iter] scalar1) ; behaviour = echo
                        (next iter)
                    )
                )
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_name), &script)
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_name).unwrap();

    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let map_value_1 = json!({"key": "key", "value": "value1"});
    let map_value_2 = json!({"key": -42, "value": "value2"});
    let map_value_3 = json!({"key": 42, "value": "value3"});
    let map_value_4 = json!({"key": "other", "value": "value4"});

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
            {
                "result": map_value_1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_2,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_3,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_4,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            map_value_1.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_1.clone()]
        ),
        scalar_tracked!(
            map_value_2.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_2.clone()]
        ),
        scalar_tracked!(
            map_value_3.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_3.clone()]
        ),
        scalar_tracked!(
            map_value_4.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value_4.clone()]
        ),
        fold(vec![
            subtrace_lore(0, subtrace_desc(10, 1), subtrace_desc(14, 0)),
            subtrace_lore(1, subtrace_desc(11, 1), subtrace_desc(14, 0)),
            subtrace_lore(2, subtrace_desc(12, 1), subtrace_desc(14, 0)),
            subtrace_lore(3, subtrace_desc(13, 1), subtrace_desc(14, 0)),
        ]),
        scalar_tracked!(
            map_value_1.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..1",
            function = "f",
            args = [map_value_1]
        ),
        scalar_tracked!(
            map_value_2.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..1",
            function = "f",
            args = [map_value_2]
        ),
        scalar_tracked!(
            map_value_3.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..1",
            function = "f",
            args = [map_value_3]
        ),
        scalar_tracked!(
            map_value_4.clone(),
            cid_tracker,
            peer = &vm_1_peer_id,
            service = "m..1",
            function = "f",
            args = [map_value_4]
        ),
    ];

    assert_eq!(&*actual_trace, expected_trace);
}
