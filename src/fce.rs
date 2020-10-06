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

mod air;
mod defines;
mod errors;
mod instructions;
mod stepper;

pub(crate) use crate::defines::*;

use crate::stepper::execute_aqua;
use fluence::fce;

pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub fn invoke(init_user_id: String, aqua: String, data: String) -> StepperOutcome {
    execute_aqua(init_user_id, aqua, data)
}

#[fce]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult;
}
