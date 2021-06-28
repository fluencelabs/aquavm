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

use crate::avm::ParticleParameters;
use crate::CallServiceClosure;
use crate::IValue;

use std::path::PathBuf;

/// Describes behaviour of the AVM.
pub struct AVMConfig {
    /// Path to a AIR interpreter Wasm file.
    pub air_wasm_path: PathBuf,

    /// Descriptor of a closure that will be invoked on call_service call from the AIR interpreter.
    pub call_service: CallServiceClosure,

    /// Current peer id.
    pub current_peer_id: String,

    /// Path to a folder contains prev data.
    /// AVM uses it to store data obtained after interpreter execution, and load it as a prev_data by particle_id.
    pub particle_data_store: PathBuf,

    /// Mask used to filter logs, for details see `log_utf8_string` in fluence-faas.
    pub logging_mask: i32,
}
