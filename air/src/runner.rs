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
use air_interpreter_interface::SerializedCallResults;
use air_interpreter_interface::SoftLimitsTriggering;
use air_log_targets::RUN_PARAMS;
use air_utils::farewell_if_fail;
use air_utils::measure;

#[tracing::instrument(skip_all)]
pub fn execute_air(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: RunParameters,
    call_results: SerializedCallResults,
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
    call_results: SerializedCallResults,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    use crate::preparation_step::check_against_size_limits;

    let mut soft_limits_triggering = farewell_if_fail!(
        check_against_size_limits(&params, &air, &raw_current_data),
        raw_prev_data,
        SoftLimitsTriggering::default()
    );

    farewell_if_fail!(
        check_against_size_limits(&params, &air, &raw_current_data),
        raw_prev_data,
        soft_limits_triggering
    );

    let ParsedDataPair {
        prev_data,
        current_data,
    } = farewell_if_fail!(
        parse_data(&raw_prev_data, &raw_current_data),
        raw_prev_data,
        soft_limits_triggering
    );

    // TODO currently we use particle ID, but it should be changed to signature,
    // as partical ID can be equally replayed
    let salt = params.particle_id.clone();
    let signature_store = farewell_if_fail!(
        verify(&prev_data, &current_data, &salt),
        raw_prev_data,
        soft_limits_triggering
    );

    let PreparationDescriptor {
        mut exec_ctx,
        mut trace_handler,
        air,
        keypair,
    } = farewell_if_fail!(
        prepare(
            prev_data,
            current_data,
            &air,
            &call_results,
            params,
            signature_store,
            &mut soft_limits_triggering
        ),
        raw_prev_data,
        soft_limits_triggering
    );

    // match here is used instead of map_err, because the compiler can't determine that
    // they are exclusive and would treat exec_ctx and trace_handler as moved
    let exec_result = measure!(
        air.execute(&mut exec_ctx, &mut trace_handler),
        tracing::Level::INFO,
        "execute",
    );

    farewell_if_fail!(
        sign_produced_cids(
            &mut exec_ctx.peer_cid_tracker,
            &mut exec_ctx.signature_store,
            &salt,
            &keypair,
        ),
        raw_prev_data,
        soft_limits_triggering
    );

    measure!(
        match exec_result {
            Ok(_) => farewell::from_success_result(exec_ctx, trace_handler, &keypair, soft_limits_triggering),
            // return new collected trace in case of errors
            Err(error) if error.is_catchable() => {
                Err(farewell::from_execution_error(
                    exec_ctx,
                    trace_handler,
                    error,
                    &keypair,
                    soft_limits_triggering,
                ))
            }
            // return the prev data in case of any trace errors
            Err(error) => Err(farewell::from_uncatchable_error(
                raw_prev_data,
                error,
                soft_limits_triggering
            )),
        },
        tracing::Level::INFO,
        "farewell",
    )
}
