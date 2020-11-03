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

use super::triplet::{ResolvedTriplet, Triplet};
use super::utils::{resolve_jvalue, set_local_call_result, set_remote_call_result};
use super::Call;

use crate::air::ExecutionCtx;
use crate::build_targets::CALL_SERVICE_SUCCESS;
use crate::call_evidence::{CallEvidenceCtx, CallResult, EvidenceState};
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use air_parser::ast::{CallOutput, Value};

use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ParsedCall<'i> {
    peer_pk: String,
    service_id: String,
    function_name: String,
    function_arg_paths: Vec<Value<'i>>,
    output: CallOutput<'i>,
}

impl<'i> ParsedCall<'i> {
    /// Builds `ParsedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> Result<Self> {
        let triplet = Triplet::try_from(&raw_call.peer_part, &raw_call.function_part)?;
        #[rustfmt::skip]
        let ResolvedTriplet { peer_pk, service_id, function_name } = triplet.resolve(exec_ctx)?;

        Ok(Self {
            peer_pk,
            service_id,
            function_name,
            function_arg_paths: raw_call.args.clone(),
            output: raw_call.output.clone(),
        })
    }

    pub(super) fn execute(self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        let should_execute = self.prepare_evidence_state(exec_ctx, call_ctx)?;
        if !should_execute {
            return Ok(());
        }

        if self.peer_pk != exec_ctx.current_peer_id {
            set_remote_call_result(self.peer_pk, exec_ctx, call_ctx);

            return Ok(());
        }

        let function_args = self.function_arg_paths.iter();
        let function_args: Result<Vec<_>> = function_args.map(|v| resolve_jvalue(v, exec_ctx)).collect();
        let function_args = JValue::Array(function_args?).to_string();

        let result = unsafe { crate::call_service(self.service_id, self.function_name, function_args) };

        if result.ret_code != CALL_SERVICE_SUCCESS {
            call_ctx
                .new_path
                .push_back(EvidenceState::Call(CallResult::CallServiceFailed(
                    result.result.clone(),
                )));
            return Err(AquamarineError::LocalServiceError(result.result));
        }

        let result: JValue = serde_json::from_str(&result.result)
            .map_err(|e| AquamarineError::CallServiceResultDeserializationError(result, e))?;
        let result = Rc::new(result);
        set_local_call_result(self.output, exec_ctx, result.clone())?;

        let new_evidence_state = EvidenceState::Call(CallResult::Executed(result));
        log::info!(
            target: EVIDENCE_CHANGING,
            "  adding new call evidence state {:?}",
            new_evidence_state
        );
        call_ctx.new_path.push_back(new_evidence_state);

        Ok(())
    }

    pub(super) fn prepare_evidence_state(
        &self,
        exec_ctx: &mut ExecutionCtx<'i>,
        call_ctx: &mut CallEvidenceCtx,
    ) -> Result<bool> {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        if call_ctx.current_subtree_elements_count == 0 {
            log::info!(target: EVIDENCE_CHANGING, "  previous call evidence state wasn't found");
            return Ok(true);
        }

        call_ctx.current_subtree_elements_count -= 1;
        // unwrap is safe here, because current_subtree_elements_count depends on current_path len,
        // and it's been checked previously
        let prev_state = call_ctx.current_path.pop_front().unwrap();

        log::info!(
            target: EVIDENCE_CHANGING,
            "  previous call evidence state found {:?}",
            prev_state
        );

        match &prev_state {
            // this call was failed on one of the previous executions,
            // here it's needed to bubble this special error up
            Call(CallServiceFailed(err_msg)) => {
                let err_msg = err_msg.clone();
                call_ctx.new_path.push_back(prev_state);
                exec_ctx.subtree_complete = false;
                Err(AquamarineError::LocalServiceError(err_msg))
            }
            Call(RequestSent(..)) => {
                // check whether current node can execute this call
                let is_current_peer = self.peer_pk == exec_ctx.current_peer_id;
                if is_current_peer {
                    Ok(true)
                } else {
                    exec_ctx.subtree_complete = false;
                    call_ctx.new_path.push_back(prev_state);
                    Ok(false)
                }
            }
            // this instruction's been already executed
            Call(Executed(result)) => {
                set_local_call_result(self.output.clone(), exec_ctx, result.clone())?;
                call_ctx.new_path.push_back(prev_state);
                Ok(false)
            }
            // state has inconsistent order - return a error, call shouldn't be executed
            par_state @ Par(..) => Err(AquamarineError::InvalidEvidenceState(
                par_state.clone(),
                String::from("call"),
            )),
        }
    }
}
