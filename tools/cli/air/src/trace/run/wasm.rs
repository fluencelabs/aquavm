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
use air_test_utils::avm_runner::AVMRunner;
use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use marine_wasmtime_backend::WasmtimeConfig;
use marine_wasmtime_backend::WasmtimeWasmBackend;

use std::error::Error as StdError;
use std::path::Path;

pub(crate) struct WasmAvmRunner(AVMRunner<WasmtimeWasmBackend>);

impl AirRunner for WasmAvmRunner {
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
        tracing_params: String,
        tracing_output_mode: u8,
        keypair: &KeyPair,
        particle_id: String,
    ) -> LocalBoxFuture<'this, eyre::Result<avm_interface::raw_outcome::RawAVMOutcome>> {
        let keypair = keypair.clone();
        async move {
            let call_tracing = self
                .0
                .call_tracing(
                    air,
                    prev_data,
                    data,
                    init_peer_id,
                    timestamp,
                    ttl,
                    current_peer_id,
                    call_results,
                    tracing_params,
                    tracing_output_mode,
                    keypair.key_format().into(),
                    keypair.secret().expect("Failed to get secret"),
                    particle_id,
                )
                .await;
            let memory_stats = self.0.memory_stats();
            tracing::warn!(memory_size = memory_stats.memory_size);

            Ok(call_tracing?)
        }
        .boxed_local()
    }
}

impl DataToHumanReadable for WasmAvmRunner {
    fn to_human_readable<'this>(
        &'this mut self,
        data: Vec<u8>,
    ) -> LocalBoxFuture<'this, Result<String, Box<dyn StdError>>> {
        async {
            self.0
                .to_human_readable_data(data)
                .await
                .map_err(|e| Box::new(e) as Box<dyn StdError>)
        }
        .boxed_local()
    }
}

pub(crate) async fn create_wasm_avm_runner(
    air_interpreter_wasm_path: &Path,
    max_heap_size: Option<u64>,
) -> eyre::Result<Box<WasmAvmRunner>> {
    let mut config = WasmtimeConfig::new();
    config
        .debug_info(true)
        .wasm_backtrace(true)
        .epoch_interruption(false);
    let wasm_backend = WasmtimeWasmBackend::new(config)?;
    Ok(Box::new(WasmAvmRunner(
        AVMRunner::new(
            air_interpreter_wasm_path.to_owned(),
            max_heap_size,
            0,
            wasm_backend,
        )
        .await?,
    )))
}
