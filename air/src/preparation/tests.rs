use super::merge_execution_traces;
use crate::JValue;

use air_test_utils::executed_state::*;
use air_test_utils::ExecutionTrace;

use std::rc::Rc;

#[test]
fn merge_call_states_1() {
    let mut prev_trace = vec![
        par(1, 1),
        request_sent_by("peer_1"),
        scalar_jvalue(JValue::Null),
        par(1, 1),
        request_sent_by("peer_3"),
        scalar_jvalue(JValue::Null),
    ];

    let current_trace = vec![
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_2"),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_4"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        par(1, 1),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_call_states_2() {
    let prev_trace = vec![
        par(1, 0),
        request_sent_by("peer_1"),
        par(1, 1),
        request_sent_by("peer_2"),
        scalar_jvalue(JValue::Null),
    ];

    let current_trace = vec![
        par(2, 2),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_2"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        par(2, 2),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_call_states_3() {
    let prev_trace = vec![
        scalar_jvalue(JValue::Null),
        par(2, 0),
        par(1, 0),
        request_sent_by("peer_1"),
        par(1, 2),
        request_sent_by("peer_1"),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
    ];

    let current_trace = vec![
        scalar_jvalue(JValue::Null),
        par(3, 3),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        scalar_jvalue(JValue::Null),
        par(3, 3),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        par(1, 1),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
        par(1, 2),
        scalar_jvalue(JValue::Null),
        scalar_jvalue(JValue::Null),
        request_sent_by("peer_1"),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}
