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

use crate::call_evidence::CallEvidenceCtx;
use crate::AquamarineError::CallEvidenceSerializationError as CallSeError;
use crate::Result;
use fluence::fce;
use serde::{Deserialize, Serialize};

pub const STEPPER_SUCCESS: i32 = 0;

#[fce]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepperOutcome {
    /// A return code, where SUCCESS_ERROR_CODE means success.
    pub ret_code: i32,

    /// Contains error message if ret_code != SUCCESS_ERROR_CODE.
    pub error_message: String,

    /// Contains script data that should be preserved in an executor of this stepper
    /// regardless of ret_code value.
    pub data: String,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,
}

impl StepperOutcome {
    pub(crate) fn from_contexts(exec_ctx: ExecutionContext<'_>, call_ctx: &CallEvidenceCtx) -> Result<Self> {
        let data = serde_json::to_string(&call_ctx.new_path).map_err(CallSeError)?;
        let next_peer_pks = dedup(exec_ctx.next_peer_pks);

        let outcome = Self {
            ret_code: STEPPER_SUCCESS,
            error_message: String::new(),
            data,
            next_peer_pks,
        };

        Ok(outcome)
    }
}
