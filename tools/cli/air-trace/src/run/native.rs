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

use super::runner::AirRunner;
use air_interpreter_interface::RunParameters;
use avm_interface::raw_outcome::RawAVMOutcome;

struct NativeAvmRunner {
    current_peer_id: String,
}

impl AirRunner for NativeAvmRunner {
    fn call_tracing(
        &mut self,
        air: String,
        prev_data: Vec<u8>,
        data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        call_results: avm_interface::CallResults,
        // We use externally configured logger.
        _tracing_params: String,
        _tracing_output_mode: u8,
    ) -> anyhow::Result<RawAVMOutcome> {
        use avm_interface::into_raw_result;

        // some inner parts transformations
        let raw_call_results = into_raw_result(call_results);
        let raw_call_results = serde_json::to_vec(&raw_call_results).unwrap();

        let outcome = air::execute_air(
            air,
            prev_data,
            data,
            RunParameters {
                init_peer_id,
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

pub(crate) fn create_native_avm_runner(
    current_peer_id: impl Into<String>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    Ok(Box::new(NativeAvmRunner {
        current_peer_id: current_peer_id.into(),
    }))
}
