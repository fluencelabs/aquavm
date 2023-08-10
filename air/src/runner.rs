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

use crate::execution_step::ExecutableInstruction;
use crate::farewell_step as farewell;
use crate::preparation_step::parse_data;
use crate::preparation_step::prepare;
use crate::preparation_step::ParsedDataPair;
use crate::preparation_step::PreparationDescriptor;
use crate::signing_step::sign_produced_cids;
use crate::verification_step::verify;

use air_interpreter_interface::InterpreterOutcome;
use air_interpreter_interface::RunParameters;
use air_log_targets::RUN_PARAMS;
use air_utils::measure;

#[tracing::instrument(skip_all)]
pub fn execute_air(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: RunParameters,
    call_results: Vec<u8>,
) -> InterpreterOutcome {
    use std::convert::identity;

    log::trace!(
        target: RUN_PARAMS,
        "air interpreter version is {}, run parameters:\
            init peer id {}\
            current peer id {}",
        env!("CARGO_PKG_VERSION"),
        params.init_peer_id,
        params.current_peer_id,
    );

    execute_air_impl(air, prev_data, data, params, call_results).unwrap_or_else(identity)
}

#[allow(clippy::result_large_err)]
fn execute_air_impl(
    air: String,
    raw_prev_data: Vec<u8>,
    raw_current_data: Vec<u8>,
    params: RunParameters,
    call_results: Vec<u8>,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    let ParsedDataPair {
        prev_data,
        current_data,
    } = match parse_data(&raw_prev_data, &raw_current_data) {
        Ok(parsed_datas) => parsed_datas,
        // return the prev data in case of errors
        Err(error) => return Err(farewell::from_uncatchable_error(raw_prev_data, error)),
    };

    let particle_id = params.particle_id.clone();
    let signature_store = match verify(&prev_data, &current_data, &particle_id) {
        Ok(signature_store) => signature_store,
        // return the prev data in case of errors
        Err(error) => return Err(farewell::from_uncatchable_error(raw_prev_data, error)),
    };

    let PreparationDescriptor {
        mut exec_ctx,
        mut trace_handler,
        air,
        keypair,
    } = match prepare(
        prev_data,
        current_data,
        air.as_str(),
        &call_results,
        params,
        signature_store,
    ) {
        Ok(descriptor) => descriptor,
        // return the prev data in case of errors
        Err(error) => return Err(farewell::from_uncatchable_error(raw_prev_data, error)),
    };

    // match here is used instead of map_err, because the compiler can't determine that
    // they are exclusive and would treat exec_ctx and trace_handler as moved
    let exec_result = measure!(
        air.execute(&mut exec_ctx, &mut trace_handler),
        tracing::Level::INFO,
        "execute",
    );

    match sign_produced_cids(
        &mut exec_ctx.peer_cid_tracker,
        &mut exec_ctx.signature_store,
        &particle_id,
        &keypair,
    ) {
        Ok(()) => {}
        Err(error) => return Err(farewell::from_uncatchable_error(raw_prev_data, error)),
    }

    measure!(
        match exec_result {
            Ok(_) => farewell::from_success_result(exec_ctx, trace_handler, &keypair),
            // return new collected trace in case of errors
            Err(error) if error.is_catchable() => {
                Err(farewell::from_execution_error(exec_ctx, trace_handler, error, &keypair))
            }
            // return the prev data in case of any trace errors
            Err(error) => Err(farewell::from_uncatchable_error(raw_prev_data, error)),
        },
        tracing::Level::INFO,
        "farewell",
    )
}
