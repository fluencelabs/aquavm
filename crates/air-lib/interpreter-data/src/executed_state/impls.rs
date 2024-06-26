/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::*;
use crate::RawValue;

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

    pub fn executed_scalar(service_result_agg_cid: CID<ServiceResultCidAggregate>) -> Self {
        Self::executed_service_result(ValueRef::Scalar(service_result_agg_cid))
    }

    pub fn executed_stream_stub(cid: CID<ServiceResultCidAggregate>) -> CallResult {
        let generation = GenerationIdx::stub();
        let value = ValueRef::Stream { cid, generation };
        CallResult::Executed(value)
    }

    pub fn executed_unused(value_cid: CID<JValue>) -> CallResult {
        Self::executed_service_result(ValueRef::Unused(value_cid))
    }

    pub fn failed(service_result_agg_cid: CID<ServiceResultCidAggregate>) -> CallResult {
        CallResult::Failed(service_result_agg_cid)
    }

    pub fn get_cid(&self) -> Option<&CID<ServiceResultCidAggregate>> {
        match self {
            CallResult::RequestSentBy(_) => None,
            CallResult::Executed(executed) => executed.get_cid(),
            CallResult::Failed(cid) => Some(cid),
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
    pub fn executed(cid: CID<CanonResultCidAggregate>) -> Self {
        CanonResult::Executed(cid)
    }

    pub fn request_sent_by(peer_id: Rc<String>) -> Self {
        CanonResult::RequestSentBy(peer_id)
    }
}

impl CanonResultCidAggregate {
    pub fn new(tetraplet: CID<SecurityTetraplet>, values: Vec<CID<CanonCidAggregate>>) -> Self {
        Self { tetraplet, values }
    }
}

impl CanonCidAggregate {
    pub fn new(
        value: CID<RawValue>,
        tetraplet: CID<SecurityTetraplet>,
        provenance: Provenance,
    ) -> Self {
        Self {
            value,
            tetraplet,
            provenance,
        }
    }
}

impl ServiceResultCidAggregate {
    pub fn new(
        value_cid: CID<RawValue>,
        argument_hash: Rc<str>,
        tetraplet_cid: CID<SecurityTetraplet>,
    ) -> Self {
        Self {
            value_cid,
            argument_hash,
            tetraplet_cid,
        }
    }
}

impl Provenance {
    #[inline]
    pub fn literal() -> Self {
        Self::Literal
    }

    #[inline]
    pub fn service_result(cid: CID<ServiceResultCidAggregate>) -> Self {
        Self::ServiceResult { cid }
    }

    #[inline]
    pub fn canon(cid: CID<CanonResultCidAggregate>) -> Self {
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

impl ValueRef {
    pub(crate) fn get_cid(&self) -> Option<&CID<ServiceResultCidAggregate>> {
        match self {
            ValueRef::Scalar(cid) => Some(cid),
            ValueRef::Stream { cid, .. } => Some(cid),
            ValueRef::Unused(_) => None,
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
