/*
 * Copyright 2022 Fluence Labs Limited
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

use air_interpreter_interface::RunParameters;
use avm_server::avm_runner::*;
use avm_server::into_raw_result;

pub struct NativeAirRunner {
    current_peer_id: String,
}

impl NativeAirRunner {
    pub fn new(current_peer_id: impl Into<String>) -> Self {
        Self {
            current_peer_id: current_peer_id.into(),
        }
    }

    pub fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        call_results: avm_server::CallResults,
    ) -> Result<RawAVMOutcome, Box<dyn std::error::Error>> {
        // some inner parts transformations
        let raw_call_results = into_raw_result(call_results);
        let raw_call_results = serde_json::to_vec(&raw_call_results).unwrap();

        let outcome = air::execute_air(
            air.into(),
            prev_data.into(),
            data.into(),
            RunParameters {
                init_peer_id: init_peer_id.into(),
                current_peer_id: self.current_peer_id.clone(),
                timestamp,
                ttl,
            },
            raw_call_results,
        );
        let outcome = RawAVMOutcome::from_interpreter_outcome(outcome)?;

        Ok(outcome)
    }
}
