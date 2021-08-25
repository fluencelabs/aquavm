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

use super::CallResults;
use crate::AVMError;
use crate::AVMResult;
use crate::InterpreterOutcome;

use fluence_faas::FluenceFaaS;
use fluence_faas::IValue;
use fluence_faas::{FaaSConfig, ModuleDescriptor};

use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

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
    /// file name of the AIR interpreter .wasm
    wasm_filename: String,
}

impl AVMRunner {
    /// Create AVM with provided config.
    pub fn new(
        air_wasm_path: PathBuf,
        current_peer_id: impl Into<String>,
        logging_mask: i32,
    ) -> AVMResult<Self> {
        let (wasm_dir, wasm_filename) = split_dirname(air_wasm_path)?;

        let faas_config = make_faas_config(wasm_dir, &wasm_filename, logging_mask);
        let faas = FluenceFaaS::with_raw_config(faas_config)?;
        let current_peer_id = current_peer_id.into();

        let avm = Self {
            faas: SendSafeFaaS(faas),
            current_peer_id,
            wasm_filename,
        };

        Ok(avm)
    }

    pub fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_user_id: impl Into<String>,
        call_results: &CallResults,
    ) -> AVMResult<InterpreterOutcome> {
        let init_user_id = init_user_id.into();
        let args = prepare_args(
            air,
            prev_data,
            data,
            init_user_id,
            self.current_peer_id.clone(),
            call_results,
        );

        let result =
            self.faas
                .call_with_ivalues(&self.wasm_filename, "invoke", &args, <_>::default())?;

        let result = try_as_one_value_vec(result)?;
        let outcome =
            InterpreterOutcome::from_ivalue(result).map_err(AVMError::InterpreterResultDeError)?;

        Ok(outcome)
    }
}

fn prepare_args(
    air: impl Into<String>,
    prev_data: impl Into<Vec<u8>>,
    data: impl Into<Vec<u8>>,
    init_peer_id: impl Into<String>,
    current_peer_id: String,
    call_results: &CallResults,
) -> Vec<IValue> {
    use fluence_faas::ne_vec::NEVec;

    let run_parameters = vec![
        IValue::String(init_peer_id.into()),
        IValue::String(current_peer_id),
    ];
    let run_parameters = NEVec::new(run_parameters).unwrap();

    let call_results =
        serde_json::to_vec(call_results).expect("the default serializer shouldn't fail");
    vec![
        IValue::String(air.into()),
        IValue::ByteArray(prev_data.into()),
        IValue::ByteArray(data.into()),
        IValue::Record(run_parameters),
        IValue::ByteArray(call_results),
    ]
}

/// Splits given path into its directory and file name
///
/// # Example
/// For path `/path/to/air_interpreter_server.wasm` result will be `Ok(PathBuf(/path/to), "air_interpreter_server.wasm")`
fn split_dirname(path: PathBuf) -> AVMResult<(PathBuf, String)> {
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

fn make_faas_config(air_wasm_dir: PathBuf, air_wasm_file: &str, logging_mask: i32) -> FaaSConfig {
    let air_module_config = fluence_faas::FaaSModuleConfig {
        mem_pages_count: None,
        logger_enabled: true,
        host_imports: <_>::default(),
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

fn try_as_one_value_vec(mut ivalues: Vec<IValue>) -> AVMResult<IValue> {
    use AVMError::IncorrectInterpreterResult;

    if ivalues.len() != 1 {
        return Err(IncorrectInterpreterResult(ivalues));
    }

    Ok(ivalues.remove(0))
}
