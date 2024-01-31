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

use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

use std::ops::Deref;

#[test]
fn empty_stream() {
    fn arg_type_check_closure() -> CallServiceClosure {
        Box::new(move |params| -> CallServiceResult {
            let actual_call_args: Vec<Vec<serde_json::Value>> =
                serde_json::from_value(serde_json::Value::Array(params.arguments))
                    .expect("json deserialization shouldn't fail");
            let expected_call_args: Vec<Vec<serde_json::Value>> = vec![vec![]];

            assert_eq!(actual_call_args, expected_call_args);

            CallServiceResult::ok(json!(""))
        })
    }

    let vm_peer_id = "vm_peer_id";
    let mut vm = create_avm(arg_type_check_closure(), vm_peer_id);

    let script = format!(
        r#"
        (seq
            (par
                (call "unknown_peer" ("" "") [] $stream) ; to make validator happy
                (seq
                    (canon "{vm_peer_id}" $stream #canon_stream)
                    (call "{vm_peer_id}" ("" "") [#canon_stream] $other_stream)
                )
            )
            (null)
        )"#
    );

    let _ = checked_call_vm!(vm, <_>::default(), script, "", "");
}

#[test]
fn stream_merging_v0() {
    let initiator_id = "initiator_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";
    let executor_id = "stream_executor";

    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1")), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2")), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3")), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v0.air"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, <_>::default(), &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, <_>::default(), &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, <_>::default(), &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace_1 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(17, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(19, 2), subtrace_desc(21, 0)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
    ]);
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_1.data,
        setter_2_res.data
    );
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(17, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(19, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(8, subtrace_desc(21, 2), subtrace_desc(25, 0)),
            executed_state::subtrace_lore(13, subtrace_desc(23, 2), subtrace_desc(25, 0)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
    ];
    assert_eq!(actual_trace_2.deref(), expected_trace_2);

    let executor_result_3 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_2.data,
        setter_3_res.data
    );
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(17, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(19, 2), subtrace_desc(21, 0)),
            executed_state::subtrace_lore(8, subtrace_desc(21, 2), subtrace_desc(25, 0)),
            executed_state::subtrace_lore(13, subtrace_desc(23, 2), subtrace_desc(25, 0)),
            executed_state::subtrace_lore(10, subtrace_desc(25, 2), subtrace_desc(29, 0)),
            executed_state::subtrace_lore(11, subtrace_desc(27, 2), subtrace_desc(29, 0)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
    ]);
    assert_eq!(actual_trace_3, expected_trace_3);
}

#[test]
fn stream_merging_v1() {
    let initiator_id = "initiator_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";
    let executor_id = "stream_executor";

    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1")), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2")), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3")), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v1.air"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, <_>::default(), &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, <_>::default(), &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, <_>::default(), &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace_1 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(18, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(21, 3), subtrace_desc(24, 0)),
        ]),
        executed_state::par(2, 6),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
    ]);
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_1.data,
        setter_2_res.data
    );
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(18, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(21, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(8, subtrace_desc(24, 3), subtrace_desc(30, 0)),
            executed_state::subtrace_lore(13, subtrace_desc(27, 3), subtrace_desc(30, 0)),
        ]),
        executed_state::par(2, 6),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
    ]);
    assert_eq!(actual_trace_2, expected_trace_2);

    let executor_result_3 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_2.data,
        setter_3_res.data
    );
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(18, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(12, subtrace_desc(21, 3), subtrace_desc(24, 0)),
            executed_state::subtrace_lore(8, subtrace_desc(24, 3), subtrace_desc(30, 0)),
            executed_state::subtrace_lore(13, subtrace_desc(27, 3), subtrace_desc(30, 0)),
            executed_state::subtrace_lore(10, subtrace_desc(30, 3), subtrace_desc(36, 0)),
            executed_state::subtrace_lore(11, subtrace_desc(33, 3), subtrace_desc(36, 0)),
        ]),
        executed_state::par(2, 6),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        executed_state::par(2, 3),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        executed_state::par(2, 0),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
    ]);
    assert_eq!(actual_trace_3, expected_trace_3);
}

#[test]
#[ignore]
fn stream_merging_v2() {
    let initiator_id = "initiator_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";
    let executor_id = "stream_executor";

    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1")), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2")), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3")), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v2.air"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, <_>::default(), &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, <_>::default(), &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, <_>::default(), &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, <_>::default(), &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace_1 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 1), subtrace_desc(21, 2)),
            executed_state::subtrace_lore(9, subtrace_desc(16, 1), subtrace_desc(19, 2)),
            executed_state::subtrace_lore(12, subtrace_desc(17, 1), subtrace_desc(18, 2)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
    ]);
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_1.data,
        setter_2_res.data
    );
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 0), subtrace_desc(19, 2)),
            executed_state::subtrace_lore(9, subtrace_desc(15, 0), subtrace_desc(17, 2)),
            executed_state::subtrace_lore(12, subtrace_desc(15, 0), subtrace_desc(15, 2)),
            executed_state::subtrace_lore(8, subtrace_desc(21, 0), subtrace_desc(23, 2)),
            executed_state::subtrace_lore(13, subtrace_desc(21, 0), subtrace_desc(21, 2)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
    ]);
    assert_eq!(actual_trace_2, expected_trace_2);

    let executor_result_3 = checked_call_vm!(
        executor,
        <_>::default(),
        &script,
        executor_result_2.data,
        setter_3_res.data
    );
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = ExecutionTrace::from(vec![
        unused!(unit_call_service_result, peer = initiator_id),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("3", 2, peer = setter_3_id),
        stream!("1", 0, peer = setter_1_id),
        stream!("2", 1, peer = setter_2_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, subtrace_desc(15, 0), subtrace_desc(19, 2)),
            executed_state::subtrace_lore(9, subtrace_desc(15, 0), subtrace_desc(17, 2)),
            executed_state::subtrace_lore(12, subtrace_desc(15, 0), subtrace_desc(15, 2)),
            executed_state::subtrace_lore(8, subtrace_desc(21, 0), subtrace_desc(23, 2)),
            executed_state::subtrace_lore(13, subtrace_desc(21, 0), subtrace_desc(21, 2)),
            executed_state::subtrace_lore(10, subtrace_desc(25, 0), subtrace_desc(27, 2)),
            executed_state::subtrace_lore(11, subtrace_desc(25, 0), subtrace_desc(25, 2)),
        ]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["1"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["2"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
        unused!(unit_call_service_result, peer = executor_id, args = ["3"]),
    ]);
    assert_eq!(actual_trace_3, expected_trace_3);
}
