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

pub(crate) mod call_result_setter;
mod prev_result_handler;
mod resolved_call;
pub(crate) mod triplet;

use resolved_call::ResolvedCall;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::Joinable;
use crate::execution_step::RcSecurityTetraplet;
use crate::joinable;
use crate::log_instruction;

use air_parser::ast::Call;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Call<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        println!("> {}", self);
        exec_ctx.tracker.meet_call();

        let resolved_call = joinable!(ResolvedCall::new(self, exec_ctx), exec_ctx)
            .map_err(|e| set_last_error(self, exec_ctx, e, None))?;

        let tetraplet = resolved_call.as_tetraplet();
        joinable!(resolved_call.execute(self, exec_ctx, trace_ctx), exec_ctx)
            .map_err(|e| set_last_error(self, exec_ctx, e, Some(tetraplet)))
    }
}

fn set_last_error<'i>(
    call: &Call<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    execution_error: ExecutionError,
    tetraplet: Option<RcSecurityTetraplet>,
) -> ExecutionError {
    let catchable_error = match execution_error {
        ExecutionError::Catchable(catchable) => catchable,
        ExecutionError::Uncatchable(_) => return execution_error,
    };

    let current_peer_id = match &tetraplet {
        // use tetraplet if they set, because an error could be propagated from data
        // (from CallServiceFailed state) and exec_ctx.run_parameters.current_peer_id won't mean
        // a peer where the error was occurred
        Some(tetraplet) => tetraplet.peer_pk.clone(),
        None => exec_ctx.run_parameters.current_peer_id.to_string(),
    };

    log::warn!(
        "call failed with an error `{}`, peerId `{}`",
        catchable_error,
        current_peer_id
    );

    let _ = exec_ctx.last_error_descriptor.try_to_set_from_error(
        catchable_error.as_ref(),
        &call.to_string(),
        &current_peer_id,
        tetraplet,
    );
    ExecutionError::Catchable(catchable_error)
}
