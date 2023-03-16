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
use super::ValueRef;
use crate::FoldLore;
use crate::FoldResult;
use crate::FoldSubTraceLore;
use crate::SubTraceDesc;

use air_interpreter_cid::value_to_json_cid;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::CidTracker;
use air_interpreter_data::GenerationIdx;
use avm_server::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

pub fn scalar(result: JValue) -> ExecutedState {
    let cid = value_to_json_cid(&result)
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, result));
    let value = ValueRef::Scalar(Rc::new(cid));
    ExecutedState::Call(CallResult::Executed(value))
}

pub fn scalar_tracked(result: impl Into<JValue>, tracker: &mut CidTracker) -> ExecutedState {
    let cid = tracker.record_value(Rc::new(result.into())).unwrap();
    let value = ValueRef::Scalar(cid);
    ExecutedState::Call(CallResult::Executed(value))
}

pub fn scalar_number(result: impl Into<serde_json::Number>) -> ExecutedState {
    let result = JValue::Number(result.into());

    scalar(result)
}

pub fn stream_call_result(result: JValue, generation: GenerationIdx) -> CallResult {
    let cid = value_to_json_cid(&result)
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, result));
    CallResult::executed_stream(Rc::new(cid), generation)
}

pub fn stream(result: JValue, generation: GenerationIdx) -> ExecutedState {
    ExecutedState::Call(stream_call_result(result, generation))
}

pub fn stream_tracked(
    value: impl Into<JValue>,
    generation: GenerationIdx,
    tracker: &mut CidTracker,
) -> ExecutedState {
    let cid = tracker.record_value(Rc::new(value.into())).unwrap();
    ExecutedState::Call(CallResult::executed_stream(cid, generation))
}

pub fn scalar_string(result: impl Into<String>) -> ExecutedState {
    let result = JValue::String(result.into());
    scalar(result)
}

pub fn scalar_string_array(result: Vec<impl Into<String>>) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();
    let value = JValue::Array(result);

    scalar(value)
}

pub fn stream_string(result: impl Into<String>, generation: GenerationIdx) -> ExecutedState {
    let result = JValue::String(result.into());

    stream(result, generation)
}

pub fn stream_number(result: impl Into<serde_json::Number>, generation: GenerationIdx) -> ExecutedState {
    let result = JValue::Number(result.into());

    stream(result, generation)
}

pub fn stream_string_array(result: Vec<impl Into<String>>, generation: GenerationIdx) -> ExecutedState {
    let result = result
        .into_iter()
        .map(|s| JValue::String(s.into()))
        .collect::<Vec<_>>();
    let value = JValue::Array(result);

    stream(value, generation)
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
        Rc::new(format!(r#""{error_message}""#)),
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

pub fn ap(generation: GenerationIdx) -> ExecutedState {
    let ap_result = ApResult::new(generation);
    ExecutedState::Ap(ap_result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueAggregateAlike {
    pub result: Rc<JValue>,
    pub tetraplet: Rc<SecurityTetraplet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanonResultAlike {
    pub tetraplet: Rc<SecurityTetraplet>,
    pub values: Vec<ValueAggregateAlike>,
}

/// This function takes a JSON DSL-like struct for compatibility and test writer
/// convenience.
pub fn canon(canonicalized_element: JValue) -> ExecutedState {
    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

    canon_tracked(
        canonicalized_element,
        &mut value_tracker,
        &mut tetraplet_tracker,
        &mut canon_tracker,
    )
}

pub fn canon_tracked(
    canonicalized_element: JValue,
    value_tracker: &mut CidTracker<JValue>,
    tetraplet_tracker: &mut CidTracker<SecurityTetraplet>,
    canon_tracker: &mut CidTracker<CanonCidAggregate>,
) -> ExecutedState {
    let canon_input = serde_json::from_value::<CanonResultAlike>(canonicalized_element)
        .expect("Malformed canon input");
    let tetraplet_cid = tetraplet_tracker
        .record_value(canon_input.tetraplet.clone())
        .unwrap_or_else(|e| {
            panic!(
                "{:?}: failed to compute CID of {:?}",
                e, canon_input.tetraplet
            )
        });
    let value_cids = canon_input
        .values
        .iter()
        .map(|value| {
            let value_cid = value_tracker.record_value(value.result.clone())?;
            let tetraplet_cid = tetraplet_tracker.record_value(value.tetraplet.clone())?;
            canon_tracker.record_value(CanonCidAggregate {
                value: value_cid,
                tetraplet: tetraplet_cid,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, canon_input.values));
    let canon_result = CanonResult::new(tetraplet_cid, value_cids);
    ExecutedState::Canon(canon_result)
}
