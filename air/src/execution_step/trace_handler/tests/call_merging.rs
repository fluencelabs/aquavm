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

use super::*;

use DataMergingError::IncompatibleCallResults;

use serde_json::json;

#[test]
fn merge_sent_and_executed() {
    let executed_state = scalar_jvalue(json!([]));
    let prev_trace = vec![request_sent_by("peer_1")];
    let current_trace = vec![executed_state.clone()];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into())
        .expect("merging should be successful");
    let expected = vec![executed_state];
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into()).expect("merging should be successful");
    assert_eq!(actual, expected);
}

#[test]
fn merge_sent_and_failed() {
    let failed_state = service_failed(1, "");
    let prev_trace = vec![request_sent_by("peer_1")];
    let current_trace = vec![failed_state.clone()];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into())
        .expect("merging should be successful");
    let expected = vec![failed_state];
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into()).expect("merging should be successful");
    assert_eq!(actual, expected);
}

#[test]
fn merge_failed_and_executed() {
    let prev_trace = vec![scalar_jvalue(json!([]))];
    let current_trace = vec![service_failed(1, "")];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::executed(json!([])),
        CallResult::failed(1, String::new()),
    ));
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::failed(1, String::new()),
        CallResult::executed(json!([])),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn merge_different_executed() {
    let prev_result = json!(1);
    let prev_trace = vec![scalar_jvalue(prev_result.clone())];
    let current_result = json!(2);
    let current_trace = vec![scalar_jvalue(current_result.clone())];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::executed(prev_result.clone()),
        CallResult::executed(current_result.clone()),
    ));
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::executed(current_result),
        CallResult::executed(prev_result),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn merge_different_request_sent() {
    let prev_sender = "sender_1";
    let prev_trace = vec![request_sent_by(prev_sender)];
    let current_sender = "sender_2";
    let current_trace = vec![request_sent_by(current_sender)];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::sent(prev_sender),
        CallResult::sent(current_sender),
    ));
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::sent(current_sender),
        CallResult::sent(prev_sender),
    ));
    assert_eq!(actual, expected);
}

#[test]
fn merge_different_errors() {
    let prev_ret_code = 1;
    let prev_trace = vec![service_failed(prev_ret_code, "")];
    let current_ret_code = 2;
    let current_trace = vec![service_failed(current_ret_code, "")];

    let actual = merge_execution_traces(prev_trace.clone().into(), current_trace.clone().into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::failed(prev_ret_code, ""),
        CallResult::failed(current_ret_code, ""),
    ));
    assert_eq!(actual, expected);

    let actual = merge_execution_traces(current_trace.into(), prev_trace.into());
    let expected: MergeResult<ExecutionTrace> = Err(IncompatibleCallResults(
        CallResult::failed(current_ret_code, ""),
        CallResult::failed(prev_ret_code, ""),
    ));
    assert_eq!(actual, expected);
}
