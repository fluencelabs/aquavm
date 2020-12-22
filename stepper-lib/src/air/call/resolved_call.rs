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
use crate::air::ExecutionCtx;
use crate::build_targets::CALL_SERVICE_SUCCESS;
use crate::call_evidence::{CallEvidenceCtx, CallResult, EvidenceState};
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::{CallOutput, InstructionValue};
use polyplets::ResolvedTriplet;

use std::rc::Rc;

/// Represents Call instruction with resolved internal parts.
#[derive(Clone)]
pub(super) struct ResolvedCall<'i> {
    triplet: Rc<ResolvedTriplet>,
    function_arg_paths: Rc<Vec<InstructionValue<'i>>>,
    output: CallOutput<'i>,
}

struct ResolvedArguments {
    call_arguments: String,
    tetraplets: Vec<Vec<SecurityTetraplet>>,
}

impl<'i> ResolvedCall<'i> {
    /// Build `ResolvedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`.
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> Result<Self> {
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
    pub(super) fn execute(self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        use AquamarineError::CallServiceResultDeserializationError as DeError;
        use CallResult::*;
        use EvidenceState::Call;

        let should_execute = self.prepare_evidence_state(exec_ctx, call_ctx)?;
        if !should_execute {
            return Ok(());
        }

        // call can be executed only on peers with such peer_id
        if self.triplet.peer_pk != exec_ctx.current_peer_id {
            set_remote_call_result(self.triplet.peer_pk.clone(), exec_ctx, call_ctx);

            return Ok(());
        }

        let ResolvedArguments {
            call_arguments,
            tetraplets,
        } = self.resolve_args(exec_ctx)?;

        let service_result = unsafe {
            crate::call_service(
                // copying here is necessary because of current limitations of rust-sdk
                self.triplet.service_id.clone(),
                self.triplet.function_name.clone(),
                call_arguments,
                tetraplets,
            )
        };

        // check that service call succeeded
        if service_result.ret_code != CALL_SERVICE_SUCCESS {
            call_ctx
                .new_path
                .push_back(Call(CallServiceFailed(service_result.result.clone())));
            return Err(AquamarineError::LocalServiceError(service_result.result));
        }

        let result: JValue = serde_json::from_str(&service_result.result).map_err(|e| DeError(service_result, e))?;
        let result = Rc::new(result);

        set_local_call_result(result.clone(), self.triplet.clone(), &self.output, exec_ctx)?;
        let new_evidence_state = Call(Executed(result));

        log::trace!(
            target: EVIDENCE_CHANGING,
            "  adding new call evidence state {:?}",
            new_evidence_state
        );

        call_ctx.new_path.push_back(new_evidence_state);

        Ok(())
    }

    /// Determine whether this call should be really called and adjust prev call evidence path accordingly.
    fn prepare_evidence_state(&self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<bool> {
        if call_ctx.current_subtree_size == 0 {
            log::trace!(target: EVIDENCE_CHANGING, "  previous call evidence state wasn't found");
            return Ok(true);
        }

        call_ctx.current_subtree_size -= 1;
        // unwrap is safe here, because current_subtree_size depends on current_path len,
        // and it's been checked previously
        let prev_state = call_ctx.current_path.pop_front().unwrap();

        log::trace!(
            target: EVIDENCE_CHANGING,
            "  previous call evidence state found {:?}",
            prev_state
        );

        handle_prev_state(&self.triplet, &self.output, prev_state, exec_ctx, call_ctx)
    }

    /// Prepare arguments of this call instruction by resolving and preparing their security tetraplets.
    fn resolve_args(&self, exec_ctx: &ExecutionCtx<'i>) -> Result<ResolvedArguments> {
        use crate::air::resolve::resolve_to_args;

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
