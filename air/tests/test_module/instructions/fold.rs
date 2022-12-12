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

use air::PreparationError;
use air::ToErrorCode;
use air_test_utils::prelude::*;

#[test]
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
    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{i}"), i as u32 - 1);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[test]
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

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{}", 6 - i), i as u32 - 1);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[test]
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

    let expected_state = executed_state::scalar_string_array(vec!["1", "2", "3", "4", "5"]);
    assert_eq!(actual_trace[0.into()], expected_state);
    assert_eq!(actual_trace[1.into()], expected_state);

    for i in 1..=5 {
        for j in 1..=5 {
            let state_id = 1 + 5 * (i - 1) + j;
            let expected_state = executed_state::stream_string(i.to_string(), state_id as u32 - 2);
            assert_eq!(actual_trace[state_id.into()], expected_state);
        }
    }
}

#[test]
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

#[test]
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
    let expected_state = executed_state::scalar(json!([]));

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0.into()], expected_state);
}

#[test]
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

#[test]
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
    let expected_trace = vec![executed_state::scalar(json!({ "messages": [] }))];

    assert_eq!(actual_trace, expected_trace);
}

// Check that fold works with the join behaviour without hanging up.
#[test]
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

#[test]
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
    let expected_state = executed_state::scalar(json!({ "array": ["1", "2", "3", "4", "5"] }));

    assert_eq!(actual_trace.len(), 6);
    assert_eq!(actual_trace[0.into()], expected_state);

    for i in 1..=5 {
        let expected_state = executed_state::stream_string(format!("{i}"), i as u32 - 1);
        assert_eq!(actual_trace[i.into()], expected_state);
    }
}

#[test]
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
    let expected_trace = vec![
        scalar_string_array(vec!["1", "2"]),
        scalar_string_array(vec!["1", "2"]),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        par(1, 1),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("2"),
        scalar_string("2"),
        request_sent_by("B"),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
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
    let expected_trace = vec![
        scalar_string_array(vec!["1", "2"]),
        scalar_string_array(vec!["1", "2"]),
        scalar_string("value"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("1"),
        scalar_string("value"),
        scalar_string("value"),
        scalar_string("2"),
        request_sent_by("A"),
    ];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_waits_on_empty_stream() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = f!(r#"
            (par
                (call "" ("" "") [] $stream)
                (fold $stream iterator
                    (seq
                        (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                        (next iterator))))
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![executed_state::par(1, 0), executed_state::request_sent_by(vm_peer_id)];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_stream_seq_next_never_completes() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = f!(r#"
            (seq
                (call "{vm_peer_id}" ("" "") [] $stream)
                (seq
                    (fold $stream iterator
                        (seq
                            (call "{vm_peer_id}" ("" "") [iterator] $new_stream)
                            (next iterator)))
                    (call "{vm_peer_id}" ("" "") [])))
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::stream_number(1, 0),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        executed_state::stream_number(1, 0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_stream_seq_next_never_completes_with_never() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = f!(r#"
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
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::stream_number(1, 0),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        executed_state::stream_number(1, 0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_stream_seq_next_completes_with_null() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(set_variable_call_service(json!(1)), vm_peer_id);

    let script = f!(r#"
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
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::stream_number(1, 0),
        executed_state::fold(vec![subtrace_lore(
            0,
            SubTraceDesc::new(2.into(), 1),
            SubTraceDesc::new(3.into(), 0),
        )]),
        executed_state::stream_number(1, 0),
        executed_state::scalar_number(1),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_scalar_seq_next_completes_with_null() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([1, 2]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = f!(r#"
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
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(service_result.clone()),
        executed_state::par(1, 2),
        executed_state::stream(service_result.clone(), 0),
        executed_state::par(1, 0),
        executed_state::stream(service_result.clone(), 0),
        executed_state::canon(
            json!({"tetraplet": {"function_name": "", "json_path": "", "peer_pk": "vm_peer_id", "service_id": ""}, "values": []}),
        ),
        executed_state::scalar(service_result),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_scalar_seq_next_not_completes_with_never() {
    let vm_peer_id = "vm_peer_id";
    let service_result = json!([1, 2]);
    let mut vm = create_avm(set_variable_call_service(service_result.clone()), vm_peer_id);

    let script = f!(r#"
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
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::scalar(service_result),
        executed_state::par(1, 2),
        executed_state::request_sent_by(vm_peer_id),
        executed_state::par(1, 0),
        executed_state::request_sent_by(vm_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_stream_seq_next_saves_call_result() {
    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = f!(r#"
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
            "#);

    let result = checked_call_vm!(vm, <_>::default(), &script, "", "");
    let actual_trace = trace_from_result(&result);

    let expected_trace = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        executed_state::fold(vec![
            subtrace_lore(0, SubTraceDesc::new(3.into(), 1), SubTraceDesc::new(6.into(), 0)),
            subtrace_lore(1, SubTraceDesc::new(4.into(), 1), SubTraceDesc::new(5.into(), 1)),
        ]),
        executed_state::stream_number(1, 0),
        executed_state::stream_number(2, 1),
        executed_state::stream_number(2, 2),
        executed_state::scalar_number(0),
    ];
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_par_next_completes() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(set_variable_call_service(json!(1)), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(set_variable_call_service(json!(1)), vm_2_peer_id);

    let vm_3_peer_id = "vm_3_peer_id";
    let mut vm_3 = create_avm(set_variable_call_service(json!(1)), vm_3_peer_id);

    let vm_4_peer_id = "vm_4_peer_id";
    let mut vm_4 = create_avm(set_variable_call_service(json!(1)), vm_4_peer_id);

    let script = f!(r#"
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
            "#);

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
        executed_state::stream_number(1, 0),
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
        executed_state::stream_number(1, 0),
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
        executed_state::stream_number(1, 0),
        executed_state::request_sent_by(vm_4_peer_id),
    ];
    assert_eq!(actual_trace, expected_trace);
}
