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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    RequestSentBy(String),

    /// A corresponding call's been already executed with such value and result.
    Executed(Rc<JValue>),

    /// call_service ended with a service error.
    CallServiceFailed(i32, Rc<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutedState {
    Par(usize, usize),
    Call(CallResult),
}

impl std::fmt::Display for ExecutedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CallResult::*;
        use ExecutedState::*;

        match self {
            Par(left, right) => write!(f, "Par({}, {})", left, right),
            Call(RequestSentBy(peer_id)) => write!(f, "RequestSentBy({})", peer_id),
            Call(Executed(result)) => write!(f, "Executed({:?})", result),
            Call(CallServiceFailed(ret_code, err_msg)) => write!(f, "CallServiceFailed({}, {})", ret_code, err_msg),
        }
    }
}
