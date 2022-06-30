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
    fn call(
        &mut self,
        air: String,
        prev_data: Vec<u8>,
        data: Vec<u8>,
        init_peer_id: String,
        timestamp: u64,
        ttl: u32,
        call_results: avm_server::CallResults,
    ) -> anyhow::Result<air_test_utils::RawAVMOutcome> {
        Ok(self.0.call(
            air,
            prev_data,
            data,
            init_peer_id,
            timestamp,
            ttl,
            call_results,
        )?)
    }
}

pub(crate) fn create_wasm_avm_runner(
    current_peer_id: impl Into<String>,
    air_wasm_runtime_path: &Path,
    max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    let current_peer_id = current_peer_id.into();

    Ok(Box::new(WasmAvmRunner(AVMRunner::new(
        air_wasm_runtime_path.to_owned(),
        current_peer_id,
        max_heap_size,
        0,
    )?)))
}
