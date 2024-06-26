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
use fluence_keypair::KeyPair;

use marine_wasm_backend_traits::WasmBackend;

use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;
use std::time::Instant;

/// A newtype needed to mark it as `unsafe impl Send`
struct SendSafeRunner<WB: WasmBackend>(AVMRunner<WB>);

/// Mark runtime as Send, so libp2p on the node (use-site) is happy
unsafe impl<WB: WasmBackend> Send for SendSafeRunner<WB> {}

impl<WB: WasmBackend> Deref for SendSafeRunner<WB> {
    type Target = AVMRunner<WB>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<WB: WasmBackend> DerefMut for SendSafeRunner<WB> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AVM<E, WB: WasmBackend> {
    runner: SendSafeRunner<WB>,
    data_store: AVMDataStore<E>,
}

impl<E, WB: WasmBackend> AVM<E, WB> {
    /// Create AVM with provided config.
    #[allow(clippy::result_large_err)]
    pub async fn new(config: AVMConfig<E>, wasm_backend: WB) -> AVMResult<Self, E> {
        let AVMConfig {
            air_wasm_path,
            max_heap_size,
            logging_mask,
            mut data_store,
        } = config;

        data_store.initialize()?;

        let runner = AVMRunner::new(
            air_wasm_path,
            max_heap_size,
            <_>::default(),
            logging_mask,
            wasm_backend,
        )
        .await
        .map_err(AVMError::RunnerError)?;
        let runner = SendSafeRunner(runner);
        let avm = Self { runner, data_store };

        Ok(avm)
    }

    #[allow(clippy::result_large_err)]
    pub async fn call(
        &mut self,
        air: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_parameters: ParticleParameters<'_>,
        call_results: CallResults,
        keypair: &KeyPair,
    ) -> AVMResult<AVMOutcome, E> {
        let air = air.into();
        let prev_data = self.data_store.read_data(
            &particle_parameters.particle_id,
            &particle_parameters.current_peer_id,
        )?;
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
                keypair,
                particle_parameters.particle_id.to_string(),
            )
            .await
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
        self.data_store.store_data(
            &outcome.data,
            &particle_parameters.particle_id,
            &particle_parameters.current_peer_id,
        )?;
        let outcome = AVMOutcome::from_raw_outcome(outcome, memory_delta, execution_time)
            .map_err(AVMError::InterpreterFailed)?;

        Ok(outcome)
    }

    /// Cleanup data that become obsolete.
    #[allow(clippy::result_large_err)]
    pub fn cleanup_data(&mut self, particle_id: &str, current_peer_id: &str) -> AVMResult<(), E> {
        self.data_store.cleanup_data(particle_id, current_peer_id)?;
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
        let prev_data = self.data_store.read_data(
            &particle_parameters.particle_id,
            &particle_parameters.current_peer_id,
        )?;
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
            .collect_anomaly_data(
                &particle_parameters.particle_id,
                &particle_parameters.current_peer_id,
                anomaly_data,
            )
            .map_err(Into::into)
    }
}
