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
    pub fn size(&self) -> Option<usize> {
        self.left_size
            .checked_add(self.right_size)
            .map(|v| v as usize)
    }
}

impl CallResult {
    pub fn sent_peer_id(peer_id: Rc<String>) -> CallResult {
        CallResult::RequestSentBy(Sender::PeerId(peer_id))
    }

    pub fn sent_peer_id_with_call_id(peer_id: Rc<String>, call_id: u32) -> CallResult {
        CallResult::RequestSentBy(Sender::PeerIdWithCallId { peer_id, call_id })
    }

    pub fn executed_scalar(value: Rc<JValue>) -> CallResult {
        let value = Value::Scalar(value);

        CallResult::Executed(value)
    }

    pub fn executed_stream(value: Rc<JValue>, generation: u32) -> CallResult {
        let value = Value::Stream { value, generation };

        CallResult::Executed(value)
    }

    pub fn failed(ret_code: i32, error_msg: impl Into<String>) -> CallResult {
        CallResult::CallServiceFailed(ret_code, Rc::new(error_msg.into()))
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
    pub fn new(res_generation: u32) -> Self {
        Self {
            res_generations: vec![res_generation],
        }
    }
}

impl CanonResult {
    pub fn new(canonicalized_element: JValue) -> Self {
        Self {
            canonicalized_element,
        }
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
            Call(Executed(value)) => {
                write!(f, "executed({value})")
            }
            Call(CallServiceFailed(ret_code, err_msg)) => {
                write!(f, r#"call_service_failed({ret_code}, "{err_msg}")"#)
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Scalar(value) => write!(f, "scalar: {value}"),
            Value::Stream { value, generation } => {
                write!(f, "stream: {value} generation: {generation}")
            }
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
