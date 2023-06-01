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

use super::PreparationError;
use crate::execution_step::execution_context::ExecCtxIngredients;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::TraceHandler;

use air_interpreter_data::verification;
use air_interpreter_data::InterpreterData;
use air_interpreter_interface::RunParameters;
use air_interpreter_signatures::FullSignatureStore;
use air_parser::ast::Instruction;

type PreparationResult<T> = Result<T, PreparationError>;

/// Represents result of the preparation_step step.
pub(crate) struct PreparationDescriptor<'ctx, 'i> {
    pub(crate) exec_ctx: ExecutionCtx<'ctx>,
    pub(crate) trace_handler: TraceHandler,
    pub(crate) air: Instruction<'i>,
}

/// Parse and prepare supplied data and AIR script.
#[tracing::instrument(skip_all)]
pub(crate) fn prepare<'i>(
    prev_data: &[u8],
    current_data: &[u8],
    raw_air: &'i str,
    call_results: &[u8],
    run_parameters: RunParameters,
) -> PreparationResult<PreparationDescriptor<'static, 'i>> {
    let prev_data = try_to_data(prev_data)?;
    let current_data = try_to_data(current_data)?;

    check_version_compatibility(&current_data)?;

    let air: Instruction<'i> = *air_parser::parse(raw_air).map_err(PreparationError::AIRParseError)?;

    current_data.cid_info.verify()?;

    // prev_data is always correct
    let prev_data_verifier = verification::Verifier::new(&prev_data).unwrap();
    let current_data_verifier = verification::Verifier::new(&current_data).expect("TODO");
    current_data_verifier.verify().expect("TODO");
    let signature_store = prev_data_verifier.merge(current_data_verifier).expect("TODO");

    let prev_ingredients = ExecCtxIngredients {
        global_streams: prev_data.global_streams,
        last_call_request_id: prev_data.last_call_request_id,
        restricted_streams: prev_data.restricted_streams,
        cid_info: prev_data.cid_info,
    };

    let current_ingredients = ExecCtxIngredients {
        global_streams: current_data.global_streams,
        last_call_request_id: current_data.last_call_request_id,
        restricted_streams: current_data.restricted_streams,
        cid_info: current_data.cid_info,
    };

    let exec_ctx = make_exec_ctx(
        prev_ingredients,
        current_ingredients,
        call_results,
        signature_store,
        run_parameters,
    )?;
    let trace_handler = TraceHandler::from_trace(prev_data.trace, current_data.trace);

    let result = PreparationDescriptor {
        exec_ctx,
        trace_handler,
        air,
    };

    Ok(result)
}

fn try_to_data(raw_data: &[u8]) -> PreparationResult<InterpreterData> {
    // treat empty slice as an empty data,
    // it allows abstracting from an internal format for an empty data
    if raw_data.is_empty() {
        return Ok(InterpreterData::new(super::min_supported_version().clone()));
    }

    InterpreterData::try_from_slice(raw_data).map_err(|de_error| to_date_de_error(raw_data.to_vec(), de_error))
}

fn to_date_de_error(raw_data: Vec<u8>, de_error: serde_json::Error) -> PreparationError {
    match InterpreterData::try_get_versions(&raw_data) {
        Ok(versions) => PreparationError::data_de_failed_with_versions(raw_data, de_error, versions),
        Err(_) => PreparationError::data_de_failed(raw_data, de_error),
    }
}

#[tracing::instrument(skip_all)]
fn make_exec_ctx(
    prev_ingredients: ExecCtxIngredients,
    current_ingredients: ExecCtxIngredients,
    call_results: &[u8],
    signature_store: FullSignatureStore,
    run_parameters: RunParameters,
) -> PreparationResult<ExecutionCtx<'static>> {
    let call_results = serde_json::from_slice(call_results)
        .map_err(|e| PreparationError::call_results_de_failed(call_results.to_vec(), e))?;

    let ctx = ExecutionCtx::new(
        prev_ingredients,
        current_ingredients,
        call_results,
        signature_store,
        run_parameters,
    );
    Ok(ctx)
}

fn check_version_compatibility(data: &InterpreterData) -> PreparationResult<()> {
    if &data.versions.interpreter_version < super::min_supported_version() {
        return Err(PreparationError::UnsupportedInterpreterVersion {
            actual_version: data.versions.interpreter_version.clone(),
            required_version: super::min_supported_version().clone(),
        });
    }

    Ok(())
}
