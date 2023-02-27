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

use air::ExecutionCidState;
use air_interpreter_cid::value_to_json_cid;
use air_interpreter_cid::CID;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::ServiceResultAggregate;
use air_interpreter_interface::CallServiceResult;
use avm_server::SecurityTetraplet;
use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

pub fn simple_value_aggregate_cid(
    result: impl Into<serde_json::Value>,
    cid_state: &mut ExecutionCidState,
) -> Rc<CID<ServiceResultAggregate>> {
    let value_cid = cid_state
        .value_tracker
        .record_value(Rc::new(result.into()))
        .unwrap();
    let tetraplet = SecurityTetraplet::default();
    let tetraplet_cid = cid_state
        .tetraplet_tracker
        .record_value(Rc::new(tetraplet))
        .unwrap();
    let service_result_agg = ServiceResultAggregate {
        value_cid,
        argument_hash: "".into(),
        tetraplet_cid,
    };
    cid_state
        .service_result_agg_tracker
        .record_value(Rc::new(service_result_agg))
        .unwrap()
}

pub fn value_aggregate_cid(
    result: impl Into<serde_json::Value>,
    tetraplet: SecurityTetraplet,
    args: Vec<serde_json::Value>,
    cid_state: &mut ExecutionCidState,
) -> Rc<CID<ServiceResultAggregate>> {
    let value_cid = cid_state
        .value_tracker
        .record_value(Rc::new(result.into()))
        .unwrap();
    let tetraplet_cid = cid_state
        .tetraplet_tracker
        .record_value(Rc::new(tetraplet))
        .unwrap();

    let arguments = serde_json::Value::Array(args);
    let argument_hash = value_to_json_cid(&arguments).unwrap().into_inner().into();

    let service_result_agg = ServiceResultAggregate {
        value_cid,
        argument_hash,
        tetraplet_cid,
    };

    cid_state
        .service_result_agg_tracker
        .record_value(Rc::new(service_result_agg))
        .unwrap()
}

pub fn scalar_tracked(
    result: impl Into<JValue>,
    tetraplet: SecurityTetraplet,
    args: Vec<serde_json::Value>,
    cid_state: &mut ExecutionCidState,
) -> ExecutedState {
    let service_result_agg_cid = value_aggregate_cid(result, tetraplet, args, cid_state);
    let value = ValueRef::Scalar(service_result_agg_cid);
    ExecutedState::Call(CallResult::Executed(value))
}

pub fn scalar_unused_tracked(
    result: impl Into<JValue>,
    tetraplet: SecurityTetraplet,
    args: Vec<serde_json::Value>,
    cid_state: &mut ExecutionCidState,
) -> ExecutedState {
    let service_result_agg_cid = value_aggregate_cid(result, tetraplet, args, cid_state);
    let value = ValueRef::Unused(service_result_agg_cid);
    ExecutedState::Call(CallResult::Executed(value))
}

pub fn scalar(result: JValue) -> ExecutedState {
    let mut cid_state = ExecutionCidState::new();
    scalar_tracked(result, SecurityTetraplet::default(), vec![], &mut cid_state)
}

pub fn scalar_number(result: impl Into<serde_json::Number>) -> ExecutedState {
    let result = JValue::Number(result.into());

    scalar(result)
}

pub fn stream_call_result(result: JValue, generation: u32) -> CallResult {
    let mut cid_state = ExecutionCidState::new();
    let service_result_agg_cid = simple_value_aggregate_cid(result, &mut cid_state);
    CallResult::Executed(ValueRef::Stream {
        cid: service_result_agg_cid,
        generation,
    })
}

pub fn stream(result: JValue, generation: u32) -> ExecutedState {
    ExecutedState::Call(stream_call_result(result, generation))
}

pub fn stream_tracked(
    value: impl Into<JValue>,
    generation: u32,
    tetraplet: SecurityTetraplet,
    args: Vec<serde_json::Value>,
    cid_state: &mut ExecutionCidState,
) -> ExecutedState {
    let service_result_agg_cid = value_aggregate_cid(value, tetraplet, args, cid_state);
    ExecutedState::Call(CallResult::Executed(ValueRef::Stream {
        cid: service_result_agg_cid,
        generation,
    }))
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

pub fn stream_string(result: impl Into<String>, generation: u32) -> ExecutedState {
    let result = JValue::String(result.into());

    stream(result, generation)
}

pub fn stream_number(result: impl Into<serde_json::Number>, generation: u32) -> ExecutedState {
    let result = JValue::Number(result.into());

    stream(result, generation)
}

pub fn stream_string_array(result: Vec<impl Into<String>>, generation: u32) -> ExecutedState {
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
    let mut cid_state = ExecutionCidState::new();
    let result = CallServiceResult {
        ret_code,
        result: error_message.to_owned(),
    };
    let result_value = serde_json::to_value(result).unwrap();
    let service_result_agg_cid = simple_value_aggregate_cid(result_value, &mut cid_state);
    ExecutedState::Call(CallResult::Failed(service_result_agg_cid))
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
    let mut cid_state = ExecutionCidState::new();

    canon_tracked(canonicalized_element, &mut cid_state)
}

pub fn canon_tracked(
    canonicalized_element: JValue,
    cid_state: &mut ExecutionCidState,
) -> ExecutedState {
    let canon_input = serde_json::from_value::<CanonResultAlike>(canonicalized_element)
        .expect("Malformed canon input");
    let tetraplet_cid = cid_state
        .tetraplet_tracker
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
            let value_cid = cid_state.value_tracker.record_value(value.result.clone())?;
            let tetraplet_cid = cid_state
                .tetraplet_tracker
                .record_value(value.tetraplet.clone())?;
            cid_state.canon_tracker.record_value(CanonCidAggregate {
                value: value_cid,
                tetraplet: tetraplet_cid,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, canon_input.values));
    let canon_result = CanonResult::new(tetraplet_cid, value_cids);
    ExecutedState::Canon(canon_result)
}
