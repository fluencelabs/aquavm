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

#![allow(unused_unsafe)] // for wasm_bindgen target where calling FFI is safe

use super::triplet::Triplet;
use super::utils::*;
use super::Call;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use crate::build_targets::CALL_SERVICE_SUCCESS;
use crate::contexts::execution_trace::*;
use crate::log_targets::EXECUTED_STATE_CHANGING;
use crate::JValue;
use crate::ResolvedTriplet;
use crate::SecurityTetraplet;

use air_parser::ast::{CallInstructionValue, CallOutput};

use std::rc::Rc;

/// Represents Call instruction with resolved internal parts.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) struct ResolvedCall<'i> {
    triplet: Rc<ResolvedTriplet>,
    function_arg_paths: Rc<Vec<CallInstructionValue<'i>>>,
    output: CallOutput<'i>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ResolvedArguments {
    call_arguments: String,
    tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl<'i> ResolvedCall<'i> {
    /// Build `ResolvedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`.
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<Self> {
        let triplet = Triplet::try_from(&raw_call.peer_part, &raw_call.function_part)?;
        let triplet = triplet.resolve(exec_ctx)?;
        let triplet = Rc::new(triplet);

        Ok(Self {
            triplet,
            function_arg_paths: raw_call.args.clone(),
            output: raw_call.output.clone(),
        })
    }

    /// Executes resolved instruction, updates contexts based on a execution result.
    pub(super) fn execute(
        self,
        exec_ctx: &mut ExecutionCtx<'i>,
        trace_ctx: &mut ExecutionTraceCtx,
    ) -> ExecutionResult<()> {
        use CallResult::*;
        use ExecutedState::Call;
        use ExecutionError::CallServiceResultDeError as DeError;

        let should_execute = self.prepare_executed_state(exec_ctx, trace_ctx)?;
        if !should_execute {
            return Ok(());
        }

        // call can be executed only on peers with such peer_id
        if self.triplet.peer_pk != exec_ctx.current_peer_id {
            set_remote_call_result(self.triplet.peer_pk.clone(), exec_ctx, trace_ctx);

            return Ok(());
        }

        let ResolvedArguments {
            call_arguments,
            tetraplets,
        } = self.resolve_args(exec_ctx)?;

        let tetraplets = serde_json::to_string(&tetraplets).expect("default serializer shouldn't fail");

        let service_result = unsafe {
            crate::build_targets::call_service(
                // copying here is necessary because of current limitations of rust-sdk
                self.triplet.service_id.clone(),
                self.triplet.function_name.clone(),
                call_arguments,
                tetraplets,
            )
        };

        // check that service call succeeded
        if service_result.ret_code != CALL_SERVICE_SUCCESS {
            trace_ctx
                .new_trace
                .push_back(Call(CallServiceFailed(service_result.result.clone())));
            return Err(ExecutionError::LocalServiceError(service_result.result));
        }

        let result: JValue = serde_json::from_str(&service_result.result).map_err(|e| DeError(service_result, e))?;
        let result = Rc::new(result);

        set_local_call_result(result.clone(), self.triplet.clone(), &self.output, exec_ctx)?;
        let new_executed_state = Call(Executed(result));

        log::trace!(
            target: EXECUTED_STATE_CHANGING,
            "  adding new call executed state {:?}",
            new_executed_state
        );

        trace_ctx.new_trace.push_back(new_executed_state);

        Ok(())
    }

    /// Determine whether this call should be really called and adjust prev executed trace accordingly.
    fn prepare_executed_state(
        &self,
        exec_ctx: &mut ExecutionCtx<'i>,
        trace_ctx: &mut ExecutionTraceCtx,
    ) -> ExecutionResult<bool> {
        if trace_ctx.current_subtree_size == 0 {
            log::trace!(
                target: EXECUTED_STATE_CHANGING,
                "  previous executed trace state wasn't found"
            );
            return Ok(true);
        }

        trace_ctx.current_subtree_size -= 1;
        // unwrap is safe here, because current_subtree_size depends on current_path len,
        // and it's been checked previously
        let prev_state = trace_ctx.current_trace.pop_front().unwrap();

        log::trace!(
            target: EXECUTED_STATE_CHANGING,
            "  previous executed trace found {:?}",
            prev_state
        );

        handle_prev_state(&self.triplet, &self.output, prev_state, exec_ctx, trace_ctx)
    }

    /// Prepare arguments of this call instruction by resolving and preparing their security tetraplets.
    fn resolve_args(&self, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedArguments> {
        use crate::execution::utils::resolve_to_args;

        let function_args = self.function_arg_paths.iter();
        let mut call_arguments = Vec::new();
        let mut tetraplets = Vec::new();
        for instruction_value in function_args {
            let (arg, tetraplet) = resolve_to_args(instruction_value, exec_ctx)?;
            call_arguments.push(arg);
            tetraplets.push(tetraplet);
        }

        let call_arguments = JValue::Array(call_arguments);
        let call_arguments = call_arguments.to_string();

        let resolved_arguments = ResolvedArguments {
            call_arguments,
            tetraplets,
        };

        Ok(resolved_arguments)
    }
}
