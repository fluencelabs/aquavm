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

use super::CallRequests;
use crate::raw_outcome::RawAVMOutcome;

use air_interpreter_interface::SoftLimitsTriggering;
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

    /// Time of a particle execution
    /// (it counts only execution time without operations with DataStore and so on)
    pub execution_time: Duration,

    /// To store and convey soft limits triggering flags.
    pub soft_limits_triggering: SoftLimitsTriggering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorAVMOutcome {
    pub error_code: i64,
    pub error_message: String,
    pub outcome: AVMOutcome,
}

impl AVMOutcome {
    fn new(
        data: Vec<u8>,
        call_requests: CallRequests,
        next_peer_pks: Vec<String>,
        memory_delta: usize,
        execution_time: Duration,
        soft_limits_triggering: SoftLimitsTriggering,
    ) -> Self {
        Self {
            data,
            call_requests,
            next_peer_pks,
            memory_delta,
            execution_time,
            soft_limits_triggering,
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn from_raw_outcome(
        raw_outcome: RawAVMOutcome,
        memory_delta: usize,
        execution_time: Duration,
    ) -> Result<Self, ErrorAVMOutcome> {
        use air_interpreter_interface::INTERPRETER_SUCCESS;

        let RawAVMOutcome {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
            soft_limits_triggering,
        } = raw_outcome;

        let avm_outcome = AVMOutcome::new(
            data,
            call_requests,
            next_peer_pks,
            memory_delta,
            execution_time,
            soft_limits_triggering,
        );

        if ret_code == INTERPRETER_SUCCESS {
            Ok(avm_outcome)
        } else {
            Err(ErrorAVMOutcome::new(ret_code, error_message, avm_outcome))
        }
    }
}

impl ErrorAVMOutcome {
    fn new(error_code: i64, error_message: String, outcome: AVMOutcome) -> Self {
        Self {
            error_code,
            error_message,
            outcome,
        }
    }
}
