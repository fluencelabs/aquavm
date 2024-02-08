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

use fluence_keypair::KeyPair;
use once_cell::sync::OnceCell;
use object_pool::Reusable;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use marine_wasmtime_backend::WasmtimeConfig;

use std::path::PathBuf;


// 10 Mb
const AVM_MAX_HEAP_SIZE: u64 = 10 * 1024 * 1024;
const AIR_WASM_PATH: &str = "../target/wasm32-wasi/debug/air_interpreter_server.wasm";
const RELEASE_AIR_WASM_PATH: &str = "../target/wasm32-wasi/release/air_interpreter_server.wasm";

pub struct WasmAirRunner {
    current_peer_id: String,
    runner: Reusable<'static, AVMRunner<WasmtimeWasmBackend>>,
}


fn create_wasm_backend() -> WasmtimeWasmBackend {
    let mut config = WasmtimeConfig::new();
    config
        .debug_info(true)
        .epoch_interruption(false)
        .wasm_backtrace(true);

    WasmtimeWasmBackend::new(config).unwrap()
}

async fn make_pooled_avm_runner() -> AVMRunner<WasmtimeWasmBackend> {
    let logging_mask = i32::MAX;
    let wasm_backend = create_wasm_backend();
    AVMRunner::new(
        PathBuf::from(AIR_WASM_PATH),
        Some(AVM_MAX_HEAP_SIZE),
        logging_mask,
        wasm_backend
    )
    .await
    .expect("vm should be created")
}

impl AirRunner for WasmAirRunner {
    fn new(current_peer_id: impl Into<String>) -> LocalBoxFuture<'static, Self> {
        let current_peer_id = current_peer_id.into();
        async move {
            static POOL_CELL: OnceCell<object_pool::Pool<AVMRunner<WasmtimeWasmBackend>>> = OnceCell::new();
            let pool = POOL_CELL.get_or_init(|| {
                object_pool::Pool::new(
                    // we create an empty pool and let it fill on demand
                    0,
                    || unreachable!(),
                )
            });

            let runner = match pool.try_pull() {
                Some(runner) => runner,
                None => Reusable::new(pool, make_pooled_avm_runner().await)
            };

            Self {
                current_peer_id: current_peer_id.into(),
                runner,
            }
        }.boxed_local()
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
            let current_peer_id =
                override_current_peer_id.unwrap_or_else(|| self.current_peer_id.clone());

            Ok(self.runner.call(
                air,
                prev_data,
                data,
                init_peer_id,
                timestamp,
                ttl,
                current_peer_id,
                call_results,
                &keypair,
                particle_id,
            ).await?)
        }.boxed_local()
    }

    fn get_current_peer_id(&self) -> &str {
        &self.current_peer_id
    }
}

/// WASM runner that runs release build form benchmarking.
pub struct ReleaseWasmAirRunner {
    current_peer_id: String,
    // these instances are not cached, as benches create relatively small number of instances
    runner: AVMRunner<WasmtimeWasmBackend>,
}

impl AirRunner for ReleaseWasmAirRunner {
    fn new(current_peer_id: impl Into<String>) -> LocalBoxFuture<'static, Self> {
        let current_peer_id = current_peer_id.into();
        async {
            let logging_mask = i32::MAX;

            let wasm_backend = create_wasm_backend();
            let runner = AVMRunner::new(
                PathBuf::from(RELEASE_AIR_WASM_PATH),
                Some(AVM_MAX_HEAP_SIZE),
                logging_mask,
                wasm_backend)
                .await
                .expect("vm should be created");

            Self {
                current_peer_id: current_peer_id.into(),
                runner,
            }
        }.boxed_local()
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
            let current_peer_id =
            override_current_peer_id.unwrap_or_else(|| self.current_peer_id.clone());

            Ok(self.runner.call(
                air,
                prev_data,
                data,
                init_peer_id,
                timestamp,
                ttl,
                current_peer_id,
                call_results,
                &keypair,
                particle_id,
            ).await?)
        }.boxed_local()
    }

    fn get_current_peer_id(&self) -> &str {
        &self.current_peer_id
    }
}
