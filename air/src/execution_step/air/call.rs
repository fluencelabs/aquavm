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
mod resolved_call;
mod triplet;
mod utils;

use resolved_call::ResolvedCall;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::LastErrorDescriptor;
use super::ResolvedCallResult;
use super::TraceHandler;
use crate::execution_step::joinable::Joinable;
use crate::joinable_call;
use crate::log_instruction;
use crate::SecurityTetraplet;

use air_parser::ast::Call;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Call<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);

        let resolved_call = joinable_call!(ResolvedCall::new(self, exec_ctx), exec_ctx).map_err(|e| {
            set_last_error(self, exec_ctx, e.clone(), None);
            e
        })?;

        let triplet = resolved_call.as_triplet();
        joinable_call!(resolved_call.execute(exec_ctx, trace_ctx), exec_ctx).map_err(|e| {
            let tetraplet = SecurityTetraplet::from_triplet(triplet);
            set_last_error(self, exec_ctx, e.clone(), Some(tetraplet));

            e
        })
    }
}

fn set_last_error<'i>(
    call: &Call<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    e: Rc<ExecutionError>,
    tetraplet: Option<SecurityTetraplet>,
) {
    let current_peer_id = match &tetraplet {
        // use tetraplet if they set, because an error could be propagated from data
        // (from CallServiceFailed state) and exec_ctx.current_peer_id won't mean
        // a peer where the error was occurred
        Some(tetraplet) => tetraplet.triplet.peer_pk.clone(),
        None => exec_ctx.current_peer_id.to_string(),
    };

    log::warn!("call failed with an error `{}`, peerId `{}`", e, current_peer_id);

    let instruction = call.to_string();
    let last_error = LastErrorDescriptor::new(e, instruction, current_peer_id, tetraplet);
    exec_ctx.last_error = Some(last_error);
    exec_ctx.last_error_could_be_set = false;
}
