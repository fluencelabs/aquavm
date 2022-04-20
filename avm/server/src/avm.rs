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

use super::avm_runner::AVMRunner;
use super::AVMDataStore;
use super::AVMError;
use super::AVMMemoryStats;
use super::AVMOutcome;
use super::CallResults;
use crate::config::AVMConfig;
use crate::interface::ParticleParameters;
use crate::AVMResult;

use std::ops::Deref;
use std::ops::DerefMut;

/// A newtype needed to mark it as `unsafe impl Send`
struct SendSafeRunner(AVMRunner);

/// Mark runtime as Send, so libp2p on the node (use-site) is happy
unsafe impl Send for SendSafeRunner {}

impl Deref for SendSafeRunner {
    type Target = AVMRunner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SendSafeRunner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AVM<E> {
    runner: SendSafeRunner,
    data_store: AVMDataStore<E>,
}

impl<E> AVM<E> {
    /// Create AVM with provided config.
    pub fn new(config: AVMConfig<E>) -> AVMResult<Self, E> {
        let AVMConfig {
            air_wasm_path,
            current_peer_id,
            max_heap_size,
            logging_mask,
            mut data_store,
        } = config;

        data_store.initialize()?;

        let runner = AVMRunner::new(air_wasm_path, current_peer_id, max_heap_size, logging_mask)
            .map_err(AVMError::RunnerError)?;
        let runner = SendSafeRunner(runner);
        let avm = Self { runner, data_store };

        Ok(avm)
    }

    pub fn call(
        &mut self,
        air: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_parameters: ParticleParameters<'_, '_>,
        call_results: CallResults,
    ) -> AVMResult<AVMOutcome, E> {
        let particle_id = particle_parameters.particle_id.as_str();
        let prev_data = self.data_store.read_data(particle_id)?;

        let outcome = self
            .runner
            .call(
                air,
                prev_data,
                data,
                particle_parameters.init_peer_id.into_owned(),
                particle_parameters.timestamp,
                call_results,
            )
            .map_err(AVMError::RunnerError)?;

        // persist resulted data
        self.data_store.store_data(&outcome.data, particle_id)?;
        let outcome = AVMOutcome::from_raw_outcome(outcome)?;

        Ok(outcome)
    }

    /// Cleanup data that become obsolete.
    pub fn cleanup_data(&mut self, particle_id: &str) -> AVMResult<(), E> {
        self.data_store.cleanup_data(particle_id)?;
        Ok(())
    }

    /// Return memory stat of an interpreter heap.
    pub fn memory_stats(&self) -> AVMMemoryStats {
        self.runner.memory_stats()
    }
}
