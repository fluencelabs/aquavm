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

mod outcome;

use crate::execution_step::ExecutableInstruction;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionError;
use crate::execution_step::{Catchable, TraceHandler};
use air_log_targets::RUN_PARAMS;
use crate::preparation_step::prepare;
use crate::preparation_step::PreparationDescriptor;

use air_interpreter_interface::InterpreterOutcome;
use air_interpreter_interface::RunParameters;
use std::rc::Rc;

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

fn execute_air_impl(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: RunParameters,
    call_results: Vec<u8>,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    let PreparationDescriptor {
        mut exec_ctx,
        mut trace_handler,
        air,
    } = match prepare(&prev_data, &data, air.as_str(), &call_results, params) {
        Ok(desc) => desc,
        // return the initial data in case of errors
        Err(error) => return Err(outcome::from_preparation_error(prev_data, error)),
    };

    // match here is used instead of map_err, because the compiler can't determine that
    // they are exclusive and would treat exec_ctx and trace_handler as moved
    match air.execute(&mut exec_ctx, &mut trace_handler) {
        Ok(_) => try_make_outcome(exec_ctx, trace_handler),
        // return the old data in case of any trace errors
        Err(e) if !e.is_catchable() => Err(outcome::from_trace_error(prev_data, e)),
        // return new collected trace in case of errors
        Err(e) => Err(outcome::from_execution_error(exec_ctx, trace_handler, e)),
    }
}

fn try_make_outcome(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    if exec_ctx.call_results.is_empty() {
        let outcome = outcome::from_success_result(exec_ctx, trace_handler);
        return Ok(outcome);
    }

    let exec_error = Rc::new(ExecutionError::CallResultsNotEmpty(exec_ctx.call_results.clone()));
    let outcome = outcome::from_execution_error(exec_ctx, trace_handler, exec_error);
    Err(outcome)
}
