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

use super::JValue;
use air::execution_trace::CallResult;
use air::execution_trace::ExecutedState;
use air::execution_trace::ParResult;
use air::execution_trace::ValueType;

use std::rc::Rc;

pub fn scalar_jvalue(result: JValue) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(Rc::new(result), ValueType::Scalar))
}

pub fn stream_jvalue(result: JValue, stream_name: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(
        Rc::new(result),
        ValueType::Stream(stream_name.into()),
    ))
}

pub fn scalar_string(result: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(
        Rc::new(JValue::String(result.into())),
        ValueType::Scalar,
    ))
}

pub fn scalar_string_array(result: Vec<impl Into<String>>) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();

    ExecutedState::Call(CallResult::Executed(
        Rc::new(JValue::Array(result)),
        ValueType::Scalar,
    ))
}

pub fn stream_string(result: impl Into<String>, stream_name: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(
        Rc::new(JValue::String(result.into())),
        ValueType::Stream(stream_name.into()),
    ))
}

pub fn stream_number(
    result: impl Into<serde_json::Number>,
    stream_name: impl Into<String>,
) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(
        Rc::new(JValue::Number(result.into())),
        ValueType::Stream(stream_name.into()),
    ))
}

pub fn stream_string_array(
    result: Vec<impl Into<String>>,
    stream_name: impl Into<String>,
) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();

    ExecutedState::Call(CallResult::Executed(
        Rc::new(JValue::Array(result)),
        ValueType::Stream(stream_name.into()),
    ))
}

pub fn request_sent_by(sender: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::RequestSentBy(sender.into()))
}

pub fn par(left: usize, right: usize) -> ExecutedState {
    ExecutedState::Par(ParResult(left, right))
}

pub fn service_failed(ret_code: i32, error_message: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::CallServiceFailed(
        ret_code,
        Rc::new(error_message.into()),
    ))
}
