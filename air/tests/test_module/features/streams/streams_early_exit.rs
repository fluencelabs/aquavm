/*
 * Copyright 2021 Fluence Labs Limited
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

use air::UncatchableError;
use air_interpreter_cid::value_to_json_cid;
use air_interpreter_data::CidTracker;
use air_interpreter_data::ValueRef;
use air_test_utils::prelude::*;
use air_trace_handler::merger::CallResultError;
use air_trace_handler::merger::MergeError;
use air_trace_handler::TraceHandlerError;

#[test]
fn par_early_exit() {
    let init_peer_id = "init_peer_id";
    let setter_1_id = "setter_1";
    let setter_2_id = "setter_2";
    let setter_3_id = "setter_3";

    let mut init = create_avm(unit_call_service(), init_peer_id);
    let mut setter_1 = create_avm(set_variable_call_service(json!("1")), setter_1_id);
    let mut setter_2 = create_avm(set_variable_call_service(json!("2")), setter_2_id);
    let mut setter_3 = create_avm(fallible_call_service("error"), setter_3_id);

    let script = format!(
        include_str!("scripts/par_early_exit.air"),
        init_peer_id, setter_1_id, setter_2_id, setter_3_id
    );

    let init_result_1 = checked_call_vm!(init, <_>::default(), &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, <_>::default(), &script, "", init_result_1.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, <_>::default(), &script, "", init_result_1.data.clone());
    let setter_3_res_1 = checked_call_vm!(setter_3, <_>::default(), &script, "", init_result_1.data.clone());
    let actual_trace_1 = trace_from_result(&setter_3_res_1);

    let expected_trace = vec![
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::request_sent_by(setter_3_id),
    ];
    assert_eq!(actual_trace_1, expected_trace);

    let setter_3_res_2 = checked_call_vm!(
        setter_3,
        <_>::default(),
        &script,
        setter_3_res_1.data,
        setter_1_res.data
    );
    let setter_3_res_3 = checked_call_vm!(
        setter_3,
        <_>::default(),
        &script,
        setter_3_res_2.data,
        setter_2_res.data
    );
    let init_result_2 = checked_call_vm!(
        init,
        <_>::default(),
        &script,
        init_result_1.data,
        setter_3_res_3.data.clone()
    );
    let actual_trace_2 = trace_from_result(&setter_3_res_3);
    let actual_trace_3 = trace_from_result(&init_result_2);

    let expected_trace = vec![
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("2", 2),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::request_sent_by("setter_3"),
    ];
    assert_eq!(actual_trace_2, expected_trace);

    let expected_trace = vec![
        executed_state::scalar_string("result from unit_call_service"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("2", 2),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::stream_string("success result from fallible_call_service", 0),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::scalar_string("result from unit_call_service"),
    ];
    assert_eq!(actual_trace_3, expected_trace);

    let mut setter_3_tracker = CidTracker::new();
    let setter_3_malicious_trace = vec![
        executed_state::scalar_tracked("result from unit_call_service", &mut setter_3_tracker),
        executed_state::par(10, 0),
        executed_state::par(9, 0),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::stream_tracked("non_exist_value", 0, &mut setter_3_tracker),
        executed_state::stream_tracked("success result from fallible_call_service", 0, &mut setter_3_tracker),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::request_sent_by(setter_3_id),
    ];
    let setter_3_malicious_data = raw_data_from_trace(setter_3_malicious_trace, setter_3_tracker);
    let init_result_3 = call_vm!(
        init,
        <_>::default(),
        &script,
        init_result_2.data.clone(),
        setter_3_malicious_data
    );

    let prev_value = ValueRef::Stream {
        cid: value_to_json_cid(&json!("1")).unwrap().into(),
        generation: 1,
    };
    let current_value = ValueRef::Stream {
        cid: value_to_json_cid(&json!("non_exist_value")).unwrap().into(),
        generation: 0,
    };
    let expected_error = UncatchableError::TraceError {
        trace_error: TraceHandlerError::MergeError(MergeError::IncorrectCallResult(CallResultError::ValuesNotEqual {
            prev_value,
            current_value,
        })),
        instruction: r#"call "setter_1" ("" "") [] $stream"#.to_string(),
    };
    assert!(check_error(&init_result_3, expected_error));

    let actual_trace = trace_from_result(&init_result_3);
    let expected_trace = trace_from_result(&init_result_2);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_early_exit() {
    let fold_executor_id = "fold_executor_id";
    let error_trigger_id = "error_trigger_id";
    let last_error_receiver_id = "last_error_receiver_id";
    let last_peer_checker_id = "last_peer_checker_id";

    let mut fold_executor = create_avm(unit_call_service(), fold_executor_id);
    let mut error_trigger = create_avm(fallible_call_service("error"), error_trigger_id);
    let mut last_peer_checker = create_avm(echo_call_service(), last_peer_checker_id);

    let script = format!(
        include_str!("scripts/fold_early_exit.air"),
        fold_executor_id = fold_executor_id,
        error_trigger_id = error_trigger_id,
        last_error_receiver_id = last_error_receiver_id,
        last_peer_checker_id = last_peer_checker_id
    );

    let fold_executor_result = checked_call_vm!(fold_executor, <_>::default(), &script, "", "");
    let error_trigger_result = checked_call_vm!(error_trigger, <_>::default(), &script, "", fold_executor_result.data);
    let fold_executor_result = checked_call_vm!(fold_executor, <_>::default(), &script, "", error_trigger_result.data);
    let error_trigger_result = checked_call_vm!(error_trigger, <_>::default(), &script, "", fold_executor_result.data);
    let last_peer_checker_result = checked_call_vm!(
        last_peer_checker,
        <_>::default(),
        &script,
        "",
        error_trigger_result.data
    );
    let actual_trace = trace_from_result(&last_peer_checker_result);

    let expected_state = executed_state::scalar(json!({
                "error_code": 10000i64,
                "instruction" : r#"call "error_trigger_id" ("error" "") [] "#,
                "message": r#"Local service error, ret_code is 1, error message is '"failed result from fallible_call_service"'"#,
                "peer_id": "error_trigger_id"}));

    let bubbled_error_from_stream_1 = actual_trace.len() - 3;
    assert_eq!(&actual_trace[bubbled_error_from_stream_1.into()], &expected_state);

    let bubbled_error_from_stream_2 = actual_trace.len() - 2;
    assert_eq!(&actual_trace[bubbled_error_from_stream_2.into()], &expected_state);
}

#[test]
fn fold_par_early_exit() {
    let variables_setter_id = "set_variable_id";
    let stream_setter_id = "stream_setter_id";
    let fold_executor_id = "fold_executor_id";
    let error_trigger_id = "error_trigger_id";
    let last_error_receiver_id = "last_error_receiver_id";
    let last_peer_checker_id = "last_peer_checker_id";

    let variables = maplit::hashmap!(
        "stream_1".to_string() => json!(["a1", "a2"]),
        "stream_2".to_string() => json!(["b1", "b2"]),
        "stream_3".to_string() => json!(["c1", "c2"]),
        "stream_4".to_string() => json!(["d1", "d2"]),
    );

    let mut variables_setter = create_avm(
        set_variables_call_service(variables, VariableOptionSource::Argument(0)),
        variables_setter_id,
    );
    let mut stream_setter = create_avm(echo_call_service(), stream_setter_id);
    let mut fold_executor = create_avm(unit_call_service(), fold_executor_id);
    let mut error_trigger = create_avm(fallible_call_service("error"), error_trigger_id);
    let mut last_error_receiver = create_avm(unit_call_service(), last_error_receiver_id);
    let mut last_peer_checker = create_avm(unit_call_service(), last_peer_checker_id);

    let script = format!(
        include_str!("scripts/fold_par_early_exit.air"),
        variables_setter_id,
        stream_setter_id,
        fold_executor_id,
        error_trigger_id,
        last_error_receiver_id,
        last_peer_checker_id
    );

    let variables_setter_result = checked_call_vm!(variables_setter, <_>::default(), &script, "", "");
    let stream_setter_result =
        checked_call_vm!(stream_setter, <_>::default(), &script, "", variables_setter_result.data);
    let fold_executor_result = checked_call_vm!(fold_executor, <_>::default(), &script, "", stream_setter_result.data);
    let error_trigger_result = checked_call_vm!(error_trigger, <_>::default(), &script, "", fold_executor_result.data);
    let last_error_receiver_result = checked_call_vm!(
        last_error_receiver,
        <_>::default(),
        &script,
        "",
        error_trigger_result.data
    );
    let last_peer_checker_result = checked_call_vm!(
        last_peer_checker,
        <_>::default(),
        &script,
        "",
        last_error_receiver_result.data
    );
    let actual_trace = trace_from_result(&last_peer_checker_result);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace = vec![
        executed_state::scalar_string_array(vec!["a1", "a2"]),
        executed_state::scalar_string_array(vec!["b1", "b2"]),
        executed_state::scalar_string_array(vec!["c1", "c2"]),
        executed_state::scalar_string_array(vec!["d1", "d2"]),
        executed_state::stream_string("a1", 0),
        executed_state::stream_string("a2", 1),
        executed_state::stream_string("b1", 0),
        executed_state::stream_string("b2", 1),
        executed_state::stream_string("c1", 0),
        executed_state::stream_string("c2", 1),
        executed_state::stream_string("d1", 0),
        executed_state::stream_string("d2", 1),
        executed_state::par(69, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(4, subtrace_desc(14 as PosType, 34), subtrace_desc(48 as PosType, 0)),
            executed_state::subtrace_lore(5, subtrace_desc(48 as PosType, 34), subtrace_desc(82 as PosType, 0)),
        ]),
        executed_state::par(33, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(6, subtrace_desc(16 as PosType, 16), subtrace_desc(32 as PosType, 0)),
            executed_state::subtrace_lore(7, subtrace_desc(32 as PosType, 16), subtrace_desc(48 as PosType, 0)),
        ]),
        executed_state::par(15, 0),
        executed_state::par(13, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(8, subtrace_desc(19 as PosType, 6), subtrace_desc(25 as PosType, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(25 as PosType, 6), subtrace_desc(31 as PosType, 0)),
        ]),
        executed_state::par(5, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, subtrace_desc(21 as PosType, 2), subtrace_desc(23 as PosType, 0)),
            executed_state::subtrace_lore(11, subtrace_desc(23 as PosType, 2), subtrace_desc(25 as PosType, 0)),
        ]),
        executed_state::par(1, 0),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::par(1, 0),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::par(5, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, subtrace_desc(27 as PosType, 2), subtrace_desc(29 as PosType, 0)),
            executed_state::subtrace_lore(11, subtrace_desc(29 as PosType, 2), subtrace_desc(31 as PosType, 0)),
        ]),
        executed_state::par(1, 0),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::par(1, 0),
        executed_state::scalar_string(unit_call_service_result),
        executed_state::service_failed(1, "failed result from fallible_call_service"),
        executed_state::par(15, 0),
        executed_state::par(13, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(8, subtrace_desc(35 as PosType, 6), subtrace_desc(41 as PosType, 0)),
            executed_state::subtrace_lore(9, subtrace_desc(41 as PosType, 6), subtrace_desc(47 as PosType, 0)),
        ]),
    ];
    let trace_len = expected_trace.len();

    assert_eq!((*actual_trace)[0..trace_len], expected_trace);
}
