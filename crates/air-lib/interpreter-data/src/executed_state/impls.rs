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

impl ParResult {
    pub fn new(left_size: u32, right_size: u32) -> Self {
        Self {
            left_size,
            right_size,
        }
    }

    /// Returns a size of subtrace that this par describes in execution_step trace.
    pub fn size(&self) -> Option<u32> {
        self.left_size.checked_add(self.right_size)
    }
}

impl CallResult {
    pub fn sent_peer_id(peer_id: Rc<String>) -> CallResult {
        CallResult::RequestSentBy(Sender::PeerId(peer_id))
    }

    pub fn sent_peer_id_with_call_id(peer_id: Rc<String>, call_id: u32) -> CallResult {
        CallResult::RequestSentBy(Sender::PeerIdWithCallId { peer_id, call_id })
    }

    pub fn executed_service_result(value_ref: ValueRef) -> Self {
        Self::Executed(value_ref)
    }

    pub fn executed_scalar(service_result_agg_cid: Rc<CID<ServiceResultCidAggregate>>) -> Self {
        Self::executed_service_result(ValueRef::Scalar(service_result_agg_cid))
    }

    pub fn executed_stream_stub(
        cid: Rc<CID<ServiceResultCidAggregate>>,
    ) -> CallResult {
        let generation = GenerationIdx::stub();
        let value = ValueRef::Stream { cid, generation };
        CallResult::Executed(value)
    }

    pub fn executed_unused(value_cid: Rc<CID<JValue>>) -> CallResult {
        Self::executed_service_result(ValueRef::Unused(value_cid))
    }

    pub fn failed(service_result_agg_cid: Rc<CID<ServiceResultCidAggregate>>) -> CallResult {
        CallResult::Failed(service_result_agg_cid)
    }

    pub fn get_cid(&self) -> Option<Rc<CID<ServiceResultCidAggregate>>> {
        match self {
            CallResult::RequestSentBy(_) => None,
            CallResult::Executed(executed) => match executed {
                ValueRef::Scalar(cid) => Some(cid.clone()),
                ValueRef::Stream { cid, .. } => Some(cid.clone()),
                ValueRef::Unused(_) => None,
            },
            CallResult::Failed(cid) => Some(cid.clone()),
        }
    }
}

impl SubTraceDesc {
    pub fn new(begin_pos: TracePos, subtrace_len: usize) -> Self {
        Self {
            begin_pos,
            subtrace_len: subtrace_len as _,
        }
    }
}

impl ExecutedState {
    pub fn par(left_subgraph_size: usize, right_subgraph_size: usize) -> Self {
        let par_result = ParResult {
            left_size: left_subgraph_size as _,
            right_size: right_subgraph_size as _,
        };

        Self::Par(par_result)
    }
}

impl ApResult {
    pub fn new(res_generation: GenerationIdx) -> Self {
        Self {
            res_generations: vec![res_generation],
        }
    }

    pub fn stub() -> Self {
        Self {
            res_generations: vec![GenerationIdx::stub()],
        }
    }
}

impl CanonResult {
    pub fn new(cid: Rc<CID<CanonResultCidAggregate>>) -> Self {
        Self(cid)
    }
}

impl CanonResultCidAggregate {
    pub fn new(
        tetraplet: Rc<CID<SecurityTetraplet>>,
        values: Vec<Rc<CID<CanonCidAggregate>>>,
    ) -> Self {
        Self { tetraplet, values }
    }
}

impl Provenance {
    #[inline]
    pub fn literal() -> Self {
        Self::Literal
    }

    #[inline]
    pub fn service_result(cid: Rc<CID<ServiceResultCidAggregate>>) -> Self {
        Self::ServiceResult { cid }
    }

    #[inline]
    pub fn canon(cid: Rc<CID<CanonResultCidAggregate>>) -> Self {
        Self::Canon { cid }
    }
}

impl std::fmt::Display for ExecutedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CallResult::*;
        use ExecutedState::*;

        match self {
            Par(ParResult {
                left_size: left_subgraph_size,
                right_size: right_subgraph_size,
            }) => write!(f, "par({left_subgraph_size}, {right_subgraph_size})"),
            Call(RequestSentBy(sender)) => write!(f, r"{sender}"),
            Call(Executed(value_ref)) => {
                write!(f, "executed({value_ref:?})")
            }
            Call(Failed(failed_cid)) => {
                write!(f, "failed({failed_cid:?})")
            }
            Fold(FoldResult { lore }) => {
                writeln!(f, "fold(",)?;
                for sublore in lore {
                    writeln!(
                        f,
                        "          {} - [{}, {}], [{}, {}]",
                        sublore.value_pos,
                        sublore.subtraces_desc[0].begin_pos,
                        sublore.subtraces_desc[0].subtrace_len,
                        sublore.subtraces_desc[1].begin_pos,
                        sublore.subtraces_desc[1].subtrace_len
                    )?;
                }
                write!(f, "     )")
            }
            Ap(ap) => {
                write!(f, "ap: _ -> {:?}", ap.res_generations)
            }
            Canon(_) => {
                write!(f, "canon [<object>]")
            }
        }
    }
}

impl std::fmt::Display for ValueRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueRef::Scalar(cid) => write!(f, "scalar: {cid:?}"),
            ValueRef::Stream { cid, generation } => {
                write!(f, "stream: {cid:?} generation: {generation}")
            }
            ValueRef::Unused(cid) => write!(f, "unused: {cid:?}"),
        }
    }
}

impl std::fmt::Display for Sender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sender::PeerId(peer_id) => write!(f, "request_sent_by({peer_id})"),
            Sender::PeerIdWithCallId { peer_id, call_id } => {
                write!(f, "request_sent_by({peer_id}: {call_id})")
            }
        }
    }
}
