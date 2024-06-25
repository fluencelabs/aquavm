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

use crate::CallSeDeErrors;

use super::CallRequests;

use air_interpreter_interface::InterpreterOutcome;

use air_interpreter_interface::SoftLimitsTriggering;
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
    pub soft_limits_triggering: SoftLimitsTriggering,
}

impl RawAVMOutcome {
    pub fn from_interpreter_outcome(outcome: InterpreterOutcome) -> Result<Self, CallSeDeErrors> {
        let InterpreterOutcome {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
            air_size_limit_exceeded,
            particle_size_limit_exceeded,
            call_result_size_limit_exceeded,
        } = outcome;

        let call_requests = crate::from_raw_call_requests(call_requests.into())?;
        let soft_limits_triggering = SoftLimitsTriggering::new(
            air_size_limit_exceeded,
            particle_size_limit_exceeded,
            call_result_size_limit_exceeded,
        );

        let raw_avm_outcome = Self {
            ret_code,
            error_message,
            data,
            call_requests,
            next_peer_pks,
            soft_limits_triggering,
        };

        Ok(raw_avm_outcome)
    }
}
