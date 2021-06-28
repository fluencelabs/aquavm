/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::call_service::Effect;
use crate::errors::AVMError::CreateVaultDirError;

use std::path::{Path, PathBuf};

pub fn create_vault_effect(particle_data_store: &Path, particle_id: &str) -> Effect<PathBuf> {
    let vault_dir = vault_dir(particle_data_store, particle_id);
    let closure = move || {
        std::fs::create_dir_all(&vault_dir)
            .map_err(|err| CreateVaultDirError(err, vault_dir.clone()))?;
        Ok(vault_dir.clone())
    };

    Box::new(closure)
}

pub fn vault_dir(particle_data_store: &Path, particle_id: &str) -> PathBuf {
    particle_data_store.join("vault").join(particle_id)
}

pub fn prev_data_file(particle_data_store: &Path, particle_id: &str) -> PathBuf {
    particle_data_store.join(particle_id)
}
