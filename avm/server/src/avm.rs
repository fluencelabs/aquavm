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

use parking_lot::Mutex;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

const CALL_SERVICE_NAME: &str = "call_service";
const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

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

/// Information about the particle that is being executed by the interpreter at the moment
#[derive(Debug, Default, Clone)]
pub struct ParticleParameters {
    pub init_user_id: String,
    pub particle_id: String,
    pub execution_id: String,
}

pub struct AVM {
    faas: SendSafeFaaS,
    particle_data_store: PathBuf,
    vault_dir: PathBuf,
    /// file name of the AIR interpreter .wasm
    wasm_filename: String,
    /// information about the particle that is being executed at the moment
    current_particle: Arc<Mutex<ParticleParameters>>,
}

impl AVM {
    /// Create AVM with provided config.
    pub fn new(config: AVMConfig) -> Result<Self> {
        use AVMError::{CreateVaultDirError, InvalidDataStorePath};

        let current_particle: Arc<Mutex<ParticleParameters>> = <_>::default();
        let particle_data_store = config.particle_data_store;
        let vault_dir = config.vault_dir;
        let call_service = call_service_descriptor(
            current_particle.clone(),
            config.call_service,
            vault_dir.clone(),
        );
        let (wasm_dir, wasm_filename) = split_dirname(config.air_wasm_path)?;

        let faas_config = make_faas_config(
            wasm_dir,
            &wasm_filename,
            call_service,
            config.current_peer_id,
            config.logging_mask,
        );
        let faas = FluenceFaaS::with_raw_config(faas_config)?;

        std::fs::create_dir_all(&particle_data_store)
            .map_err(|e| InvalidDataStorePath(e, particle_data_store.clone()))?;
        std::fs::create_dir_all(&vault_dir)
            .map_err(|e| CreateVaultDirError(e, vault_dir.clone()))?;

        let avm = Self {
            faas: SendSafeFaaS(faas),
            particle_data_store,
            vault_dir,
            wasm_filename,
            current_particle,
        };

        Ok(avm)
    }

    pub fn call(
        &mut self,
        init_user_id: impl Into<String>,
        air: impl Into<String>,
        data: impl Into<Vec<u8>>,
        particle_id: impl Into<String>,
        execution_id: impl Into<String>,
    ) -> Result<InterpreterOutcome> {
        use AVMError::PersistDataError;

        let particle_id = particle_id.into();
        let init_user_id = init_user_id.into();

        let prev_data_path = prev_data_file(&self.particle_data_store, &particle_id);
        // TODO: check for errors related to invalid file content (such as invalid UTF8 string)
        let prev_data = std::fs::read_to_string(&prev_data_path)
            .unwrap_or_default()
            .into_bytes();

        let args = prepare_args(prev_data, data, init_user_id.clone(), air);

        // Update ParticleParams with the new values so subsequent calls to `call_service` can use them
        self.update_current_particle(particle_id, init_user_id, execution_id);

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome =
            InterpreterOutcome::from_ivalues(result).map_err(AVMError::InterpreterResultDeError)?;

        // persist resulted data
        std::fs::write(&prev_data_path, &outcome.data)
            .map_err(|e| PersistDataError(e, prev_data_path))?;

        Ok(outcome)
    }

    /// Remove particle directories and files:
    /// - prev data file
    /// - particle file vault directory
    pub fn cleanup_particle(&self, particle_id: &str) -> Result<()> {
        let prev_data = prev_data_file(&self.particle_data_store, particle_id);
        std::fs::remove_file(&prev_data).map_err(|err| CleanupParticleError(err, prev_data))?;

        let vault_dir = particle_vault_dir(&self.vault_dir, particle_id);
        std::fs::remove_dir_all(&vault_dir).map_err(|err| CleanupParticleError(err, vault_dir))?;

        Ok(())
    }

    fn update_current_particle(
        &self,
        particle_id: String,
        init_user_id: String,
        execution_id: String,
    ) {
        let mut params = self.current_particle.lock();
        params.particle_id = particle_id;
        params.init_user_id = init_user_id;
        params.execution_id = execution_id;
    }
}

fn prepare_args(
    prev_data: Vec<u8>,
    data: impl Into<Vec<u8>>,
    init_user_id: String,
    air: impl Into<String>,
) -> Vec<IValue> {
    vec![
        IValue::String(init_user_id),
        IValue::String(air.into()),
        IValue::ByteArray(prev_data),
        IValue::ByteArray(data.into()),
    ]
}

fn call_service_descriptor(
    params: Arc<Mutex<ParticleParameters>>,
    call_service: CallServiceClosure,
    vault_dir: PathBuf,
) -> HostImportDescriptor {
    let call_service_closure: HostExportedFunc = Box::new(move |_, ivalues: Vec<IValue>| {
        let params = {
            let lock = params.lock();
            lock.deref().clone()
        };

        let create_vault = create_vault_effect(&vault_dir, &params.particle_id);

        let args = CallServiceArgs {
            particle_parameters: params,
            function_args: ivalues,
            create_vault,
        };
        call_service(args)
    });

    HostImportDescriptor {
        host_exported_func: call_service_closure,
        argument_types: vec![IType::String, IType::String, IType::String, IType::String],
        output_type: Some(IType::Record(0)),
        error_handler: None,
    }
}

/// Splits given path into its directory and file name
///
/// # Example
/// For path `/path/to/air_interpreter_server.wasm` result will be `Ok(PathBuf(/path/to), "air_interpreter_server.wasm")`
fn split_dirname(path: PathBuf) -> Result<(PathBuf, String)> {
    use AVMError::InvalidAIRPath;

    let metadata = path.metadata().map_err(|err| InvalidAIRPath {
        invalid_path: path.clone(),
        reason: "failed to get file's metadata (doesn't exist or invalid permissions)",
        io_error: Some(err),
    })?;

    if !metadata.is_file() {
        return Err(InvalidAIRPath {
            invalid_path: path,
            reason: "is not a file",
            io_error: None,
        });
    }

    let file_name = path
        .file_name()
        .expect("checked to be a file, file name must be defined");
    let file_name = file_name.to_string_lossy().into_owned();

    let mut path = path;
    // drop file name from path
    path.pop();

    Ok((path, file_name))
}

fn make_faas_config(
    air_wasm_dir: PathBuf,
    air_wasm_file: &str,
    call_service: HostImportDescriptor,
    current_peer_id: String,
    logging_mask: i32,
) -> FaaSConfig {
    use fluence_faas::FaaSModuleConfig;
    use maplit::hashmap;

    let host_imports = hashmap! {
        String::from(CALL_SERVICE_NAME) => call_service
    };

    let mut air_module_config = FaaSModuleConfig {
        mem_pages_count: None,
        logger_enabled: true,
        host_imports,
        wasi: None,
        logging_mask,
    };

    let envs = hashmap! {
        CURRENT_PEER_ID_ENV_NAME.as_bytes().to_vec() => current_peer_id.into_bytes(),
    };
    air_module_config.extend_wasi_envs(envs);

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
impl AVM {
    pub fn call_with_prev_data(
        &mut self,
        init_user_id: impl Into<String>,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
    ) -> Result<InterpreterOutcome> {
        let args = prepare_args(prev_data.into(), data, init_user_id.into(), air);

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let outcome =
            InterpreterOutcome::from_ivalues(result).map_err(AVMError::InterpreterResultDeError)?;

        Ok(outcome)
    }
}
