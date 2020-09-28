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

use instructions::Instruction;
use stepper_outcome::StepperOutcome;

use fluence::fce;

pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub fn invoke(init_user_id: String, aqua: String, data: Vec<u8>) -> StepperOutcome {
    log::info!(
        "stepper invoked with user_id = {}, aqua = {:?}, data = {:?}",
        init_user_id,
        aqua,
        data
    );

    let outcome = StepperOutcome {
        data,
        next_peer_pks: vec![init_user_id],
    };

    let parsed_aqua = match serde_sexpr::from_str::<Vec<Instruction>>(&aqua) {
        Ok(parsed) => parsed,
        Err(e) => {
            log::error!("supplied aqua script can't be parsed: {:?}", e);

            return outcome;
        }
    };
    log::info!("parsed_aqua: {:?}", parsed_aqua);

    crate::stepper::execute(parsed_aqua);

    outcome
}

#[fce]
pub struct CallServiceResult {
    pub result: i32,
    pub outcome: Vec<u8>,
}

#[fce]
#[link(wasm_import_module = "aqua_test_module")]
extern "C" {
    pub fn call_service(service_id: String, fn_name: String, args: Vec<u8>) -> CallServiceResult;
}
