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
mod instructions;
mod stepper;
mod stepper_outcome;
mod execution;

use fluence::fce;
use crate::execution::exec;
use crate::stepper_outcome::{StepperOutcome};

pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub fn invoke(init_user_id: String, aqua: String, data: String) -> StepperOutcome {
    exec(init_user_id, aqua, data)
}

#[fce]
pub struct CallServiceResult {
    pub result: i32,
    pub outcome: String,
}

#[fce]
#[link(wasm_import_module = "aqua_test_module")]
extern "C" {
    pub fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult;
}
