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

use avm_interface::raw_outcome::RawAVMOutcome;
use avm_interface::CallResults;
use fluence_keypair::KeyPair;

use std::error::Error as StdError;

pub(crate) trait AirRunner {
    #[allow(clippy::too_many_arguments)]
    fn call_tracing(
        &mut self,
        air: String,
        prev_data: Vec<u8>,
        data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        current_peer_id: String,
        call_results: CallResults,
        tracing_params: String,
        tracing_output_mode: u8,
        key_pair: &KeyPair,
        particle_id: String,
    ) -> eyre::Result<RawAVMOutcome>;
}

pub(crate) trait DataToHumanReadable {
    fn to_human_readable(&mut self, data: Vec<u8>) -> Result<String, Box<dyn StdError>>;
}

/// This struct is used to set limits for the test runner creating AVMRunner.
#[derive(Debug, Default, Clone)]
pub struct TestInitParameters {
    pub air_size_limit: Option<u64>,
    pub particle_size_limit: Option<u64>,
    pub call_result_size_limit: Option<u64>,
}

impl TestInitParameters {
    pub fn to_attributes_w_default(&self) -> (u64, u64, u64) {
        use air_interpreter_interface::MAX_AIR_SIZE;
        use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
        use air_interpreter_interface::MAX_PARTICLE_SIZE;

        let air_size_limit = self.air_size_limit.unwrap_or(MAX_AIR_SIZE);
        let particle_size_limit: u64 = self.particle_size_limit.unwrap_or(MAX_PARTICLE_SIZE);
        let call_result_size_limit = self.call_result_size_limit.unwrap_or(MAX_CALL_RESULT_SIZE);
        (air_size_limit, particle_size_limit, call_result_size_limit)
    }

    pub fn no_limits() -> Self {
        Self {
            air_size_limit: Some(u64::MAX),
            particle_size_limit: Some(u64::MAX),
            call_result_size_limit: Some(u64::MAX),
        }
    }
}
