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

use crate::test_runner::AirRunner;
use avm_server::avm_runner::*;

use once_cell::sync::OnceCell;
use std::path::PathBuf;

// 10 Mb
const AVM_MAX_HEAP_SIZE: u64 = 10 * 1024 * 1024;
const AIR_WASM_PATH: &str = "../target/wasm32-wasi/debug/air_interpreter_server.wasm";

pub struct WasmAirRunner(object_pool::Reusable<'static, AVMRunner>);

fn make_pooled_avm_runner() -> AVMRunner {
    let fake_current_peer_id = "";
    let logging_mask = i32::MAX;

    AVMRunner::new(
        PathBuf::from(AIR_WASM_PATH),
        fake_current_peer_id,
        Some(AVM_MAX_HEAP_SIZE),
        logging_mask,
    )
    .expect("vm should be created")
}

impl AirRunner for WasmAirRunner {
    fn new(current_peer_id: impl Into<String>) -> Self {
        static POOL_CELL: OnceCell<object_pool::Pool<AVMRunner>> = OnceCell::new();

        let pool = POOL_CELL.get_or_init(|| {
            object_pool::Pool::new(
                // we create an empty pool and let it fill on demand
                0,
                || unreachable!(),
            )
        });

        let mut runner = pool.pull(make_pooled_avm_runner);
        runner.set_peer_id(current_peer_id);

        Self(runner)
    }

    fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        override_current_peer_id: Option<String>,
        call_results: avm_server::CallResults,
    ) -> Result<RawAVMOutcome, Box<dyn std::error::Error>> {
        Ok(self.0.call(
            air,
            prev_data,
            data,
            init_peer_id,
            timestamp,
            ttl,
            override_current_peer_id,
            call_results,
        )?)
    }
}
