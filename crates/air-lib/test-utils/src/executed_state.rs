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
use super::CanonResultAggregate;
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
use air_interpreter_data::Provenance;
use air_interpreter_data::ServiceResultAggregate;
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
    // TODO convert data and remove Provenance
    pub provenance: Option<Provenance>,
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
            cid_state
                .canon_element_tracker
                .record_value(CanonCidAggregate {
                    value: value_cid,
                    tetraplet: tetraplet_cid,
                    provenance: value
                        .provenance
                        .clone()
                        .unwrap_or_else(|| Provenance::literal(None)),
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, canon_input.values));

    let canon_result = CanonResultAggregate::new(tetraplet_cid, value_cids);
    let canon_result_cid = cid_state
        .canon_result_tracker
        .record_value(canon_result.clone())
        .unwrap_or_else(|e| panic!("{:?}: failed to compute CID of {:?}", e, canon_result));
    ExecutedState::Canon(CanonResult::new(canon_result_cid))
}

#[macro_export]
macro_rules! _trace_value_body {
    ($value:expr) => {
        $crate::executed_state::ExecutedCallBuilder::new($value)
    };

    ($value:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        $crate::executed_state::ExecutedCallBuilder::new($value) .$func1($v1) $(. $func($v))*
    };
}

#[macro_export]
macro_rules! scalar {
    ($value:expr) => {
        _trace_value_body!($value).scalar()
    };

    ($value:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        _trace_value_body!($value, $func1 = $v1 $(, $func = $v)*).scalar()
    };
}

#[macro_export]
macro_rules! scalar_tracked {
    ($value:expr, $state:expr) => {
        _trace_value_body!($value).scalar_tracked(&mut $state)
    };

    ($value:expr, $state:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        _trace_value_body!($value, $func1 = $v1 $(, $func = $v)*).scalar_tracked(&mut $state)
    };
}

