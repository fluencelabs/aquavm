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

mod impls;
mod se_de;

use crate::GenerationIdx;
use crate::JValue;
use crate::RawValue;
use crate::TracePos;

use air_interpreter_cid::CID;
use polyplets::SecurityTetraplet;
use se_de::par_serializer;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::Formatter;
use std::rc::Rc;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub struct ParResult {
    pub left_size: u32,
    pub right_size: u32,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub enum Sender {
    PeerId(Rc<String>),
    PeerIdWithCallId { peer_id: Rc<String>, call_id: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    #[serde(rename = "sent_by")]
    RequestSentBy(Sender),

    /// A corresponding call's been already executed with such value as a result.
    Executed(ValueRef),

    /// The call returned a service error.
    ///
    /// The `JValue` has to be a two element array `[i32, String]`.
    Failed(CID<ServiceResultCidAggregate>),
}

/*
 * The current value structure is:
 *
 * ```
 * Scalar(CID<ServiceResultAggregate>) ---+
 *                                        |
 *   +----<service_result_store>------+
 *   |
 *   +-------> ServiceResultAggregate:
 *                value_cid ------------<value_store>----> JValue
 *                tetraplet_cid --------<tetraplet_store>----> SecurityTetraplet
 *                argument_hash: String
 * ```
 *
 * `Stream` variant is similar, however, `Unused` is different: it has value CID only, but the value
 * is not stored into the `value_store`:
 *
 * ```
 * Unused(CID<JValue>) ---> X
 * ```
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum ValueRef {
    /// The call value is stored to a scalar variable.
    Scalar(CID<ServiceResultCidAggregate>),
    /// The call value is stored to a stream variable.
    Stream {
        cid: CID<ServiceResultCidAggregate>,
        generation: GenerationIdx,
    },
    /// The call value is not stored.
    Unused(CID<JValue>),
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub struct CallServiceFailed {
    pub ret_code: i32,
    /// This field contains a JSON-serialized value, not a plain error message.
    pub message: Rc<String>,
}

impl CallServiceFailed {
    pub fn new(ret_code: i32, message: Rc<String>) -> Self {
        Self { ret_code, message }
    }

    pub fn to_value(&self) -> JValue {
        serde_json::to_value(self)
            .expect("serde_json serializer shouldn't fail")
            .into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
/// A proof of service result execution result.
pub struct ServiceResultCidAggregate {
    pub value_cid: CID<RawValue>,
    /// Hash of the call arguments.
    pub argument_hash: Rc<str>,
    /// The tetraplet of the call result.
    pub tetraplet_cid: CID<SecurityTetraplet>,
}

/// Let's consider an example of trace that could be produces by the following fold:
/// (fold $stream v
///     (call 1)
///     (call 2)
///     (next v)
///     (call 3)
///     (call 4)
/// )
///x
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
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
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
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
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
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct FoldResult {
    pub lore: FoldLore,
}

/// Describes result of applying functor `apply` to streams.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct ApResult {
    #[serde(rename = "gens")]
    pub res_generations: Vec<GenerationIdx>,
}

/// Contains ids of element that were on a stream at the moment of an appropriate canon call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum CanonResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    #[serde(rename = "sent_by")]
    RequestSentBy(Rc<String>),
    Executed(CID<CanonResultCidAggregate>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct CanonResultCidAggregate {
    pub tetraplet: CID<SecurityTetraplet>,
    pub values: Vec<CID<CanonCidAggregate>>,
}

/// The type Canon trace CID refers to.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub struct CanonCidAggregate {
    pub value: CID<RawValue>,
    pub tetraplet: CID<SecurityTetraplet>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum Provenance {
    Literal,
    ServiceResult {
        // the original call result CID; not changed on lambda application
        cid: CID<ServiceResultCidAggregate>,
    },
    Canon {
        // the original canon CID; not changed on lambda application
        cid: CID<CanonResultCidAggregate>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]
#[archive(check_bytes)]
pub enum ExecutedState {
    #[serde(with = "par_serializer")]
    Par(ParResult),
    Call(CallResult),
    Fold(FoldResult),
    Ap(ApResult),
    Canon(CanonResult),
}
