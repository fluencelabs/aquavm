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

use super::runner::AirRunner;
use super::runner::DataToHumanReadable;
use super::runner::TestInitParameters;

use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use avm_interface::raw_outcome::RawAVMOutcome;
use avm_server::AquaVMRuntimeLimits;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use std::error::Error as StdError;

pub(crate) struct NativeAvmRunner {
    pub aquavm_runtime_limits: AquaVMRuntimeLimits,
}

impl AirRunner for NativeAvmRunner {
    fn call_tracing<'this>(
        &'this mut self,
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
    ) -> LocalBoxFuture<'this, eyre::Result<RawAVMOutcome>> {
        let keypair = keypair.clone();
        async move {
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
        .boxed_local()
    }
}

impl DataToHumanReadable for NativeAvmRunner {
    fn to_human_readable<'this>(
        &'this mut self,
        data: Vec<u8>,
    ) -> LocalBoxFuture<'this, Result<String, Box<dyn StdError>>> {
        async move { air::to_human_readable_data(data) }.boxed_local()
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
