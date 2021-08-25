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

use super::AVMDataStore;
use super::AVMRunner;
use super::CallResults;
use crate::config::AVMConfig;
use crate::AVMResult;
use crate::InterpreterOutcome;

pub struct AVM {
    runner: AVMRunner,
    data_store: AVMDataStore,
}

impl AVM {
    /// Create AVM with provided config.
    pub fn new(config: AVMConfig) -> AVMResult<Self> {
        let AVMConfig {
            air_wasm_path,
            current_peer_id,
            logging_mask,
            mut data_store,
        } = config;

        data_store.initialize()?;

        let runner = AVMRunner::new(air_wasm_path, current_peer_id, logging_mask)?;
        let avm = Self { runner, data_store };

        Ok(avm)
    }

    pub fn call(
        &mut self,
        air: impl Into<String>,
        data: impl Into<Vec<u8>>,
        init_user_id: impl Into<String>,
        particle_id: &str,
        call_results: &CallResults,
    ) -> AVMResult<InterpreterOutcome> {
        let init_user_id = init_user_id.into();
        let prev_data = self.data_store.read_data(particle_id)?;

        let outcome = self
            .runner
            .call(air, prev_data, data, init_user_id, call_results)?;

        // persist resulted data
        self.data_store.store_data(&outcome.data, particle_id)?;

        Ok(outcome)
    }

    /// Cleanup data that become obsolete.
    pub fn cleanup_data(&mut self, particle_id: &str) -> AVMResult<()> {
        self.data_store.cleanup_data(particle_id)?;
        Ok(())
    }
}
