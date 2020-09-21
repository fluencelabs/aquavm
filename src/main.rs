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

use fluence::fce;
use log::info;

pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[fce]
pub struct StepperOutcome {
    pub aqua: String,
    pub next_peer_pks: Vec<String>,
}

#[fce]
pub fn invoke(init_user_id: String, aqua: String) -> StepperOutcome {
    info!("stepper invoked with user_id = {}, aqua = {:?}", init_user_id, aqua);

    let outcome = StepperOutcome {
        aqua,
        next_peer_pks: vec![init_user_id]
    };

    outcome
}
