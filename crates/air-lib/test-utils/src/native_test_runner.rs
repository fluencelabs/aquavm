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

use crate::prelude::TestInitParameters;
use crate::test_runner::AirRunner;
use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use air_interpreter_sede::ToSerialized;
use avm_server::avm_runner::*;
use avm_server::into_raw_result;
use avm_server::AquaVMRuntimeLimits;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

pub struct NativeAirRunner {
    current_peer_id: String,
    test_init_parameters: TestInitParameters,
}

impl NativeAirRunner {
    fn new(current_peer_id: impl Into<String>, test_init_parameters: TestInitParameters) -> Self {
        Self {
            current_peer_id: current_peer_id.into(),
            test_init_parameters,
        }
    }
}
impl AirRunner for NativeAirRunner {
    fn new(
        current_peer_id: impl Into<String>,
        test_init_parameters: TestInitParameters,
    ) -> LocalBoxFuture<'static, Self> {
        let current_peer_id = current_peer_id.into();
        async move { Self::new(current_peer_id, test_init_parameters) }.boxed_local()
    }

    fn call<'this>(
        &'this mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        override_current_peer_id: Option<String>,
        call_results: avm_server::CallResults,
        keypair: &KeyPair,
        particle_id: String,
    ) -> LocalBoxFuture<'this, Result<RawAVMOutcome, Box<dyn std::error::Error + 'this>>> {
        let air = air.into();
        let prev_data = prev_data.into();
        let data = data.into();
        let init_peer_id = init_peer_id.into();
        let keypair = keypair.clone();
        async move {
            // some inner parts transformations
            let raw_call_results = into_raw_result(call_results);
            let raw_call_results = CallResultsRepr.serialize(&raw_call_results).unwrap();

            let current_peer_id =
                override_current_peer_id.unwrap_or_else(|| self.current_peer_id.clone());

            let key_format = keypair.key_format().into();
            let secret_key_bytes = keypair.secret().unwrap();

            let AquaVMRuntimeLimits {
                air_size_limit,
                particle_size_limit,
                call_result_size_limit,
                hard_limit_enabled,
            } = self.test_init_parameters.into();

            let outcome = air::execute_air(
                air.into(),
                prev_data.into(),
                data.into(),
                RunParameters {
                    init_peer_id: init_peer_id.into(),
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

    fn get_current_peer_id(&self) -> &str {
        &self.current_peer_id
    }
}
