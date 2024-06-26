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

use avm_server::avm_runner::*;

use fluence_keypair::KeyPair;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use marine_wasmtime_backend::WasmtimeConfig;
use marine_wasmtime_backend::WasmtimeWasmBackend;
use object_pool::Reusable;
use once_cell::sync::OnceCell;

use std::path::PathBuf;

// 100 Mb
const AVM_MAX_HEAP_SIZE: u64 = 100 * 1024 * 1024;
const AIR_WASM_PATH: &str = "../target/wasm32-wasi/debug/air_interpreter_server.wasm";
const RELEASE_AIR_WASM_PATH: &str = "../target/wasm32-wasi/release/air_interpreter_server.wasm";

pub struct WasmAirRunner {
    current_peer_id: String,
    runner: Reusable<'static, AVMRunner<WasmtimeWasmBackend>>,
}

fn create_wasm_backend() -> WasmtimeWasmBackend {
    let mut config = WasmtimeConfig::default();
    config
        .debug_info(true)
        .epoch_interruption(false)
        .wasm_backtrace(true);

    WasmtimeWasmBackend::new(config).unwrap()
}

async fn make_pooled_avm_runner(
    test_init_parameters: TestInitParameters,
) -> AVMRunner<WasmtimeWasmBackend> {
    let logging_mask = i32::MAX;
    let wasm_backend = create_wasm_backend();
    AVMRunner::new(
        PathBuf::from(AIR_WASM_PATH),
        Some(AVM_MAX_HEAP_SIZE),
        test_init_parameters.into(),
        logging_mask,
        wasm_backend,
    )
    .await
    .expect("vm should be created")
}

impl AirRunner for WasmAirRunner {
    fn new(
        current_peer_id: impl Into<String>,
        test_init_parameters: TestInitParameters,
    ) -> LocalBoxFuture<'static, Self> {
        let current_peer_id = current_peer_id.into();
        async move {
            static POOL_CELL: OnceCell<object_pool::Pool<AVMRunner<WasmtimeWasmBackend>>> =
                OnceCell::new();
            let pool = POOL_CELL.get_or_init(|| {
                object_pool::Pool::new(
                    // we create an empty pool and let it fill on demand
                    0,
                    || unreachable!(),
                )
            });

            let runner = match pool.try_pull() {
                Some(runner) => runner,
                None => Reusable::new(pool, make_pooled_avm_runner(test_init_parameters).await),
            };

            Self {
                current_peer_id: current_peer_id.into(),
                runner,
            }
        }
        .boxed_local()
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

            Ok(self
                .runner
                .call(
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
                )
                .await?)
        }
        .boxed_local()
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
    fn new(
        current_peer_id: impl Into<String>,
        test_init_parameters: TestInitParameters,
    ) -> LocalBoxFuture<'static, Self> {
        let current_peer_id = current_peer_id.into();
        async move {
            let logging_mask = i32::MAX;

            let wasm_backend = create_wasm_backend();
            let runner = AVMRunner::new(
                PathBuf::from(RELEASE_AIR_WASM_PATH),
                Some(AVM_MAX_HEAP_SIZE),
                test_init_parameters.into(),
                logging_mask,
                wasm_backend,
            )
            .await
            .expect("vm should be created");

            Self {
                current_peer_id: current_peer_id.into(),
                runner,
            }
        }
        .boxed_local()
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

            Ok(self
                .runner
                .call(
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
                )
                .await?)
        }
        .boxed_local()
    }

    fn get_current_peer_id(&self) -> &str {
        &self.current_peer_id
    }
}
