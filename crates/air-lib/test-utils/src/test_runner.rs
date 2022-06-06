/*
 * Copyright 2020 Fluence Labs Limited
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

use super::CallServiceClosure;
#[cfg(feature = "test_with_native_code")]
use air_interpreter_interface::{CallServiceResult, RunParameters};
use avm_server::avm_runner::*;

#[cfg(not(feature = "test_with_native_code"))]
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::collections::HashSet;
#[cfg(not(feature = "test_with_native_code"))]
use std::path::PathBuf;

// 10 Mb
#[cfg(not(feature = "test_with_native_code"))]
const AVM_MAX_HEAP_SIZE: u64 = 10 * 1024 * 1024;
#[cfg(not(feature = "test_with_native_code"))]
const AIR_WASM_PATH: &str = "../target/wasm32-wasi/debug/air_interpreter_server.wasm";

#[cfg(not(feature = "test_with_native_code"))]
pub struct TestRunner {
    pub runner: object_pool::Reusable<'static, AVMRunner>,
    pub call_service: CallServiceClosure,
}

#[cfg(feature = "test_with_native_code")]
#[derive(Default)]
pub struct NativeAirRunner {
    current_peer_id: String,
}

#[cfg(feature = "test_with_native_code")]
impl NativeAirRunner {
    pub fn new(current_peer_id: impl Into<String>) -> Self {
        Self {
            current_peer_id: current_peer_id.into(),
        }
    }

    pub fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        call_results: avm_server::CallResults,
    ) -> Result<RawAVMOutcome, String> {
        // some inner parts transformations
        let raw_call_results: air_interpreter_interface::CallResults = call_results
            .into_iter()
            .map(|(call_id, call_result)| {
                (
                    call_id,
                    CallServiceResult {
                        ret_code: call_result.ret_code,
                        result: call_result.result.to_string(),
                    },
                )
            })
            .collect::<_>();
        let raw_call_results = serde_json::to_vec(&raw_call_results).unwrap();

        let outcome = air::execute_air(
            air.into(),
            prev_data.into(),
            data.into(),
            RunParameters {
                init_peer_id: init_peer_id.into(),
                current_peer_id: self.current_peer_id.clone(),
                timestamp,
                ttl,
            },
            raw_call_results,
        );
        let outcome =
            RawAVMOutcome::from_interpreter_outcome(outcome).map_err(|e| e.to_string())?;

        Ok(outcome)
    }
}

#[cfg(feature = "test_with_native_code")]
pub struct TestRunner {
    pub runner: NativeAirRunner,
    pub call_service: CallServiceClosure,
}

#[cfg(not(feature = "test_with_native_code"))]
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

#[derive(Debug, Default, Clone)]
pub struct TestRunParameters {
    pub init_peer_id: String,
    pub timestamp: u64,
    pub ttl: u32,
}

impl TestRunner {
    pub fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        test_run_params: TestRunParameters,
    ) -> Result<RawAVMOutcome, String> {
        let air = air.into();
        let mut prev_data = prev_data.into();
        let mut data = data.into();

        let TestRunParameters {
            init_peer_id,
            timestamp,
            ttl,
        } = test_run_params;

        let mut call_results = HashMap::new();
        let mut next_peer_pks = HashSet::new();

        loop {
            let mut outcome: RawAVMOutcome = self
                .runner
                .call(
                    air.clone(),
                    prev_data,
                    data,
                    init_peer_id.clone(),
                    timestamp,
                    ttl,
                    call_results,
                )
                .map_err(|e| e.to_string())?;

            next_peer_pks.extend(outcome.next_peer_pks);

            if outcome.call_requests.is_empty() {
                outcome.next_peer_pks = next_peer_pks.into_iter().collect::<Vec<_>>();
                return Ok(outcome);
            }

            call_results = outcome
                .call_requests
                .into_iter()
                .map(|(id, call_parameters)| {
                    let service_result = (self.call_service)(call_parameters);
                    (id, service_result)
                })
                .collect::<HashMap<_, _>>();

            prev_data = outcome.data;
            data = vec![];
        }
    }
}

#[cfg(not(feature = "test_with_native_code"))]
pub fn create_avm(
    call_service: CallServiceClosure,
    current_peer_id: impl Into<String>,
) -> TestRunner {
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

    TestRunner {
        runner,
        call_service,
    }
}

#[cfg(feature = "test_with_native_code")]
pub fn create_avm(
    call_service: CallServiceClosure,
    current_peer_id: impl Into<String>,
) -> TestRunner {
    TestRunner {
        runner: NativeAirRunner::new(current_peer_id),
        call_service,
    }
}

impl TestRunParameters {
    pub fn new(init_peer_id: impl Into<String>, timestamp: u64, ttl: u32) -> Self {
        Self {
            init_peer_id: init_peer_id.into(),
            timestamp,
            ttl,
        }
    }

    pub fn from_init_peer_id(init_peer_id: impl Into<String>) -> Self {
        Self {
            init_peer_id: init_peer_id.into(),
            timestamp: 0,
            ttl: 0,
        }
    }

    pub fn from_timestamp(timestamp: u64) -> Self {
        Self {
            init_peer_id: String::new(),
            timestamp,
            ttl: 0,
        }
    }

    pub fn from_ttl(ttl: u32) -> Self {
        Self {
            init_peer_id: String::new(),
            timestamp: 0,
            ttl,
        }
    }
}