#[macro_export]
macro_rules! failed {
    ($ret_code:expr, $error_message:expr) => {{
        let failed_value = $crate::executed_state::_failure_to_value($ret_code, $error_message);
        _trace_value_body!(failed_value).failed()
    }};
    ($ret_code:expr, $error_message:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {{
        let failed_value = $crate::executed_state::_failure_to_value($ret_code, $error_message);
        _trace_value_body!(failed_value, $func1 = $v1 $(, $func = $v)*).failed()
    }}
}

#[macro_export]
macro_rules! stream {
    ($value:expr, $generation:expr) => {
        _trace_value_body!($value).stream($generation)
    };

    ($value:expr, $generation:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        _trace_value_body!($value, $func1 = $v1 $(, $func = $v)*).stream($generation)
    };
}

#[macro_export]
macro_rules! stream_tracked {
    ($value:expr, $generation:expr, $state:expr) => {
        _trace_value_body!($value).stream_tracked($generation, &mut $state)
    };

    ($value:expr, $generation:expr, $state:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        _trace_value_body!($value, $func1 = $v1 $(, $func = $v)*).stream_tracked($generation, &mut $state)
    };
}

/// Please note that `unused_tracked` does not exist as unused is never tracked.
#[macro_export]
macro_rules! unused {
    ($value:expr) => {
        _trace_value_body!($value).unused()
    };

    ($value:expr, $func1:ident = $v1:expr $(, $func:ident = $v:expr)*) => {
        _trace_value_body!($value, $func1 = $v1 $(, $func = $v)*).unused()
    };
}

pub fn _failure_to_value(ret_code: i32, error_message: &str) -> JValue {
    let message_serialized = serde_json::to_string(error_message).unwrap();
    crate::CallServiceFailed::new(ret_code, message_serialized.into()).to_value()
}

pub struct ExecutedCallBuilder {
    result: JValue,
    tetraplet: SecurityTetraplet,
    args: Vec<JValue>,
}

impl ExecutedCallBuilder {
    pub fn new(result: impl Into<JValue>) -> Self {
        Self {
            result: result.into(),
            tetraplet: Default::default(),
            args: Default::default(),
        }
    }

    pub fn peer(mut self, peer_pk: impl Into<String>) -> Self {
        self.tetraplet.peer_pk = peer_pk.into();
        self
    }

    pub fn service(mut self, service_id: impl Into<String>) -> Self {
        self.tetraplet.service_id = service_id.into();
        self
    }

    pub fn function(mut self, function_name: impl Into<String>) -> Self {
        self.tetraplet.function_name = function_name.into();
        self
    }

    pub fn json_path(mut self, json_path: impl Into<String>) -> Self {
        self.tetraplet.json_path = json_path.into();
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<JValue>>) -> Self {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn scalar(self) -> ExecutedState {
        let mut cid_state = ExecutionCidState::new();
        self.scalar_tracked(&mut cid_state)
    }

    pub fn unused(self) -> ExecutedState {
        let value_cid = value_to_json_cid(&self.result).unwrap();
        let value = ValueRef::Unused(value_cid.into());
        ExecutedState::Call(CallResult::Executed(value))
    }

    pub fn scalar_tracked(self, cid_state: &mut ExecutionCidState) -> ExecutedState {
        let service_result_agg_cid =
            value_aggregate_cid(self.result, self.tetraplet, self.args, cid_state);
        let value = ValueRef::Scalar(service_result_agg_cid);
        ExecutedState::Call(CallResult::Executed(value))
    }

    pub fn failed(self) -> ExecutedState {
        let mut cid_state = ExecutionCidState::new();
        self.failed_tracked(&mut cid_state)
    }

    pub fn failed_tracked(self, cid_state: &mut ExecutionCidState) -> ExecutedState {
        let service_result_agg_cid =
            value_aggregate_cid(self.result, self.tetraplet, self.args, cid_state);
        ExecutedState::Call(CallResult::Failed(service_result_agg_cid))
    }

    pub fn stream(self, generation: u32) -> ExecutedState {
        let mut cid_state = ExecutionCidState::new();
        self.stream_tracked(generation, &mut cid_state)
    }

    pub fn stream_tracked(
        self,
        generation: u32,
        cid_state: &mut ExecutionCidState,
    ) -> ExecutedState {
        let service_result_agg_cid =
            value_aggregate_cid(self.result, self.tetraplet, self.args, cid_state);
        let value = ValueRef::Stream {
            cid: service_result_agg_cid,
            generation,
        };
        ExecutedState::Call(CallResult::Executed(value))
    }
}

pub fn extract_service_result_cid(
    stream_exec_state: &ExecutedState,
) -> Rc<CID<ServiceResultAggregate>> {
    match stream_exec_state {
        ExecutedState::Call(CallResult::Executed(ValueRef::Stream { cid, .. })) => cid.clone(),
        ExecutedState::Call(CallResult::Executed(ValueRef::Scalar(cid))) => cid.clone(),
        _ => panic!("the function is intended for call results values only"),
    }
}

pub fn extract_canon_result_cid(canon_state: &ExecutedState) -> Rc<CID<CanonResultAggregate>> {
    match canon_state {
        ExecutedState::Canon(CanonResult(cid)) => cid.clone(),
        _ => panic!("the function is intended for canon only"),
    }
}

#[cfg(test)]
mod tests {
    use air::ExecutionCidState;
    use serde_json::json;

    #[test]
    fn test_scalar() {
        assert_eq!(scalar!(42), scalar!(42));
        assert_eq!(scalar!("test"), scalar!("test"));
        assert_ne!(scalar!(42), scalar!(42, peer = "test"));
        assert_ne!(
            scalar!(42, peer = "test"),
            scalar!(42, peer = "test", args = vec![json!(1)]),
        );
    }

    #[test]
    fn test_scalar_tracked() {
        let mut store = ExecutionCidState::new();
        assert_eq!(scalar_tracked!(42, store), scalar_tracked!(42, store));
        assert_eq!(scalar!(42), scalar_tracked!(42, store));
        assert_eq!(
            scalar_tracked!("test", store),
            scalar_tracked!("test", store)
        );
        assert_ne!(
            scalar_tracked!(42, store),
            scalar_tracked!(42, store, peer = "test")
        );
        assert_ne!(
            scalar_tracked!(42, store, peer = "test"),
            scalar_tracked!(42, store, peer = "test", args = vec![json!(1)]),
        );
        assert_eq!(
            scalar!(42, peer = "test", args = vec![json!(1)]),
            scalar_tracked!(42, store, peer = "test", args = vec![json!(1)]),
        );
    }
}
