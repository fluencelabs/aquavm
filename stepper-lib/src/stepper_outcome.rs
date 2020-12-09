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

use fluence::fce;
use serde::{Deserialize, Serialize};

pub const STEPPER_SUCCESS: i32 = 0;

/// Describes a return value of the stepper.
#[fce]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepperOutcome {
    /// A return code, where SUCCESS_ERROR_CODE means success.
    pub ret_code: i32,

    /// Contains data if ret_code == 0, otherwise error message (that could be empty string).
    pub call_path: String,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,
}
