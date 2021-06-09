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

use super::*;

use serde_json::json;

#[test]
fn merge_par_states_1() {
    let prev_trace = vec![
        par(1, 1),
        request_sent_by("peer_1"),
        scalar_jvalue(json!([])),
        par(1, 1),
        request_sent_by("peer_3"),
        scalar_jvalue(json!([])),
    ];

    let current_trace = vec![
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_2"),
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_4"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        par(1, 1),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        par(1, 1),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_par_states_2() {
    let prev_trace = vec![
        par(1, 0),
        request_sent_by("peer_1"),
        par(1, 1),
        request_sent_by("peer_2"),
        scalar_jvalue(json!([])),
    ];

    let current_trace = vec![
        par(2, 2),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_2"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        par(2, 2),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        par(1, 1),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_par_states_3() {
    let prev_trace = vec![
        scalar_jvalue(json!([])),
        par(2, 0),
        par(1, 0),
        request_sent_by("peer_1"),
        par(1, 2),
        request_sent_by("peer_1"),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
    ];

    let current_trace = vec![
        scalar_jvalue(json!([])),
        par(3, 3),
        par(1, 1),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
    ];

    let actual_merged_trace =
        merge_execution_traces(prev_trace.into(), current_trace.into()).expect("merging should be successful");

    let expected_merged_trace = vec![
        scalar_jvalue(json!([])),
        par(3, 3),
        par(1, 1),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        par(1, 1),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
        par(1, 2),
        scalar_jvalue(json!([])),
        scalar_jvalue(json!([])),
        request_sent_by("peer_1"),
    ];

    assert_eq!(actual_merged_trace, expected_merged_trace);
}
