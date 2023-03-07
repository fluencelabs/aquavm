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

mod impls;
mod se_de;

use crate::JValue;
use crate::TracePos;

use air_interpreter_cid::CID;
use polyplets::SecurityTetraplet;
use se_de::par_serializer;
use se_de::sender_serializer;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ParResult {
    pub left_size: u32,
    pub right_size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sender {
    PeerId(Rc<String>),
    PeerIdWithCallId {
        peer_id: Rc<String>,
        call_id: u32,
        argument_hash: Rc<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    #[serde(with = "sender_serializer")]
    #[serde(rename = "sent_by")]
    RequestSentBy(Sender),

    /// A corresponding call's been already executed with such value as a result.
    Executed(ValueRef),

    /// The call returned a service error.
    ///
    /// The `JValue` has to be a two element array `[i32, String]`.
    Failed(Rc<CID<ServiceResultAggregate>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueRef {
    /// The call value is stored to a scalar variable.
    Scalar(Rc<CID<ServiceResultAggregate>>),
    /// The call value is stored to a stream variable.
    Stream {
        cid: Rc<CID<ServiceResultAggregate>>,
        generation: u32,
    },
    /// The call value is not stored.
    Unused(Rc<CID<ServiceResultAggregate>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallServiceFailed(pub i32, pub Rc<String>);

impl CallServiceFailed {
    pub fn new(ret_code: i32, message: Rc<String>) -> Self {
        Self(ret_code, message)
    }

    pub fn to_value(&self) -> JValue {
        serde_json::to_value(self).expect("default serializer shouldn't fail")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// A proof of service result execution result.
pub struct ServiceResultAggregate {
    pub value_cid: Rc<CID<JValue>>,
    /// Hash of the call arguments.
    pub argument_hash: Rc<str>,
    /// The tetraplet of the call result.
    pub tetraplet_cid: Rc<CID<SecurityTetraplet>>,
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
    #[serde(rename = "pos")]
    pub value_pos: TracePos,

    /// Descriptors of a subtrace that are corresponded to the current value. Technically, now
    /// it always contains two values, and Vec here is used to have a possibility to handle more
    /// than one next inside fold in future.
    #[serde(rename = "desc")]
    pub subtraces_desc: Vec<SubTraceDesc>,
}

/// Descriptor of a subtrace inside execution trace.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubTraceDesc {
    /// Start position in a trace of this subtrace.
    #[serde(rename = "pos")]
    pub begin_pos: TracePos,

    /// Length of the subtrace.
    #[serde(rename = "len")]
    pub subtrace_len: u32,
}

/// This type represents all information in an execution trace about states executed during
/// a fold execution.
pub type FoldLore = Vec<FoldSubTraceLore>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FoldResult {
    pub lore: FoldLore,
}

/// Describes result of applying functor `apply` to streams.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApResult {
    #[serde(rename = "gens")]
    pub res_generations: Vec<u32>,
}

/// Contains ids of element that were on a stream at the moment of an appropriate canon call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CanonResult {
    pub tetraplet: Rc<CID<SecurityTetraplet>>,
    pub values: Vec<Rc<CID<CanonCidAggregate>>>,
}

/// The type Canon trace CID refers to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonCidAggregate {
    pub value: Rc<CID<serde_json::Value>>,
    pub tetraplet: Rc<CID<SecurityTetraplet>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutedState {
    #[serde(with = "par_serializer")]
    Par(ParResult),
    Call(CallResult),
    Fold(FoldResult),
    Ap(ApResult),
    Canon(CanonResult),
}
