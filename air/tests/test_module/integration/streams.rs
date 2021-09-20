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

use air_test_utils::*;

use serde_json::json;
use serde_json::Value as JValue;

#[test]
fn empty_stream() {
    fn arg_type_check_closure() -> CallServiceClosure {
        Box::new(move |args| -> Option<IValue> {
            let call_args = match &args.function_args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            let actual_call_args: Vec<Vec<JValue>> =
                serde_json::from_str(call_args).expect("json deserialization shouldn't fail");
            let expected_call_args: Vec<Vec<JValue>> = vec![vec![]];

            assert_eq!(actual_call_args, expected_call_args);

            Some(IValue::Record(
                NEVec::new(vec![IValue::S32(0), IValue::String(r#""""#.to_string())]).unwrap(),
            ))
        })
    }

    let mut vm = create_avm(arg_type_check_closure(), "A");

    let script = r#"
        (seq
            (call "A" ("" "") [$stream] $other_stream)
            (null)
        )"#;

    let _ = checked_call_vm!(vm, "", script, "", "");
}

#[test]
fn stream_merging_v0() {
    let initiator_id = "initiator_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";
    let executor_id = "stream_executor";

    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1").to_string()), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2").to_string()), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3").to_string()), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v0.clj"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, "", &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, "", &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, "", &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, "", &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, "", &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let test_value = "test";
    let expected_trace_1 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(17, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(19, 2), SubTraceDesc::new(21, 0)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(executor, "", &script, executor_result_1.data.clone(), setter_2_res.data);
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(17, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(19, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 2), SubTraceDesc::new(25, 0)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(23, 2), SubTraceDesc::new(25, 0)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_2, expected_trace_2);

    let executor_result_3 = checked_call_vm!(executor, "", &script, executor_result_2.data.clone(), setter_3_res.data);
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(17, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(19, 2), SubTraceDesc::new(21, 0)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 2), SubTraceDesc::new(25, 0)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(23, 2), SubTraceDesc::new(25, 0)),
            executed_state::subtrace_lore(10, SubTraceDesc::new(25, 2), SubTraceDesc::new(29, 0)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(27, 2), SubTraceDesc::new(29, 0)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
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
    let mut setter_1 = create_avm(set_variable_call_service(json!("1").to_string()), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2").to_string()), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3").to_string()), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v1.clj"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, "", &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, "", &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, "", &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, "", &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, "", &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let test_value = "test";
    let expected_trace_1 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 1), SubTraceDesc::new(20, 1)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(16, 1), SubTraceDesc::new(19, 1)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(17, 1), SubTraceDesc::new(18, 1)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(executor, "", &script, executor_result_1.data.clone(), setter_2_res.data);
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 1), SubTraceDesc::new(20, 1)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(16, 1), SubTraceDesc::new(19, 1)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(17, 1), SubTraceDesc::new(18, 1)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 1), SubTraceDesc::new(24, 1)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(22, 1), SubTraceDesc::new(23, 1)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_2, expected_trace_2);

    let executor_result_3 = checked_call_vm!(executor, "", &script, executor_result_2.data.clone(), setter_3_res.data);
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 1), SubTraceDesc::new(20, 1)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(16, 1), SubTraceDesc::new(19, 1)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(17, 1), SubTraceDesc::new(18, 1)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 1), SubTraceDesc::new(24, 1)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(22, 1), SubTraceDesc::new(23, 1)),
            executed_state::subtrace_lore(10, SubTraceDesc::new(25, 1), SubTraceDesc::new(28, 1)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(26, 1), SubTraceDesc::new(27, 1)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_3, expected_trace_3);
}

#[test]
fn stream_merging_v2() {
    let initiator_id = "initiator_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";
    let executor_id = "stream_executor";

    let mut initiator = create_avm(unit_call_service(), initiator_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1").to_string()), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2").to_string()), setter_2_id);
    let mut setter_3 = create_avm(set_variable_call_service(json!("3").to_string()), setter_3_id);
    let mut executor = create_avm(unit_call_service(), executor_id);

    let script = format!(
        include_str!("scripts/stream_fold_merging_v2.clj"),
        initiator_id, setter_1_id, setter_2_id, setter_3_id, executor_id
    );

    let initiator_result = checked_call_vm!(initiator, "", &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, "", &script, "", initiator_result.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, "", &script, "", initiator_result.data.clone());
    let setter_3_res = checked_call_vm!(setter_3, "", &script, "", initiator_result.data);

    let executor_result_1 = checked_call_vm!(executor, "", &script, "", setter_1_res.data);
    let actual_trace_1 = trace_from_result(&executor_result_1);

    let test_value = "test";
    let expected_trace_1 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 0), SubTraceDesc::new(19, 2)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(15, 0), SubTraceDesc::new(17, 2)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(15, 0), SubTraceDesc::new(15, 2)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_1, expected_trace_1);

    let executor_result_2 = checked_call_vm!(executor, "", &script, executor_result_1.data.clone(), setter_2_res.data);
    let actual_trace_2 = trace_from_result(&executor_result_2);

    let expected_trace_2 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::request_sent_by(initiator_id),
        executed_state::request_sent_by(initiator_id),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 0), SubTraceDesc::new(19, 2)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(15, 0), SubTraceDesc::new(17, 2)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(15, 0), SubTraceDesc::new(15, 2)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 0), SubTraceDesc::new(23, 2)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(21, 0), SubTraceDesc::new(21, 2)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_2, expected_trace_2);

    let executor_result_3 = checked_call_vm!(executor, "", &script, executor_result_2.data.clone(), setter_3_res.data);
    let actual_trace_3 = trace_from_result(&executor_result_3);

    let expected_trace_3 = vec![
        executed_state::scalar_string(test_value),
        executed_state::par(11, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("3", 2),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(7, SubTraceDesc::new(15, 0), SubTraceDesc::new(19, 2)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(15, 0), SubTraceDesc::new(17, 2)),
            executed_state::subtrace_lore(12, SubTraceDesc::new(15, 0), SubTraceDesc::new(15, 2)),
            executed_state::subtrace_lore(8, SubTraceDesc::new(21, 0), SubTraceDesc::new(23, 2)),
            executed_state::subtrace_lore(13, SubTraceDesc::new(21, 0), SubTraceDesc::new(21, 2)),
            executed_state::subtrace_lore(10, SubTraceDesc::new(25, 0), SubTraceDesc::new(27, 2)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(25, 0), SubTraceDesc::new(25, 2)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];
    assert_eq!(actual_trace_3, expected_trace_3);
}
