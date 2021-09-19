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

use air_test_utils::prelude::*;

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
        include_str!("scripts/par_early_exit.clj"),
        init_peer_id, setter_1_id, setter_2_id, setter_3_id
    );

    let init_result_1 = checked_call_vm!(init, "", &script, "", "");
    let setter_1_res = checked_call_vm!(setter_1, "", &script, "", init_result_1.data.clone());
    let setter_2_res = checked_call_vm!(setter_2, "", &script, "", init_result_1.data.clone());
    let setter_3_res_1 = checked_call_vm!(setter_3, "", &script, "", init_result_1.data.clone());
    let actual_trace_1 = trace_from_result(&setter_3_res_1);

    let expected_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::stream_string("test", 0),
        executed_state::service_failed(1, r#""error""#),
        executed_state::stream_string("test", 0),
        executed_state::service_failed(1, r#""error""#),
        executed_state::service_failed(1, r#""error""#),
        executed_state::request_sent_by(setter_3_id),
    ];
    assert_eq!(actual_trace_1, expected_trace);

    let setter_3_res_2 = checked_call_vm!(
        setter_3,
        "",
        &script,
        setter_3_res_1.data.clone(),
        setter_1_res.data.clone()
    );
    let setter_3_res_3 = checked_call_vm!(
        setter_3,
        "",
        &script,
        setter_3_res_2.data.clone(),
        setter_2_res.data.clone()
    );
    let init_result_2 = checked_call_vm!(
        init,
        "",
        &script,
        init_result_1.data.clone(),
        setter_3_res_3.data.clone()
    );
    let actual_trace_2 = trace_from_result(&setter_3_res_3);
    let actual_trace_3 = trace_from_result(&init_result_2);

    let expected_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("2", 2),
        executed_state::stream_string("1", 1),
        executed_state::stream_string("res", 0),
        executed_state::service_failed(1, "error"),
        executed_state::stream_string("res", 0),
        executed_state::service_failed(1, "error"),
        executed_state::service_failed(1, "error"),
        executed_state::request_sent_by("setter_3"),
    ];
    assert_eq!(actual_trace_2, expected_trace);

    let expected_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::par(12, 1),
        executed_state::par(9, 1),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("2", 0),
        executed_state::stream_string("1", 0),
        executed_state::stream_string("res", 0),
        executed_state::service_failed(1, "error"),
        executed_state::stream_string("res", 0),
        executed_state::service_failed(1, "error"),
        executed_state::service_failed(1, "error"),
        executed_state::scalar_string("test"),
    ];
    assert_eq!(actual_trace_3, expected_trace);

    let setter_3_malicious_trace = vec![
        executed_state::scalar_string("test"),
        executed_state::par(10, 0),
        executed_state::par(9, 0),
        executed_state::par(7, 1),
        executed_state::par(5, 1),
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        executed_state::request_sent_by(init_peer_id),
        executed_state::request_sent_by(init_peer_id),
        executed_state::stream_string("non_exist_value", 0),
        executed_state::stream_string("test", 0),
        executed_state::service_failed(1, r#""error""#),
        executed_state::request_sent_by(setter_3_id),
    ];
    let setter_3_malicious_data = raw_data_from_trace(setter_3_malicious_trace);
    let init_result_3 = call_vm!(init, "", &script, init_result_2.data.clone(), setter_3_malicious_data);
    assert_eq!(init_result_3.ret_code, 1018);

    let actual_trace = trace_from_result(&init_result_3);
    let expected_trace = trace_from_result(&init_result_2);
    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn fold_early_exit__() {
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

    let mut variables_setter = create_avm(set_variables_call_service(variables), variables_setter_id);
    let mut stream_setter = create_avm(echo_call_service(), stream_setter_id);
    let mut fold_executor = create_avm(unit_call_service(), fold_executor_id);
    let mut error_trigger = create_avm(fallible_call_service("error"), error_trigger_id);
    let mut last_error_receiver = create_avm(unit_call_service(), last_error_receiver_id);
    let mut last_peer_checker = create_avm(unit_call_service(), last_peer_checker_id);

    let script = format!(
        include_str!("scripts/fold_early_exit.clj"),
        variables_setter_id,
        stream_setter_id,
        fold_executor_id,
        error_trigger_id,
        last_error_receiver_id,
        last_peer_checker_id
    );

    let variables_setter_result = checked_call_vm!(variables_setter, "", &script, "", "");
    let stream_setter_result = checked_call_vm!(stream_setter, "", &script, "", variables_setter_result.data);
    let fold_executor_result = checked_call_vm!(fold_executor, "", &script, "", stream_setter_result.data);
    let error_trigger_result = checked_call_vm!(error_trigger, "", &script, "", fold_executor_result.data);
    let last_error_receiver_result = checked_call_vm!(last_error_receiver, "", &script, "", error_trigger_result.data);
    let last_peer_checker_result =
        checked_call_vm!(last_peer_checker, "", &script, "", last_error_receiver_result.data);
    let actual_trace = trace_from_result(&last_peer_checker_result);

    let test_value = "test";
    let expected_trace = vec![
        executed_state::scalar_string_array(vec!["a1", "a2"]),
        executed_state::scalar_string_array(vec!["b1", "b2"]),
        executed_state::scalar_string_array(vec!["c1", "c2"]),
        executed_state::scalar_string_array(vec!["d1", "d2"]),
        executed_state::stream_string("a1", 0),
        executed_state::stream_string("a2", 0),
        executed_state::stream_string("b1", 0),
        executed_state::stream_string("b2", 0),
        executed_state::stream_string("c1", 0),
        executed_state::stream_string("c2", 0),
        executed_state::stream_string("d1", 0),
        executed_state::stream_string("d2", 0),
        executed_state::par(11, 1),
        executed_state::fold(vec![executed_state::subtrace_lore(
            4,
            SubTraceDesc::new(14, 9),
            SubTraceDesc::new(23, 0),
        )]),
        executed_state::fold(vec![executed_state::subtrace_lore(
            6,
            SubTraceDesc::new(15, 8),
            SubTraceDesc::new(23, 0),
        )]),
        executed_state::fold(vec![
            executed_state::subtrace_lore(8, SubTraceDesc::new(16, 3), SubTraceDesc::new(22, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(19, 3), SubTraceDesc::new(22, 0)),
        ]),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, SubTraceDesc::new(17, 1), SubTraceDesc::new(19, 0)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(18, 1), SubTraceDesc::new(19, 0)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, SubTraceDesc::new(20, 1), SubTraceDesc::new(22, 0)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(21, 1), SubTraceDesc::new(22, 0)),
        ]),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
        executed_state::service_failed(1, r#""error""#),
        executed_state::scalar_string(test_value),
        executed_state::scalar_string(test_value),
    ];

    assert_eq!(actual_trace, expected_trace);
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

    let mut variables_setter = create_avm(set_variables_call_service(variables), variables_setter_id);
    let mut stream_setter = create_avm(echo_call_service(), stream_setter_id);
    let mut fold_executor = create_avm(unit_call_service(), fold_executor_id);
    let mut error_trigger = create_avm(fallible_call_service("error"), error_trigger_id);
    let mut last_error_receiver = create_avm(unit_call_service(), last_error_receiver_id);
    let mut last_peer_checker = create_avm(unit_call_service(), last_peer_checker_id);

    let script = format!(
        include_str!("scripts/fold_par_early_exit.clj"),
        variables_setter_id,
        stream_setter_id,
        fold_executor_id,
        error_trigger_id,
        last_error_receiver_id,
        last_peer_checker_id
    );

    let variables_setter_result = checked_call_vm!(variables_setter, "", &script, "", "");
    let stream_setter_result = checked_call_vm!(stream_setter, "", &script, "", variables_setter_result.data);
    let fold_executor_result = checked_call_vm!(fold_executor, "", &script, "", stream_setter_result.data);
    let error_trigger_result = checked_call_vm!(error_trigger, "", &script, "", fold_executor_result.data);
    let last_error_receiver_result = checked_call_vm!(last_error_receiver, "", &script, "", error_trigger_result.data);
    let last_peer_checker_result =
        checked_call_vm!(last_peer_checker, "", &script, "", last_error_receiver_result.data);
    let actual_trace = trace_from_result(&last_peer_checker_result);

    let test_value = "test";
    let expected_trace = vec![
        executed_state::scalar_string_array(vec!["a1", "a2"]),
        executed_state::scalar_string_array(vec!["b1", "b2"]),
        executed_state::scalar_string_array(vec!["c1", "c2"]),
        executed_state::scalar_string_array(vec!["d1", "d2"]),
        executed_state::stream_string("a1", 0),
        executed_state::stream_string("a2", 0),
        executed_state::stream_string("b1", 0),
        executed_state::stream_string("b2", 0),
        executed_state::stream_string("c1", 0),
        executed_state::stream_string("c2", 0),
        executed_state::stream_string("d1", 0),
        executed_state::stream_string("d2", 0),
        executed_state::par(69, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(4, SubTraceDesc::new(14, 34), SubTraceDesc::new(82, 0)),
            executed_state::subtrace_lore(5, SubTraceDesc::new(48, 34), SubTraceDesc::new(82, 0)),
        ]),
        executed_state::par(33, 34),
        executed_state::fold(vec![
            executed_state::subtrace_lore(6, SubTraceDesc::new(16, 16), SubTraceDesc::new(48, 0)),
            executed_state::subtrace_lore(7, SubTraceDesc::new(32, 16), SubTraceDesc::new(48, 0)),
        ]),
        executed_state::par(15, 16),
        executed_state::par(13, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(8, SubTraceDesc::new(19, 6), SubTraceDesc::new(31, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(25, 6), SubTraceDesc::new(31, 0)),
        ]),
        executed_state::par(5, 6),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, SubTraceDesc::new(21, 2), SubTraceDesc::new(25, 0)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(23, 2), SubTraceDesc::new(25, 0)),
        ]),
        executed_state::par(1, 2),
        executed_state::scalar_string(test_value),
        executed_state::par(1, 0),
        executed_state::scalar_string(test_value),
        executed_state::par(5, 0),
        executed_state::fold(vec![
            executed_state::subtrace_lore(10, SubTraceDesc::new(27, 2), SubTraceDesc::new(31, 0)),
            executed_state::subtrace_lore(11, SubTraceDesc::new(29, 2), SubTraceDesc::new(31, 0)),
        ]),
        executed_state::par(1, 2),
        executed_state::scalar_string(test_value),
        executed_state::par(1, 0),
        executed_state::scalar_string(test_value),
        executed_state::service_failed(1, r#""error""#),
        executed_state::par(15, 0),
        executed_state::par(13, 1),
        executed_state::fold(vec![
            executed_state::subtrace_lore(8, SubTraceDesc::new(35, 6), SubTraceDesc::new(47, 0)),
            executed_state::subtrace_lore(9, SubTraceDesc::new(41, 6), SubTraceDesc::new(47, 0)),
        ]),
    ];
    let trace_len = expected_trace.len();

    assert_eq!(&actual_trace[0..trace_len], expected_trace);
}
