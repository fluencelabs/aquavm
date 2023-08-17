/*
 * Copyright 2023 Fluence Labs Limited
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

use air::execute_air;
use air::RunParameters;
use air_interpreter_interface::InterpreterOutcome;

use near_sdk::near_bindgen;
use near_sdk::borsh as borsh;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Aqua {}

#[near_bindgen]
impl Aqua {
    #[result_serializer(borsh)]
    pub fn execute_script(
        &self,
        #[serializer(borsh)] air_script: String,
        #[serializer(borsh)] prev_data: Vec<u8>,
        #[serializer(borsh)] current_data: Vec<u8>,
        #[serializer(borsh)] run_parameters: String,
        #[serializer(borsh)] call_results: String,
    ) -> String {
        let outcome = Self::execute(
            air_script,
            prev_data,
            current_data,
            run_parameters.into(),
            call_results.into(),
        );
        serde_json::to_string(&outcome).unwrap()
    }

    fn execute(
        air: String,
        prev_data: Vec<u8>,
        cur_data: Vec<u8>,
        params: Vec<u8>,
        call_results: Vec<u8>,
    ) -> InterpreterOutcome {
        let params: RunParameters =
            serde_json::from_slice(&params).expect("cannot parse RunParameters");

        execute_air(air, prev_data, cur_data, params, call_results)
    }
}
