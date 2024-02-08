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
use super::runner::DataToHumanReadable;
use super::runner::TestInitParameters;

use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use avm_interface::raw_outcome::RawAVMOutcome;
use avm_server::AquaVMRuntimeLimits;
use fluence_keypair::KeyPair;

use std::error::Error as StdError;

pub(crate) struct NativeAvmRunner {
    pub aquavm_runtime_limits: AquaVMRuntimeLimits,
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
        current_peer_id: String,
        call_results: avm_interface::CallResults,
        // We use externally configured logger.
        _tracing_params: String,
        _tracing_output_mode: u8,
        keypair: &KeyPair,
        particle_id: String,
    ) -> eyre::Result<RawAVMOutcome> {
        use air_interpreter_sede::ToSerialized;
        use avm_interface::into_raw_result;

        // some inner parts transformations
        let raw_call_results = into_raw_result(call_results);
        let raw_call_results = CallResultsRepr.serialize(&raw_call_results).unwrap();

        let key_format = keypair.key_format().into();
        let secret_key_bytes = keypair.secret().expect("Failed to get secret key");
        let AquaVMRuntimeLimits {
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        } = self.aquavm_runtime_limits;

        let outcome = air::execute_air(
            air,
            prev_data,
            data,
            RunParameters {
                init_peer_id,
                current_peer_id,
                timestamp,
                ttl,
                key_format,
                secret_key_bytes,
                particle_id,
                air_size_limit,
                particle_size_limit,
                call_result_size_limit,
                hard_limit_enabled,
            },
            raw_call_results,
        );
        let outcome = RawAVMOutcome::from_interpreter_outcome(outcome)?;

        Ok(outcome)
    }
}

impl DataToHumanReadable for NativeAvmRunner {
    fn to_human_readable(&mut self, data: Vec<u8>) -> Result<String, Box<dyn StdError>> {
        air::to_human_readable_data(data)
    }
}

pub(crate) fn create_native_avm_runner(
    test_init_parameters: TestInitParameters,
) -> eyre::Result<Box<NativeAvmRunner>> {
    let aquavm_runtime_limits: AquaVMRuntimeLimits = test_init_parameters.into();

    Ok(Box::new(NativeAvmRunner {
        aquavm_runtime_limits,
    }))
}
