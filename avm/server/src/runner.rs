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

use crate::call_service::CallServiceArgs;
use crate::config::AVMConfig;
use crate::data_store::{create_vault_effect, particle_vault_dir, prev_data_file};
use crate::errors::AVMError::CleanupParticleError;
use crate::AVMError;
use crate::InterpreterOutcome;
use crate::{CallServiceClosure, IType, Result};

use fluence_faas::FluenceFaaS;
use fluence_faas::HostImportDescriptor;
use fluence_faas::IValue;
use fluence_faas::{FaaSConfig, HostExportedFunc, ModuleDescriptor};
use air_interpreter_interface::RunParameters;

use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

/// A newtype needed to mark it as `unsafe impl Send`
struct SendSafeFaaS(FluenceFaaS);

/// Mark runtime as Send, so libp2p on the node (use-site) is happy
unsafe impl Send for SendSafeFaaS {}

impl Deref for SendSafeFaaS {
    type Target = FluenceFaaS;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SendSafeFaaS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AVMRunner {
    faas: SendSafeFaaS,
    current_peer_id: String,
}

impl AVMRunner {
    /// Create AVM with provided config.
    pub fn new(config: AVMConfig) -> Result<Self> {
        let (wasm_dir, wasm_filename) = split_dirname(config.air_wasm_path)?;

        let faas_config = make_faas_config(
            wasm_dir,
            &wasm_filename,
            config.logging_mask,
        );
        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        let avm = Self {
            faas: SendSafeFaaS(faas),
            current_peer_id: config.current_peer_id,
        };

        Ok(avm)
    }

    pub fn call(
        &mut self,
        init_user_id: impl Into<String>,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
    ) -> Result<InterpreterOutcome> {
        let init_user_id = init_user_id.into();
        let args = prepare_args(prev_data, data, init_user_id.clone(), air);

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome =
            InterpreterOutcome::from_ivalues(result).map_err(AVMError::InterpreterResultDeError)?;

        Ok(outcome)
    }
}

fn prepare_args(
    prev_data: impl Into<Vec<u8>>,
    data: impl Into<Vec<u8>>,
    init_user_id: impl Into<String>,
    current_peer_id: String,
    air: impl Into<String>,
) -> Vec<IValue> {
    vec![
        IValue::String(init_user_id.into()),
        IValue::String(air.into()),
        IValue::ByteArray(prev_data.into()),
        IValue::ByteArray(data.into()),
    ]
}

fn make_faas_config(
    air_wasm_dir: PathBuf,
    air_wasm_file: &str,
    logging_mask: i32,
) -> FaaSConfig {
    let mut air_module_config = fluence_faas::FaaSModuleConfig {
        mem_pages_count: None,
        logger_enabled: true,
        host_imports,
        wasi: None,
        logging_mask,
    };

    FaaSConfig {
        modules_dir: Some(air_wasm_dir),
        modules_config: vec![ModuleDescriptor {
            file_name: String::from(air_wasm_file),
            import_name: String::from(air_wasm_file),
            config: air_module_config,
        }],
        default_modules_config: None,
    }
}

// This API is intended for testing purposes
#[cfg(feature = "raw-avm-api")]
impl AVMRunner {
    pub fn call_with_prev_data(
        &mut self,
        init_user_id: impl Into<String>,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
    ) -> Result<InterpreterOutcome> {
        let args = prepare_args(prev_data, data, init_user_id, air);

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome =
            InterpreterOutcome::from_ivalues(result).map_err(AVMError::InterpreterResultDeError)?;

        Ok(outcome)
    }
}
