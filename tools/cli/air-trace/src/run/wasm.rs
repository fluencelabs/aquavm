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
use avm_server::avm_runner::AVMRunner;
use std::path::Path;

pub(crate) struct WasmAvmRunner(AVMRunner);

impl AirRunner for WasmAvmRunner {
    fn call_tracing(
        &mut self,
        air: String,
        prev_data: Vec<u8>,
        data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        call_results: avm_server::CallResults,
        tracing_params: String,
        tracing_output_mode: u8,
    ) -> anyhow::Result<air_test_utils::RawAVMOutcome> {
        Ok(self.0.call_tracing(
            air,
            prev_data,
            data,
            init_peer_id,
            timestamp,
            ttl,
            call_results,
            tracing_params,
            tracing_output_mode,
        )?)
    }
}

pub(crate) fn create_wasm_avm_runner(
    current_peer_id: impl Into<String>,
    air_interpreter_wasm_path: &Path,
    max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    let current_peer_id = current_peer_id.into();

    Ok(Box::new(WasmAvmRunner(AVMRunner::new(
        air_interpreter_wasm_path.to_owned(),
        current_peer_id,
        max_heap_size,
        0,
    )?)))
}
