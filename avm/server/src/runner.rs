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

use crate::RunnerError;
use crate::RunnerResult;

use air_interpreter_interface::try_as_string;
use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::InterpreterOutcome;
use air_interpreter_sede::ToSerialized;
use air_utils::measure;
use avm_interface::raw_outcome::RawAVMOutcome;
use avm_interface::CallResults;
use fluence_keypair::KeyPair;
use marine::generic::Marine;
use marine::generic::MarineConfig;
use marine::generic::ModuleDescriptor;
use marine::IValue;
use marine_wasm_backend_traits::WasmBackend;

use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub struct AquaVMRuntimeLimits {
    pub air_size_limit: u64, // WIP remove pub?
    /// The particle data size limit.
    pub particle_size_limit: u64,
    /// This is the limit for the size of service call result.
    pub call_result_size_limit: u64,
    /// This knob controls hard RAM limits behavior for AVMRunner.
    pub hard_limit_enabled: bool,
}

#[derive(Default)]
pub struct AVMRuntimeLimits {
    // The AIR script size limit.
    pub air_size_limit: Option<u64>,
    /// The particle data size limit.
    pub particle_size_limit: Option<u64>,
    /// This is the limit for the size of service call result.
    pub call_result_size_limit: Option<u64>,
    /// This knob controls hard RAM limits behavior for AVMRunner.
    pub hard_limit_enabled: bool,
}

pub struct AVMRunner<WB: WasmBackend> {
    marine: Marine<WB>,
    /// file name of the AIR interpreter .wasm
    wasm_filename: String,
    /// The memory limit provided by constructor
    total_memory_limit: Option<u64>,
    /// This struct contains runtime RAM allowance.
    aquavm_runtime_limits: AquaVMRuntimeLimits,
}

/// Return statistic of AVM server Wasm module heap footprint.
pub struct AVMMemoryStats {
    /// Size of currently used linear memory in bytes.
    /// Please note that linear memory contains not only heap, but globals, shadow stack and so on.
    pub memory_size: usize,
    /// Possibly set max memory size for AVM server.
    pub total_memory_limit: Option<u64>,
    /// Number of allocations rejected due to memory limit.
    /// May be not recorded by some backends in Marine.
    pub allocation_rejects: Option<u32>,
}

