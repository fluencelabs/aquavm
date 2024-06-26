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

use super::PreparationError;
use crate::execution_step::execution_context::ExecCtxIngredients;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::TraceHandler;

use air_interpreter_data::DataDeserializationError;
use air_interpreter_data::InterpreterData;
use air_interpreter_data::InterpreterDataEnvelope;
use air_interpreter_data::Versions;
use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use air_interpreter_interface::SerializedCallResults;
use air_interpreter_interface::SoftLimitsTriggering;
use air_interpreter_sede::FromSerialized;
use air_interpreter_signatures::KeyError;
use air_interpreter_signatures::KeyPair;
use air_interpreter_signatures::SignatureStore;
use air_parser::ast::Instruction;
use air_utils::measure;
use fluence_keypair::KeyFormat;

pub(crate) type PreparationResult<T> = Result<T, PreparationError>;

/// Represents result of the preparation_step step.
pub(crate) struct PreparationDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) trace_handler: TraceHandler,
    pub(crate) air: Instruction<'i>,
    pub(crate) keypair: KeyPair,
}

pub(crate) struct ParsedDataPair {
    pub(crate) prev_data: InterpreterData,
    pub(crate) current_data: InterpreterData,
}

/// Parse data and check its version.
#[tracing::instrument(skip_all)]
pub(crate) fn parse_data(prev_data: &[u8], current_data: &[u8]) -> PreparationResult<ParsedDataPair> {
    let prev_envelope = try_to_envelope(prev_data)?;
    let current_envelope = try_to_envelope(current_data)?;

    check_version_compatibility(&current_envelope.versions)?;

    let prev_data = try_to_data(&prev_envelope.inner_data)?;
    let current_data = try_to_data(&current_envelope.inner_data)?;

    Ok(ParsedDataPair {
        prev_data,
        current_data,
    })
}

/// Parse and prepare supplied data and AIR script.
#[tracing::instrument(skip_all)]
pub(crate) fn prepare<'i>(
    prev_data: InterpreterData,
    current_data: InterpreterData,
    raw_air: &'i str,
    call_results: &SerializedCallResults,
    run_parameters: RunParameters,
    signature_store: SignatureStore,
    soft_limits_triggering: &mut SoftLimitsTriggering,
) -> PreparationResult<PreparationDescriptor<'static, 'i>> {
    let air: Instruction<'i> = air_parser::parse(raw_air).map_err(PreparationError::AIRParseError)?;

    let prev_ingredients = ExecCtxIngredients {
        last_call_request_id: prev_data.last_call_request_id,
        cid_info: prev_data.cid_info,
    };

    let current_ingredients = ExecCtxIngredients {
        last_call_request_id: current_data.last_call_request_id,
        cid_info: current_data.cid_info,
    };

    let exec_ctx = make_exec_ctx(
        prev_ingredients,
        current_ingredients,
        call_results,
        signature_store,
        &run_parameters,
        soft_limits_triggering,
    )?;
    let trace_handler = TraceHandler::from_trace(prev_data.trace, current_data.trace);

    let key_format = KeyFormat::try_from(run_parameters.key_format).map_err(KeyError::from)?;
    let keypair = KeyPair::from_secret_key(run_parameters.secret_key_bytes, key_format)?;

    let result = PreparationDescriptor {
        exec_ctx,
        trace_handler,
        air,
        keypair,
    };

    Ok(result)
}

pub(crate) fn try_to_envelope(raw_env_data: &[u8]) -> PreparationResult<InterpreterDataEnvelope<'_>> {
    // treat empty slice as an empty data,
    // it allows abstracting from an internal format for an empty data
    if raw_env_data.is_empty() {
        return Ok(InterpreterDataEnvelope::new(super::min_supported_version().clone()));
    }

    InterpreterDataEnvelope::try_from_slice(raw_env_data)
        .map_err(|de_error| to_envelope_de_error(raw_env_data.to_vec(), de_error))
}

pub(crate) fn try_to_data(raw_data: &[u8]) -> PreparationResult<InterpreterData> {
    InterpreterData::try_from_slice(raw_data).map_err(to_data_de_error)
}

fn to_envelope_de_error(env_raw_data: Vec<u8>, de_error: DataDeserializationError) -> PreparationError {
    match InterpreterDataEnvelope::try_get_versions(&env_raw_data) {
        Ok(versions) => PreparationError::env_de_failed_with_versions(de_error, versions),
        Err(_) => PreparationError::envelope_de_failed(de_error),
    }
}

fn to_data_de_error(de_error: DataDeserializationError) -> PreparationError {
    PreparationError::data_de_failed(de_error)
}

#[tracing::instrument(skip_all)]
fn make_exec_ctx(
    prev_ingredients: ExecCtxIngredients,
    current_ingredients: ExecCtxIngredients,
    call_results: &SerializedCallResults,
    signature_store: SignatureStore,
    run_parameters: &RunParameters,
    soft_limits_triggering: &mut SoftLimitsTriggering,
) -> PreparationResult<ExecutionCtx<'static>> {
    use crate::preparation_step::sizes_limits_check::handle_limit_exceeding;

    let call_results = measure!(
        CallResultsRepr
            .deserialize(call_results)
            .map_err(PreparationError::call_results_de_failed)?,
        tracing::Level::INFO,
        "CallResultsRepr.deserialize",
    );

    // This is a part of argument size limit check where we check the size of every call result.
    if call_results
        .values()
        .any(|call_result| call_result.result.len() as u64 > run_parameters.call_result_size_limit)
    {
        let error: PreparationError = PreparationError::call_result_size_limit(run_parameters.call_result_size_limit);
        handle_limit_exceeding(
            run_parameters,
            error,
            &mut soft_limits_triggering.call_result_size_limit_exceeded,
        )?;
    }

    let ctx = ExecutionCtx::new(
        prev_ingredients,
        current_ingredients,
        call_results,
        signature_store,
        run_parameters,
    );
    Ok(ctx)
}

pub(crate) fn check_version_compatibility(versions: &Versions) -> PreparationResult<()> {
    if &versions.interpreter_version < super::min_supported_version() {
        return Err(PreparationError::UnsupportedInterpreterVersion {
            actual_version: versions.interpreter_version.clone(),
            required_version: super::min_supported_version().clone(),
        });
    }

    Ok(())
}
