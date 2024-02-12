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
use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use avm_interface::raw_outcome::RawAVMOutcome;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use std::error::Error as StdError;

pub(crate) struct NativeAvmRunner {}

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
    ) -> LocalBoxFuture<'this, anyhow::Result<RawAVMOutcome>> {
        let keypair = keypair.clone();
        async move {
            use air_interpreter_sede::ToSerialized;
            use avm_interface::into_raw_result;

            // some inner parts transformations
            let raw_call_results = into_raw_result(call_results);
            let raw_call_results = CallResultsRepr.serialize(&raw_call_results).unwrap();

            let key_format = keypair.key_format().into();
            let secret_key_bytes = keypair.secret().expect("Failed to get secret key");

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

pub(crate) fn create_native_avm_runner() -> anyhow::Result<Box<NativeAvmRunner>> {
    Ok(Box::new(NativeAvmRunner {}))
}
