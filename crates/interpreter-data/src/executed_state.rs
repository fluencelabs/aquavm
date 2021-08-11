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

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JValue;
use std::rc::Rc;

pub const SCALAR_GENERATION: u32 = 0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParResult(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    RequestSentBy(Rc<String>),

    /// A corresponding call's been already executed with such value as a result.
    /// The second value is a generation, for scalar it means nothing and should be equal zero,
    /// for streams it's used for merging.
    Executed(Rc<JValue>, u32),

    /// call_service ended with a service error.
    CallServiceFailed(i32, Rc<String>),
}

/// Let's consider an example of trace that could be produces by the following fold:
/// (fold $stream v
///     (call 1)
///     (call 2)
///     (next v)
///     (call 3)
///     (call 4)
/// )
///
/// Having started with stream with two elements {v1, v2} the resulted trace would looks like
/// [(1) (2)] [(1) (2)] [(3) (4)] [(3) (4)]  <---  the sequence of call states
///    v1        v2        v2        v1      <---- corresponding values from $stream that
///                                                the iterable v had at the moment of call
///
/// From this example, it could be seen that each instruction sequence inside fold is divided into
/// two intervals (left and right), each of these intervals has borders [begin, end).
/// So, this struct describes position inside overall execution_step trace belongs to one fold iteration.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FoldSubTraceLore {
    /// Position of current value in a trace.
    pub value_pos: u32,

    /// Descriptors of a subtrace that are corresponded to the current value. Technically, now
    /// it always contains two values, and Vec here is used to have a possibility to handle more
    /// than one next inside fold in future.
    pub subtraces_desc: Vec<SubTraceDesc>,
}

/// Descriptor of a subtrace inside execution trace.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubTraceDesc {
    /// Start position in a trace of this subtrace.
    pub begin_pos: u32,

    /// Length of the subtrace.
    pub subtrace_len: u32,
}

/// This type represents all information in an execution trace about states executed during
/// a fold execution.
pub type FoldLore = Vec<FoldSubTraceLore>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FoldResult(pub FoldLore);

/// Describes result of applying functor `apply` to streams. This functor has the
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApResult {
    pub src_generations: Vec<u32>,
    pub dst_generations: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutedState {
    Par(ParResult),
    Call(CallResult),
    Fold(FoldResult),
    Ap(ApResult),
}

impl ParResult {
    /// Returns a size of subtrace that this par describes in execution_step trace.
    pub fn size(&self) -> Option<usize> {
        self.0.checked_add(self.1).map(|v| v as usize)
    }
}

impl CallResult {
    pub fn sent(sender: impl Into<String>) -> CallResult {
        CallResult::RequestSentBy(Rc::new(sender.into()))
    }

    pub fn executed_scalar(value: JValue) -> CallResult {
        CallResult::Executed(Rc::new(value), 0)
    }

    pub fn executed_stream(value: JValue, generation: u32) -> CallResult {
        CallResult::Executed(Rc::new(value), generation)
    }

    pub fn failed(ret_code: i32, error_msg: impl Into<String>) -> CallResult {
        CallResult::CallServiceFailed(ret_code, Rc::new(error_msg.into()))
    }
}

impl SubTraceDesc {
    pub fn new(begin_pos: usize, subtrace_len: usize) -> Self {
        Self {
            begin_pos: begin_pos as _,
            subtrace_len: subtrace_len as _,
        }
    }
}

impl ExecutedState {
    pub fn par(left: usize, right: usize) -> Self {
        Self::Par(ParResult(left as _, right as _))
    }
}

impl std::fmt::Display for ExecutedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CallResult::*;
        use ExecutedState::*;

        match self {
            Par(ParResult(left, right)) => write!(f, "par({}, {})", left, right),
            Call(RequestSentBy(peer_id)) => write!(f, r#"request_sent_by("{}")"#, peer_id),
            Call(Executed(result, generation)) => {
                write!(f, "executed({:?}, {})", result, generation)
            }
            Call(CallServiceFailed(ret_code, err_msg)) => {
                write!(f, r#"call_service_failed({}, "{}")"#, ret_code, err_msg)
            }
            Fold(FoldResult(lore)) => {
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
            },
            Ap(ap) => {
                write!(f, "ap: {:?} -> {:?}", ap.src_generations, ap.dst_generations)
            },
        }
    }
}
