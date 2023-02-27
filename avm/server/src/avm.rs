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
use crate::config::AVMConfig;
use crate::AVMResult;

use avm_data_store::AnomalyData;
use avm_interface::raw_outcome::RawAVMOutcome;
use avm_interface::AVMOutcome;
use avm_interface::CallResults;
use avm_interface::ParticleParameters;

use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;
use std::time::Instant;

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
    #[allow(clippy::result_large_err)]
    pub fn new(config: AVMConfig<E>) -> AVMResult<Self, E> {
        let AVMConfig {
            air_wasm_path,
            max_heap_size,
            logging_mask,
            mut data_store,
        } = config;

        data_store.initialize()?;

        let runner = AVMRunner::new(air_wasm_path, max_heap_size, logging_mask)
            .map_err(AVMError::RunnerError)?;
        let runner = SendSafeRunner(runner);
        let avm = Self { runner, data_store };

        Ok(avm)
    }

    #[allow(clippy::result_large_err)]
    pub fn call(
        &mut self,
        air: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_parameters: ParticleParameters<'_>,
        call_results: CallResults,
    ) -> AVMResult<AVMOutcome, E> {
        let air = air.into();
        let storage_key = store_key_from_particle(&particle_parameters);
        let prev_data = self.data_store.read_data(&storage_key)?;
        let current_data = data.into();

        let execution_start_time = Instant::now();
        let memory_size_before = self.memory_stats().memory_size;
        let outcome = self
            .runner
            .call(
                air.clone(),
                prev_data,
                current_data.clone(),
                particle_parameters.init_peer_id.clone().into_owned(),
                particle_parameters.timestamp,
                particle_parameters.ttl,
                particle_parameters.current_peer_id.clone(),
                call_results.clone(),
            )
            .map_err(AVMError::RunnerError)?;

        let execution_time = execution_start_time.elapsed();
        let memory_delta = self.memory_stats().memory_size - memory_size_before;
        if self
            .data_store
            .detect_anomaly(execution_time, memory_delta, &outcome)
        {
            self.save_anomaly_data(
                &air,
                &current_data,
                &call_results,
                &particle_parameters,
                &outcome,
                execution_time,
                memory_delta,
            )?;
        }

        // persist resulted data
        self.data_store.store_data(&outcome.data, &storage_key)?;
        let outcome = AVMOutcome::from_raw_outcome(outcome, memory_delta, execution_time)
            .map_err(AVMError::InterpreterFailed)?;

        Ok(outcome)
    }

    /// Cleanup data that become obsolete.
    #[allow(clippy::result_large_err)]
    pub fn cleanup_data(&mut self, particle_id: &str, current_peer_id: &str) -> AVMResult<(), E> {
        let store_key = store_key_from_components(particle_id, current_peer_id);
        self.data_store.cleanup_data(&store_key)?;
        Ok(())
    }

    /// Return memory stat of an interpreter heap.
    pub fn memory_stats(&self) -> AVMMemoryStats {
        self.runner.memory_stats()
    }

    #[allow(clippy::result_large_err, clippy::too_many_arguments)]
    fn save_anomaly_data(
        &mut self,
        air_script: &str,
        current_data: &[u8],
        call_result: &CallResults,
        particle_parameters: &ParticleParameters<'_>,
        avm_outcome: &RawAVMOutcome,
        execution_time: Duration,
        memory_delta: usize,
    ) -> AVMResult<(), E> {
        let store_key = store_key_from_particle(particle_parameters);
        let prev_data = self.data_store.read_data(&store_key)?;
        let call_results = serde_json::to_vec(call_result).map_err(AVMError::AnomalyDataSeError)?;
        let ser_particle =
            serde_json::to_vec(particle_parameters).map_err(AVMError::AnomalyDataSeError)?;
        let ser_avm_outcome =
            serde_json::to_vec(avm_outcome).map_err(AVMError::AnomalyDataSeError)?;

        let anomaly_data = AnomalyData::new(
            air_script,
            &ser_particle,
            &prev_data,
            current_data,
            &call_results,
            &ser_avm_outcome,
            execution_time,
            memory_delta,
        );

        self.data_store
            .collect_anomaly_data(&store_key, anomaly_data)
            .map_err(Into::into)
    }
}

fn store_key_from_particle(params: &ParticleParameters<'_>) -> String {
    store_key_from_components(&params.particle_id, &params.current_peer_id)
}

fn store_key_from_components(particle_id: &str, current_peer_id: &str) -> String {
    format!("particle_{}-peer_{}", particle_id, current_peer_id)
}