impl<WB: WasmBackend> AVMRunner<WB> {
    /// Create AVM with the provided config.
    pub async fn new(
        air_wasm_path: PathBuf,
        total_memory_limit: Option<u64>,
        avm_runtime_limits: AVMRuntimeLimits,
        logging_mask: i32,
        wasm_backend: WB,
    ) -> RunnerResult<Self> {
        let (wasm_dir, wasm_filename) = split_dirname(air_wasm_path)?;

        let marine_config =
            make_marine_config(wasm_dir, &wasm_filename, total_memory_limit, logging_mask);
        let marine = Marine::with_raw_config(wasm_backend, marine_config).await?;
        let aquavm_runtime_limits = avm_runtime_limits.into();

        let avm = Self {
            marine,
            wasm_filename,
            total_memory_limit,
            aquavm_runtime_limits,
        };

        Ok(avm)
    }

    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip_all)]
    pub async fn call(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        current_peer_id: impl Into<String>,
        call_results: CallResults,
        keypair: &KeyPair,
        particle_id: String,
    ) -> RunnerResult<RawAVMOutcome> {
        let key_format = keypair.key_format();
        // we use secret() for compatibility with JS client that doesn't have keypair type,
        // it can serialize a secret key only
        let secret_key_bytes: Vec<u8> = keypair.secret().map_err(RunnerError::KeyError)?;

        let args = prepare_args(
            air,
            prev_data,
            data,
            current_peer_id.into(),
            init_peer_id.into(),
            timestamp,
            ttl,
            self.aquavm_runtime_limits,
            call_results,
            key_format.into(),
            secret_key_bytes,
            particle_id,
        );

        let result = measure!(
            self.marine
                .call_with_ivalues_async(&self.wasm_filename, "invoke", &args, <_>::default())
                .await?,
            tracing::Level::INFO,
            "marine.call_with_ivalues",
            method = "invoke",
        );

        let result = try_as_one_value_vec(result)?;
        let outcome = InterpreterOutcome::from_ivalue(result)
            .map_err(RunnerError::InterpreterResultDeError)?;
        let outcome = RawAVMOutcome::from_interpreter_outcome(outcome)?;

        Ok(outcome)
    }

    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip_all)]
    pub async fn call_tracing(
        &mut self,
        air: impl Into<String>,
        prev_data: impl Into<Vec<u8>>,
        data: impl Into<Vec<u8>>,
        init_peer_id: impl Into<String>,
        timestamp: u64,
        ttl: u32,
        current_peer_id: impl Into<String>,
        call_results: CallResults,
        tracing_params: String,
        tracing_output_mode: u8,
        key_format: u8,
        secret_key_bytes: Vec<u8>,
        particle_id: String,
    ) -> RunnerResult<RawAVMOutcome> {
        let mut args = prepare_args(
            air,
            prev_data,
            data,
            current_peer_id.into(),
            init_peer_id.into(),
            timestamp,
            ttl,
            self.aquavm_runtime_limits,
            call_results,
            key_format,
            secret_key_bytes,
            particle_id,
        );
        args.push(IValue::String(tracing_params));
        args.push(IValue::U8(tracing_output_mode));

        let result = measure!(
            self.marine
                .call_with_ivalues_async(
                    &self.wasm_filename,
                    "invoke_tracing",
                    &args,
                    <_>::default(),
                )
                .await?,
            tracing::Level::INFO,
            "marine.call_with_ivalues",
            method = "invoke_tracing",
        );

        let result = try_as_one_value_vec(result)?;
        let outcome = InterpreterOutcome::from_ivalue(result)
            .map_err(RunnerError::InterpreterResultDeError)?;
        let outcome = RawAVMOutcome::from_interpreter_outcome(outcome)?;

        Ok(outcome)
    }

    pub async fn to_human_readable_data<'this>(
        &'this mut self,
        data: Vec<u8>,
    ) -> RunnerResult<String> {
        let args = vec![IValue::ByteArray(data)];

        let result = self
            .marine
            .call_with_ivalues_async(
                &self.wasm_filename,
                "to_human_readable_data",
                &args,
                <_>::default(),
            )
            .await?;
        let result = try_as_one_value_vec(result)?;
        let outcome = try_as_string(result, "result").map_err(RunnerError::Aux)?;
        Ok(outcome)
    }

    pub fn memory_stats(&self) -> AVMMemoryStats {
        let stats = self.marine.module_memory_stats();

        // only the interpreters must be loaded in Marine
        debug_assert!(stats.modules.len() == 1);

        AVMMemoryStats {
            memory_size: stats.modules[0].memory_size,
            total_memory_limit: self.total_memory_limit,
            allocation_rejects: stats.allocation_stats.map(|stats| stats.allocation_rejects),
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(skip(air, prev_data, data, call_results, secret_key_bytes))]
fn prepare_args(
    air: impl Into<String>,
    prev_data: impl Into<Vec<u8>>,
    data: impl Into<Vec<u8>>,
    current_peer_id: String,
    init_peer_id: String,
    timestamp: u64,
    ttl: u32,
    aquavm_runtime_limits: AquaVMRuntimeLimits,
    call_results: CallResults,
    key_format: u8,
    secret_key_bytes: Vec<u8>,
    particle_id: String,
) -> Vec<IValue> {
    let AquaVMRuntimeLimits {
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
        hard_limit_enabled,
    } = aquavm_runtime_limits;

    let run_parameters = air_interpreter_interface::RunParameters::new(
        init_peer_id,
        current_peer_id,
        timestamp,
        ttl,
        key_format,
        secret_key_bytes,
        particle_id,
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
        hard_limit_enabled,
    )
    .into_ivalue();

    let call_results = avm_interface::into_raw_result(call_results);
    let call_results = measure!(
        CallResultsRepr
            .serialize(&call_results)
            .expect("the default serializer shouldn't fail"),
        tracing::Level::INFO,
        "CallResultsRepr.serialize"
    );

    vec![
        IValue::String(air.into()),
        IValue::ByteArray(prev_data.into()),
        IValue::ByteArray(data.into()),
        run_parameters,
        IValue::ByteArray(call_results.to_vec()),
    ]
}

/// Splits given path into its directory and file name
///
/// # Example
/// For path `/path/to/air_interpreter_server.wasm` result will be `Ok(PathBuf(/path/to), "air_interpreter_server.wasm")`
fn split_dirname(path: PathBuf) -> RunnerResult<(PathBuf, String)> {
    use RunnerError::InvalidAIRPath;

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

fn make_marine_config<WB: WasmBackend>(
    air_wasm_dir: PathBuf,
    air_wasm_file: &str,
    total_memory_limit: Option<u64>,
    logging_mask: i32,
) -> MarineConfig<WB> {
    let air_module_config = marine::generic::MarineModuleConfig::<WB> {
        logger_enabled: true,
        host_imports: <_>::default(),
        wasi: None,
        logging_mask,
    };

    MarineConfig {
        modules_dir: Some(air_wasm_dir),
        total_memory_limit,
        modules_config: vec![ModuleDescriptor {
            load_from: None,
            file_name: String::from(air_wasm_file),
            import_name: String::from(air_wasm_file),
            config: air_module_config,
        }],
        default_modules_config: None,
    }
}

fn try_as_one_value_vec(mut ivalues: Vec<IValue>) -> RunnerResult<IValue> {
    use RunnerError::IncorrectInterpreterResult;

    if ivalues.len() != 1 {
        return Err(IncorrectInterpreterResult(ivalues));
    }

    Ok(ivalues.remove(0))
}

impl AquaVMRuntimeLimits {
    pub fn new(
        air_size_limit: u64,
        particle_size_limit: u64,
        call_result_size_limit: u64,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        }
    }
}

impl AVMRuntimeLimits {
    pub fn new(
        air_size_limit: Option<u64>,
        particle_size_limit: Option<u64>,
        call_result_size_limit: Option<u64>,
        hard_limit_enabled: bool,
    ) -> Self {
        Self {
            air_size_limit,
            particle_size_limit,
            call_result_size_limit,
            hard_limit_enabled,
        }
    }
}

impl From<AVMRuntimeLimits> for AquaVMRuntimeLimits {
    fn from(value: AVMRuntimeLimits) -> Self {
        use air_interpreter_interface::MAX_AIR_SIZE;
        use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
        use air_interpreter_interface::MAX_PARTICLE_SIZE;

        AquaVMRuntimeLimits::new(
            value.air_size_limit.unwrap_or(MAX_AIR_SIZE),
            value.particle_size_limit.unwrap_or(MAX_PARTICLE_SIZE),
            value.call_result_size_limit.unwrap_or(MAX_CALL_RESULT_SIZE),
            value.hard_limit_enabled,
        )
    }
}
