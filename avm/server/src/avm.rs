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

use super::AVMRunner;
use super::CallResults;
use crate::config::AVMConfig;
use crate::data_store::{particle_vault_dir, prev_data_file};
use crate::errors::AVMError::CleanupParticleError;
use crate::AVMError;
use crate::AVMResult;
use crate::InterpreterOutcome;

use std::path::PathBuf;

pub struct AVM {
    runner: AVMRunner,
    particle_data_store: PathBuf,
    vault_dir: PathBuf,
}

impl AVM {
    /// Create AVM with provided config.
    pub fn new(config: AVMConfig) -> AVMResult<Self> {
        use AVMError::{CreateVaultDirError, InvalidDataStorePath};

        let particle_data_store = config.particle_data_store;
        let vault_dir = config.vault_dir;

        let runner = AVMRunner::new(
            config.air_wasm_path,
            config.logging_mask,
            config.current_peer_id,
        )?;

        std::fs::create_dir_all(&particle_data_store)
            .map_err(|e| InvalidDataStorePath(e, particle_data_store.clone()))?;
        std::fs::create_dir_all(&vault_dir)
            .map_err(|e| CreateVaultDirError(e, vault_dir.clone()))?;

        let avm = Self {
            runner,
            particle_data_store,
            vault_dir,
        };

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
        use AVMError::PersistDataError;

        let init_user_id = init_user_id.into();

        let prev_data_path = prev_data_file(&self.particle_data_store, particle_id);
        // TODO: check for errors related to invalid file content (such as invalid UTF8 string)
        let prev_data = std::fs::read_to_string(&prev_data_path)
            .unwrap_or_default()
            .into_bytes();

        let outcome = self
            .runner
            .call(air, prev_data, data, init_user_id, call_results)?;

        // persist resulted data
        std::fs::write(&prev_data_path, &outcome.data)
            .map_err(|e| PersistDataError(e, prev_data_path))?;

        Ok(outcome)
    }

    /// Remove particle directories and files:
    /// - prev data file
    /// - particle file vault directory
    pub fn cleanup_particle(&self, particle_id: &str) -> AVMResult<()> {
        let prev_data = prev_data_file(&self.particle_data_store, particle_id);
        std::fs::remove_file(&prev_data).map_err(|err| CleanupParticleError(err, prev_data))?;

        let vault_dir = particle_vault_dir(&self.vault_dir, particle_id);
        std::fs::remove_dir_all(&vault_dir).map_err(|err| CleanupParticleError(err, vault_dir))?;

        Ok(())
    }
}
