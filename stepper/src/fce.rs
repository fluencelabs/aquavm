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

#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod air;
mod call_evidence;
mod defines;
mod errors;
mod execution;
mod stepper_outcome;

pub(crate) use crate::defines::*;

use crate::execution::execute_aqua;
use fluence::fce;
use std::env::VarError;

const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub fn invoke(
    init_user_id: String,
    aqua: String,
    prev_data: String,
    data: String,
) -> StepperOutcome {
    execute_aqua(init_user_id, aqua, prev_data, data)
}

pub fn get_current_peer_id() -> std::result::Result<String, VarError> {
    std::env::var(CURRENT_PEER_ID_ENV_NAME)
}

#[fce]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult;
}
