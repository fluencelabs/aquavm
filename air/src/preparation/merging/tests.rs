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

use super::merge_execution_traces;
use crate::contexts::execution_trace::ValueType;
use crate::preparation::CallResult;
use crate::preparation::ExecutedState;
use crate::preparation::ExecutionTrace;
use crate::preparation::ParResult;
use crate::JValue;

use air_parser::ast;
use air_parser::ast::Instruction;

use std::rc::Rc;

#[test]
fn merge_call_states_1() {
    use CallResult::*;
    use ExecutedState::*;

    let mut prev_trace = ExecutionTrace::new();
    prev_trace.push_back(Par(ParResult(1, 1)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    prev_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    prev_trace.push_back(Par(ParResult(1, 1)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_3"))));
    prev_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));

    let mut current_trace = ExecutionTrace::new();
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_2"))));
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_4"))));

    let air = Instruction::Null(ast::Null);

    let actual_merged_trace =
        merge_execution_traces(prev_trace, current_trace, &air).expect("merging should be successful");

    let mut expected_merged_trace = ExecutionTrace::new();
    expected_merged_trace.push_back(Par(ParResult(1, 1)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Par(ParResult(1, 1)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_call_states_2() {
    use CallResult::*;
    use ExecutedState::*;

    let mut prev_trace = ExecutionTrace::new();
    prev_trace.push_back(Par(ParResult(1, 0)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    prev_trace.push_back(Par(ParResult(1, 1)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_2"))));
    prev_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));

    let mut current_trace = ExecutionTrace::new();
    current_trace.push_back(Par(ParResult(2, 2)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_2"))));

    let air = Instruction::Null(ast::Null);

    let actual_merged_trace =
        merge_execution_traces(prev_trace, current_trace, &air).expect("merging should be successful");

    let mut expected_merged_trace = ExecutionTrace::new();
    expected_merged_trace.push_back(Par(ParResult(2, 2)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    expected_merged_trace.push_back(Par(ParResult(1, 1)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));

    assert_eq!(actual_merged_trace, expected_merged_trace);
}

#[test]
fn merge_call_states_3() {
    use CallResult::*;
    use ExecutedState::*;

    let mut prev_trace = ExecutionTrace::new();
    prev_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    prev_trace.push_back(Par(ParResult(2, 0)));
    prev_trace.push_back(Par(ParResult(1, 0)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    prev_trace.push_back(Par(ParResult(1, 2)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    prev_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    prev_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));

    let mut current_trace = ExecutionTrace::new();
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Par(ParResult(3, 3)));
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    current_trace.push_back(Par(ParResult(1, 1)));
    current_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    current_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));

    let air = Instruction::Null(ast::Null);

    let actual_merged_trace =
        merge_execution_traces(prev_trace, current_trace, &air).expect("merging should be successful");

    let mut expected_merged_trace = ExecutionTrace::new();
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Par(ParResult(3, 3)));
    expected_merged_trace.push_back(Par(ParResult(1, 1)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Par(ParResult(1, 1)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));
    expected_merged_trace.push_back(Par(ParResult(1, 2)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(Executed(Rc::new(JValue::Null), ValueType::Scalar)));
    expected_merged_trace.push_back(Call(RequestSentBy(String::from("peer_1"))));

    assert_eq!(actual_merged_trace, expected_merged_trace);
}
