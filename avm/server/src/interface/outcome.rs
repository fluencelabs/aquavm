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

use super::CallRequests;
use crate::avm_runner::RawAVMOutcome;
use crate::AVMError;
use crate::AVMResult;

use serde::Deserialize;
use serde::Serialize;

use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AVMOutcome {
    /// Contains script data that should be preserved in an executor of this interpreter
    /// regardless of ret_code value.
    pub data: Vec<u8>,

    /// Collected parameters of all met call instructions that could be executed on a current peer.
    pub call_requests: CallRequests,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,

    /// Memory in bytes AVM linear heap was extended during execution by.
    pub memory_delta: usize,

    /// Time of particle execution
    /// (it count only execution time without operations with DataStore and so on)
    pub execution_time: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorAVMOutcome {
    pub error_code: i64,
    pub error_message: String,
    pub outcome: AVMOutcome,
}

impl AVMOutcome {
    pub(self) fn new(
        data: Vec<u8>,
        call_requests: CallRequests,
        next_peer_pks: Vec<String>,
        memory_delta: usize,
        execution_time: Duration,
    ) -> Self {
        Self {
            data,
            call_requests,
            next_peer_pks,
            memory_delta,
            execution_time,
        }
    }

    pub(crate) fn from_raw_outcome<E>(
        raw_outcome: RawAVMOutcome,
        memory_delta: usize,
        execution_time: Duration,
    ) -> AVMResult<Self, E> {
        use air_interpreter_interface::INTERPRETER_SUCCESS;

        let RawAVMOutcome {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
        } = raw_outcome;

        let avm_outcome = AVMOutcome::new(
            data,
            call_requests,
            next_peer_pks,
            memory_delta,
            execution_time,
        );

        if ret_code == INTERPRETER_SUCCESS {
            return Ok(avm_outcome);
        }

        let error_outcome = ErrorAVMOutcome::new(ret_code, error_message, avm_outcome);
        Err(AVMError::InterpreterFailed(error_outcome))
    }
}

impl ErrorAVMOutcome {
    pub(self) fn new(error_code: i64, error_msg: String, outcome: AVMOutcome) -> Self {
        Self {
            error_code,
            error_message: error_msg,
            outcome,
        }
    }
}
