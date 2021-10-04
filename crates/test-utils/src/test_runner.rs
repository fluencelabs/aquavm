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
use avm_server::avm_runner::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct TestRunner {
    runner: AVMRunner,
    call_service: CallServiceClosure,
}

impl TestRunner {
    pub fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_user_id: impl Into<String>,
    ) -> Result<RawAVMOutcome, String> {
        let air = air.into();
        let mut prev_data = prev_data.into();
        let mut data = data.into();
        let init_user_id = init_user_id.into();
        let mut call_results = HashMap::new();

        let mut next_peer_pks = HashSet::new();

        loop {
            let mut outcome = self
                .runner
                .call(
                    air.clone(),
                    prev_data,
                    data,
                    init_user_id.clone(),
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

pub fn create_avm(
    call_service: CallServiceClosure,
    current_peer_id: impl Into<String>,
) -> TestRunner {
    let air_wasm_path = PathBuf::from("../target/wasm32-wasi/debug/air_interpreter_server.wasm");
    let current_peer_id = current_peer_id.into();
    let logging_mask = i32::MAX;

    let runner =
        AVMRunner::new(air_wasm_path, current_peer_id, logging_mask).expect("vm should be created");
    TestRunner {
        runner,
        call_service,
    }
}
