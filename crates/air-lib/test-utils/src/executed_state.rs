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

use super::ApResult;
use super::CallResult;
use super::CanonResult;
use super::ExecutedState;
use super::JValue;
use super::ParResult;
use super::Sender;
use super::TracePos;
use super::Value;
use crate::FoldLore;
use crate::FoldResult;
use crate::FoldSubTraceLore;
use crate::SubTraceDesc;

use std::rc::Rc;

pub fn scalar(result: JValue) -> ExecutedState {
    let value = Value::Scalar(Rc::new(result));
    ExecutedState::Call(CallResult::Executed(value))
}

pub fn scalar_number(result: impl Into<serde_json::Number>) -> ExecutedState {
    let result = JValue::Number(result.into());
    let value = Rc::new(result);

    ExecutedState::Call(CallResult::executed_scalar(value))
}

pub fn stream(result: JValue, generation: u32) -> ExecutedState {
    let call_result = CallResult::executed_stream(Rc::new(result), generation);
    ExecutedState::Call(call_result)
}

pub fn scalar_string(result: impl Into<String>) -> ExecutedState {
    let result = JValue::String(result.into());
    let value = Rc::new(result);
    ExecutedState::Call(CallResult::executed_scalar(value))
}

pub fn scalar_string_array(result: Vec<impl Into<String>>) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();
    let value = Rc::new(JValue::Array(result));

    ExecutedState::Call(CallResult::executed_scalar(value))
}

pub fn stream_string(result: impl Into<String>, generation: u32) -> ExecutedState {
    let result = JValue::String(result.into());
    let value = Rc::new(result);

    ExecutedState::Call(CallResult::executed_stream(value, generation))
}

pub fn stream_number(result: impl Into<serde_json::Number>, generation: u32) -> ExecutedState {
    let result = JValue::Number(result.into());
    let value = Rc::new(result);

    ExecutedState::Call(CallResult::executed_stream(value, generation))
}

pub fn stream_string_array(result: Vec<impl Into<String>>, generation: u32) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();
    let value = Rc::new(JValue::Array(result));

    ExecutedState::Call(CallResult::executed_stream(value, generation))
}

pub fn request_sent_by(sender: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::RequestSentBy(Sender::PeerId(Rc::new(
        sender.into(),
    ))))
}

pub fn par(left: usize, right: usize) -> ExecutedState {
    let par_result = ParResult {
        left_size: left as _,
        right_size: right as _,
    };

    ExecutedState::Par(par_result)
}

pub fn service_failed(ret_code: i32, error_message: &str) -> ExecutedState {
    ExecutedState::Call(CallResult::CallServiceFailed(
        ret_code,
        Rc::new(format!(r#""{}""#, error_message)),
    ))
}

pub fn fold(lore: FoldLore) -> ExecutedState {
    let result = FoldResult { lore };
    ExecutedState::Fold(result)
}

pub fn subtrace_lore(
    value_pos: usize,
    before: SubTraceDesc,
    after: SubTraceDesc,
) -> FoldSubTraceLore {
    FoldSubTraceLore {
        value_pos: value_pos.into(),
        subtraces_desc: vec![before, after],
    }
}

pub fn subtrace_desc(begin_pos: impl Into<TracePos>, subtrace_len: u32) -> SubTraceDesc {
    SubTraceDesc {
        begin_pos: begin_pos.into(),
        subtrace_len,
    }
}

pub fn ap(generation: u32) -> ExecutedState {
    let ap_result = ApResult::new(generation);
    ExecutedState::Ap(ap_result)
}

pub fn canon(canonicalized_element: JValue) -> ExecutedState {
    let canon_result = CanonResult::new(canonicalized_element);
    ExecutedState::Canon(canon_result)
}
