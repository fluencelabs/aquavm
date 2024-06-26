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
use crate::trace::run::runner::TestInitParameters;
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
    test_init_parameters: TestInitParameters,
) -> eyre::Result<Box<WasmAvmRunner>> {
    let mut config = WasmtimeConfig::default();
    config
        .debug_info(true)
        .wasm_backtrace(true)
        .epoch_interruption(false);
    let wasm_backend = WasmtimeWasmBackend::new(config)?;
    Ok(Box::new(WasmAvmRunner(
        AVMRunner::new(
            air_interpreter_wasm_path.to_owned(),
            max_heap_size,
            test_init_parameters.into(),
            0,
            wasm_backend,
        )
        .await?,
    )))
}
