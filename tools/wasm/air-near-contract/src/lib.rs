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

use air::execute_air;
use air::RunParameters;
use air_interpreter_interface::InterpreterOutcome;

use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
pub struct Aqua {}

#[near]
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

        execute_air(air, prev_data, cur_data, params, call_results.into())
    }
}
