/*
 * Copyright 2023 Fluence Labs Limited
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
use air::UncatchableError;
use air_interpreter_data::FoldSubTraceLore;
use air_interpreter_data::ParResult;
use air_interpreter_data::SubTraceDesc;
use air_test_utils::prelude::*;
use air_trace_handler::merger::ApResultError;
use air_trace_handler::merger::CallResultError;
use air_trace_handler::merger::CanonResultError;
use air_trace_handler::merger::FoldResultError;
use air_trace_handler::merger::MergeCtxType::Current;
use air_trace_handler::merger::MergeCtxType::Previous;
use air_trace_handler::merger::ResolvedFold;
use air_trace_handler::merger::ResolvedSubTraceDescs;
use air_trace_handler::KeeperError::*;
use air_trace_handler::StateFSMError::*;
use air_trace_handler::TraceHandlerError::KeeperError;
use air_trace_handler::TraceHandlerError::MergeError;
use air_trace_handler::TraceHandlerError::StateFSMError;
use maplit::hashmap;

#[test]
fn par_len_overflow() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);

    let script = format!(
        r#"
        (par
            (ap 42 some)
            (call "other" ("" "") [some] other)
        )
    "#
    );

    let error_left_pos_value = 1;
    let error_right_pos_value = PosType::MAX;
    let trace = vec![
        executed_state::par(error_left_pos_value, error_right_pos_value),
        executed_state::request_sent_by(vm_peer_id_1),
    ];
    let data = raw_data_from_trace(trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), script, "", data);
    let expected_error = UncatchableError::TraceError {
        trace_error: StateFSMError(ParLenOverflow(ParResult::new(
            error_left_pos_value,
            error_right_pos_value,
        ))),
        instruction: "par".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn par_pos_overflow() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);

    let script = format!(
        r#"
        (par
            (ap 42 some)
            (call "other" ("" "") [some] other)
        )
    "#
    );

    let error_pos_value = PosType::MAX;
    let trace = vec![
        executed_state::par(error_pos_value, error_pos_value),
        executed_state::request_sent_by(vm_peer_id_1),
    ];
    let data = raw_data_from_trace(trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), script, "", data);
    let expected_error = UncatchableError::TraceError {
        trace_error: StateFSMError(ParPosOverflow(
            ParResult::new(error_pos_value, error_pos_value),
            1.into(),
            Previous,
        )),
        instruction: "par".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn par_len_underflow() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);

    let script = format!(
        r#"
        (par
            (ap 42 some)
            (call "other" ("" "") [some] other)
        )
    "#
    );

    let trace = vec![
        executed_state::par(42, 1),
        executed_state::request_sent_by(vm_peer_id_1),
    ];
    let data = raw_data_from_trace(trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), script, "", data);
    let expected_error = UncatchableError::TraceError {
        trace_error: StateFSMError(ParLenUnderflow(ParResult::new(42, 1), 1, Current)),
        instruction: "par".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn set_subtrace_len_and_pos_failed() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([42, 43]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, cid_state, peer = vm_peer_id_1),
        executed_state::fold(vec![executed_state::subtrace_lore(
            1,
            subtrace_desc(5, 1),
            subtrace_desc(4, 0),
        )]),
        request_sent_by("vm_peer_id_1"),
    ];
    let wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), script, wrong_data, "");
    let expected_error = UncatchableError::TraceError {
        trace_error: KeeperError(SetSubtraceLenAndPosFailed {
            requested_pos: 5.into(),
            requested_subtrace_len: 1,
            trace_len: 4,
        }),
        instruction: "fold $s i".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn no_element_at_position() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, cid_state, peer = vm_peer_id_1),
        executed_state::fold(vec![executed_state::subtrace_lore(
            42,
            subtrace_desc(3, 1),
            subtrace_desc(4, 0),
        )]),
        request_sent_by("vm_peer_id_1"),
    ];
    let wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), script, wrong_data, "");
    let expected_error = UncatchableError::TraceError {
        trace_error: air_trace_handler::TraceHandlerError::KeeperError(NoElementAtPosition {
            position: 42.into(),
            trace_len: 4,
        }),
        instruction: "fold $s i".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn no_stream_state() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([42, 43]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut tracker = ExecutionCidState::new();
    let wrong_state = request_sent_by("vm_peer_id_1");
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, &mut tracker, peer = vm_peer_id_1),
        executed_state::fold(vec![executed_state::subtrace_lore(
            3,
            subtrace_desc(3, 1), // try to change the number of elems to 3
            subtrace_desc(4, 0),
        )]),
        wrong_state.clone(),
    ];
    let wrong_data = raw_data_from_trace(trace, tracker);
    let result = call_vm!(peer_vm_1, <_>::default(), script, wrong_data, "");
    let expected_error = UncatchableError::TraceError {
        trace_error: air_trace_handler::TraceHandlerError::KeeperError(NoStreamState { state: wrong_state }),
        instruction: "fold $s i".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn incompatible_executed_states() {
    let vm_peer_id = "vm_peer_id";
    let mut peer_vm_1 = create_avm(echo_call_service(), vm_peer_id);
    let script = format!(
        r#"
        (seq
            (call "{vm_peer_id}" ("" "") [] scalar)
            (ap scalar $stream)
        )
    "#
    );
    let mut cid_tracker = ExecutionCidState::new();
    let prev_trace = vec![
        scalar_tracked!("", cid_tracker, peer = vm_peer_id),
        executed_state::ap(1),
    ];
    let current_trace = vec![scalar!("", peer = vm_peer_id), scalar!("", peer = vm_peer_id)];
    let prev_data = raw_data_from_trace(prev_trace, cid_tracker.clone().into());
    let current_data = raw_data_from_trace(current_trace, cid_tracker.into());
    let result = call_vm!(peer_vm_1, <_>::default(), &script, prev_data, current_data);

    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::IncompatibleExecutedStates(
            ExecutedState::Ap(ApResult::new(1.into())),
            scalar!("", peer = vm_peer_id),
        )),
        instruction: "ap scalar $stream".to_string(),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn different_executed_state_expected() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([42, 43]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);
    let script = format!(
        r#"
        (seq
            (ap 42 some)
            (call "vm_peer_id_2" ("" "") [] $s)
        )
    "#
    );
    let wrong_state = executed_state::ap(42);
    let prev_trace = vec![wrong_state.clone()];
    let prev_data = raw_data_from_trace(prev_trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), &script, prev_data, "");
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::DifferentExecutedStateExpected(
            wrong_state,
            air_trace_handler::DataType::Previous,
            "call",
        )),
        instruction: String::from(r#"call "vm_peer_id_2" ("" "") [] $s"#),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn invalid_dst_generations() {
    use air_interpreter_sede::Format;

    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (ap "a" $s)
    "#
    );

    let empty_data = InterpreterData::from_execution_result(
        <_>::default(),
        <_>::default(),
        <_>::default(),
        <_>::default(),
        semver::Version::new(1, 1, 1),
    );
    let mut data_value = serde_json::to_value(&empty_data).unwrap();
    data_value["trace"] = json!([{"ap": {"gens": [42, 42]}}]);

    let data = InterpreterDataRepr::get_format::<JValue>().to_vec(&data_value).unwrap();
    // let result = peer_vm_1.call(script, "", data, <_>::default()).unwrap();
    let result = call_vm!(peer_vm_1, <_>::default(), &script, "", data);
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::MergeError::IncorrectApResult(
            ApResultError::InvalidDstGenerations(ApResult {
                res_generations: vec![42.into(), 42.into()],
            }),
        )),
        instruction: String::from(r#"ap "a" $s"#),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn incorrect_call_result() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (call "vm_peer_id_1" ("" "") [] v)
    "#
    );
    let prev_call_result = failed!(42, "some", peer = vm_peer_id_1);
    let prev_trace = vec![prev_call_result.clone()];
    let prev_data = raw_data_from_trace(prev_trace, <_>::default());
    let curr_call_result = scalar!("some", peer = vm_peer_id_1);
    let curr_trace = vec![curr_call_result.clone()];
    let curr_data = raw_data_from_trace(curr_trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), &script, prev_data, curr_data);

    let mut cid_store = ExecutionCidState::new();
    let curr_call_value_ref = ValueRef::Scalar(value_aggregate_cid(
        json!("some"),
        SecurityTetraplet::literal_tetraplet(vm_peer_id_1),
        vec![],
        &mut cid_store,
    ));
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::MergeError::IncorrectCallResult(
            CallResultError::IncompatibleCallResults {
                prev_call: air_interpreter_data::CallResult::Failed(value_aggregate_cid(
                    serde_json::to_value(CallServiceFailed::new(42, r#""some""#.to_owned().into())).unwrap(),
                    SecurityTetraplet::literal_tetraplet(vm_peer_id_1),
                    vec![],
                    &mut cid_store,
                )),
                current_call: air_interpreter_data::CallResult::Executed(curr_call_value_ref),
            },
        )),
        instruction: String::from(r#"call "vm_peer_id_1" ("" "") [] v"#),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn canon_result_error() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([42, 43]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg.clone()), vm_peer_id_1);
    let script = format!(
        r#"
        (canon "vm_peer_id_1" $stream #canon)
    "#
    );
    let prev_tetraplet = json!({
        "tetraplet": {"function_name": "s", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},
        "values": [
            {
                "result": 42,
                "tetraplet": {"function_name": "s", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},
            },
        ]
    });
    let prev_trace = vec![executed_state::canon(prev_tetraplet)];
    let prev_cid = extract_canon_result_cid(&prev_trace[0]);
    let prev_data = raw_data_from_trace(prev_trace, <_>::default());
    let curr_tetraplet = json!({
        "tetraplet": {"function_name": "s", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},
        "values": [
            {
                "result": 43,
                "tetraplet": {"function_name": "s", "json_path": "", "peer_pk": "vm_peer_id_1", "service_id": ""},
            },
        ]
    });

    let curr_trace = vec![executed_state::canon(curr_tetraplet)];
    let curr_cid = extract_canon_result_cid(&curr_trace[0]);
    let curr_data = raw_data_from_trace(curr_trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), &script, prev_data, curr_data);

    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::MergeError::IncorrectCanonResult(
            CanonResultError::IncompatibleState {
                prev_canon_result: air_interpreter_data::CanonResult::executed(prev_cid),
                current_canon_result: air_interpreter_data::CanonResult::executed(curr_cid),
            },
        )),
        instruction: String::from(r#"canon "vm_peer_id_1" $stream #canon"#),
    };
    assert!(check_error(&result, expected_error), "{:?}", result);
}

#[test]
fn several_records_with_same_pos() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let value_pos = 1;
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, &mut cid_state, peer = vm_peer_id_1),
        fold(vec![
            subtrace_lore(value_pos, subtrace_desc(3, 1), subtrace_desc(4, 0)),
            subtrace_lore(value_pos, subtrace_desc(3, 1), subtrace_desc(4, 0)),
        ]),
        request_sent_by("vm_peer_id_1"),
    ];
    let wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), &script, wrong_data, "");
    let fold_lore = FoldSubTraceLore {
        value_pos: value_pos.into(),
        subtraces_desc: vec![
            SubTraceDesc {
                begin_pos: 3.into(),
                subtrace_len: 1,
            },
            SubTraceDesc {
                begin_pos: 4.into(),
                subtrace_len: 0,
            },
        ],
    };
    let fold_result = FoldResult {
        lore: vec![fold_lore.clone(), fold_lore],
    };
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::IncorrectFoldResult(
            FoldResultError::SeveralRecordsWithSamePos(fold_result, value_pos.into()),
        )),
        instruction: String::from(String::from("fold $s i")),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn values_not_equal() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let arg = json!([42, 43]);
    let mut peer_vm_1 = create_avm(set_variable_call_service(arg), vm_peer_id_1);
    let script = format!(
        r#"
        (call "vm_peer_id_1" ("" "") [] $s)
    "#
    );
    let prev_value = json!(42);
    let prev_trace = vec![scalar!(prev_value.clone(), peer = vm_peer_id_1)];
    let prev_data = raw_data_from_trace(prev_trace, <_>::default());
    let curr_value = json!(43);
    let curr_trace = vec![scalar!(curr_value.clone(), peer = vm_peer_id_1)];
    let curr_data = raw_data_from_trace(curr_trace, <_>::default());
    let result = call_vm!(peer_vm_1, <_>::default(), &script, prev_data, curr_data);

    let mut cid_state = ExecutionCidState::new();
    let prev_value = ValueRef::Scalar(value_aggregate_cid(
        prev_value,
        SecurityTetraplet::literal_tetraplet(vm_peer_id_1),
        vec![],
        &mut cid_state,
    ));
    let current_value = ValueRef::Scalar(value_aggregate_cid(
        curr_value,
        SecurityTetraplet::literal_tetraplet(vm_peer_id_1),
        vec![],
        &mut cid_state,
    ));
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::IncorrectCallResult(
            CallResultError::ValuesNotEqual {
                prev_value,
                current_value,
            },
        )),
        instruction: String::from(format!(r#"call "{vm_peer_id_1}" ("" "") [] $s"#)),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn fold_pos_overflow() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let value_pos = 1;
    let before_subtrace_pos = 3;
    let after_subtrace_pos = 4;
    let wrong_after_subtrace_len = TraceLen::MAX - 1;
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, cid_state, peer = vm_peer_id_1),
        fold(vec![subtrace_lore(
            value_pos,
            subtrace_desc(before_subtrace_pos, 1),
            subtrace_desc(after_subtrace_pos, wrong_after_subtrace_len),
        )]),
        request_sent_by("vm_peer_id_1"),
    ];
    let wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), &script, wrong_data, "");
    let fold_lore = ResolvedSubTraceDescs::new(
        SubTraceDesc {
            begin_pos: before_subtrace_pos.into(),
            subtrace_len: wrong_after_subtrace_len + 1,
        },
        SubTraceDesc {
            begin_pos: after_subtrace_pos.into(),
            subtrace_len: wrong_after_subtrace_len,
        },
    );
    let resolved_fold = ResolvedFold::new(hashmap!(value_pos.into() => fold_lore), wrong_after_subtrace_len + 1);
    let expected_error = UncatchableError::TraceError {
        trace_error: StateFSMError(FoldPosOverflow(resolved_fold, before_subtrace_pos.into(), Previous)),
        instruction: String::from(String::from("fold $s i")),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn subtrace_len_overflow() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let value_pos = 1;
    let wrong_subtrace_len = TraceLen::MAX;
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, cid_state, peer = vm_peer_id_1),
        executed_state::fold(vec![executed_state::subtrace_lore(
            value_pos,
            subtrace_desc(3, 1),
            subtrace_desc(4, wrong_subtrace_len),
        )]),
        request_sent_by("vm_peer_id_1"),
    ];
    let _wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), script, _wrong_data, "");

    let fold_lore = FoldSubTraceLore {
        value_pos: value_pos.into(),
        subtraces_desc: vec![
            SubTraceDesc {
                begin_pos: 3.into(),
                subtrace_len: 1,
            },
            SubTraceDesc {
                begin_pos: 4.into(),
                subtrace_len: wrong_subtrace_len,
            },
        ],
    };
    let fold_result = FoldResult { lore: vec![fold_lore] };
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::IncorrectFoldResult(
            FoldResultError::SubtraceLenOverflow { fold_result, count: 0 },
        )),
        instruction: String::from(String::from("fold $s i")),
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn fold_incorrect_subtraces_count() {
    let vm_peer_id_1 = "vm_peer_id_1";
    let mut peer_vm_1 = create_avm(unit_call_service(), vm_peer_id_1);
    let script = format!(
        r#"
        (par
            (call "vm_peer_id_1" ("" "") [] $s)
            (fold $s i
                (call "vm_peer_id_2" ("" "") [] a)
                (next i)
            )
        )
    "#
    );
    let mut cid_state = ExecutionCidState::new();
    let value_pos = 1;
    let wrong_subtrace_lore = vec![subtrace_desc(3, 1), subtrace_desc(4, 0), subtrace_desc(4, 0)];
    let trace = vec![
        executed_state::par(1, 2),
        stream_tracked!(json!([42, 43]), 0, cid_state, peer = vm_peer_id_1),
        executed_state::fold(vec![FoldSubTraceLore {
            value_pos: value_pos.into(),
            subtraces_desc: wrong_subtrace_lore.clone(),
        }]),
        request_sent_by("vm_peer_id_1"),
    ];
    let _wrong_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(peer_vm_1, <_>::default(), script, _wrong_data, "");
    let expected_error = UncatchableError::TraceError {
        trace_error: MergeError(air_trace_handler::merger::MergeError::IncorrectFoldResult(
            FoldResultError::FoldIncorrectSubtracesCount(wrong_subtrace_lore.len()),
        )),
        instruction: String::from(String::from("fold $s i")),
    };
    assert!(check_error(&result, expected_error));
}
