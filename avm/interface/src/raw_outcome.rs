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

use crate::CallSeDeErrors;

use super::CallRequests;

use air_interpreter_interface::InterpreterOutcome;

use serde::Deserialize;
use serde::Serialize;

/// This struct is very similar to AVMOutcome, but keeps error_code and error_msg for test purposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawAVMOutcome {
    pub ret_code: i64,
    pub error_message: String,
    pub data: Vec<u8>,
    pub call_requests: CallRequests,
    pub next_peer_pks: Vec<String>,
}

impl RawAVMOutcome {
    pub fn from_interpreter_outcome(outcome: InterpreterOutcome) -> Result<Self, CallSeDeErrors> {
        let InterpreterOutcome {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
            cid: _,
        } = outcome;

        let call_requests = crate::from_raw_call_requests(call_requests)?;

        let raw_avm_outcome = Self {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
        };

        Ok(raw_avm_outcome)
    }
}
