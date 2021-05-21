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

use crate::JValue;

use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParResult(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Scalar,
    Stream(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    RequestSentBy(String),

    /// A corresponding call's been already executed with such value and result.
    Executed(Rc<JValue>, ValueType),

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
/// So, this struct describes position inside overall execution trace belongs to one fold iteration.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FoldStatePositions {
    pub left_begin: usize,
    pub left_end: usize,
    pub right_begin: usize,
    pub right_end: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FoldResult(pub String, pub Vec<FoldStatePositions>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutedState {
    Par(ParResult),
    Call(CallResult),
    Fold(FoldResult),
}

impl ExecutedState {
    pub fn par(left: usize, right: usize) -> Self {
        Self::Par(ParResult(left, right))
    }
}

impl std::fmt::Display for ExecutedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CallResult::*;
        use ExecutedState::*;

        match self {
            Par(ParResult(left, right)) => write!(f, "Par({}, {})", left, right),
            Call(RequestSentBy(peer_id)) => write!(f, "RequestSentBy({})", peer_id),
            Call(Executed(result, value_type)) => write!(f, "Executed({:?} {:?})", result, value_type),
            Call(CallServiceFailed(ret_code, err_msg)) => write!(f, "CallServiceFailed({}, {})", ret_code, err_msg),
            Fold(FoldResult(iterable, states)) => write!(f, "Fold({}, {:?})", iterable, states),
        }
    }
}
