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
use air_test_utils::avm_runner::AVMRunner;
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
        current_peer_id: String,
        call_results: avm_interface::CallResults,
        tracing_params: String,
        tracing_output_mode: u8,
    ) -> anyhow::Result<avm_interface::raw_outcome::RawAVMOutcome> {
        let call_tracing = self.0.call_tracing(
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
        );

        let memory_stats = self.0.memory_stats();
        tracing::warn!(memory_size = memory_stats.memory_size);

        Ok(call_tracing?)
    }
}

pub(crate) fn create_wasm_avm_runner(
    air_interpreter_wasm_path: &Path,
    max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    Ok(Box::new(WasmAvmRunner(AVMRunner::new(
        air_interpreter_wasm_path.to_owned(),
        max_heap_size,
        0,
    )?)))
}
