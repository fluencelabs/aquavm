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
use super::utils::{set_local_call_result, set_remote_call_result};
use super::Call;
use crate::air::ExecutionCtx;
use crate::build_targets::{CallServiceResult, CALL_SERVICE_SUCCESS};
use crate::call_evidence::{CallEvidenceCtx, CallResult, EvidenceState};
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AquamarineError;
use crate::ExecutedCallResult;
use crate::JValue;
use crate::Result;
use crate::SecurityTetraplet;

use air_parser::ast::{CallOutput, InstructionValue};

use std::rc::Rc;

/// Represents Call instruction with resolved internal parts.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(super) struct ResolvedCall<'i> {
    peer_pk: String,
    service_id: String,
    function_name: String,
    function_arg_paths: Vec<InstructionValue<'i>>,
    output: CallOutput<'i>,
}

impl<'i> ResolvedCall<'i> {
    /// Builds `ResolvedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`
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
        use CallResult::*;
        use EvidenceState::Call;

        let should_execute = self.prepare_evidence_state(exec_ctx, call_ctx)?;
        if !should_execute {
            return Ok(());
        }

        if self.peer_pk != exec_ctx.current_peer_id {
            set_remote_call_result(self.peer_pk, exec_ctx, call_ctx);

            return Ok(());
        }

        let (function_args, tetraplets) = self.prepare_args(exec_ctx)?;
        let result = unsafe {
            crate::call_service(
                self.service_id.clone(),
                self.function_name.clone(),
                function_args,
                tetraplets,
            )
        };

        if result.ret_code != CALL_SERVICE_SUCCESS {
            call_ctx
                .new_path
                .push_back(Call(CallServiceFailed(result.result.clone())));
            return Err(AquamarineError::LocalServiceError(result.result));
        }

        let result = self.prepare_result(result, exec_ctx)?;
        set_local_call_result(&self.output, exec_ctx, result.clone())?;
        let new_evidence_state = Call(Executed(result));

        log::trace!(
            target: EVIDENCE_CHANGING,
            "  adding new call evidence state {:?}",
            new_evidence_state
        );

        call_ctx.new_path.push_back(new_evidence_state);

        Ok(())
    }

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

        self.handle_prev_state(prev_state, exec_ctx, call_ctx)
    }

    fn prepare_args(&self, exec_ctx: &ExecutionCtx<'i>) -> Result<(String, Vec<Vec<SecurityTetraplet>>)> {
        use crate::air::resolve::resolve_to_args;

        let function_args = self.function_arg_paths.iter();
        let mut args = Vec::new();
        let mut tetraplets = Vec::new();
        for instruction_value in function_args {
            let (arg, tetraplet) = resolve_to_args(instruction_value, exec_ctx)?;
            args.push(arg);
            tetraplets.push(tetraplet);
        }

        let jvalue = JValue::Array(args);
        let function_args = jvalue.to_string();

        Ok((function_args, tetraplets))
    }

    fn prepare_result(&self, result: CallServiceResult, ctx: &mut ExecutionCtx<'i>) -> Result<Rc<ExecutedCallResult>> {
        use AquamarineError::CallServiceResultDeserializationError as DeError;

        let result: JValue = serde_json::from_str(&result.result).map_err(|e| DeError(result, e))?;
        let function_arguments =
            serde_json::to_string(&self.function_arg_paths).expect("default serializer wouldn't fail");

        let tetraplet = SecurityTetraplet {
            pub_key: ctx.current_peer_id.clone(),
            service_id: self.service_id.clone(),
            function_name: self.function_name.clone(),
            function_arguments,
        };

        let result = ExecutedCallResult { result, tetraplet };

        Ok(Rc::new(result))
    }

    fn handle_prev_state(
        &self,
        prev_state: EvidenceState,
        exec_ctx: &mut ExecutionCtx<'i>,
        call_ctx: &mut CallEvidenceCtx,
    ) -> Result<bool> {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

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
                set_local_call_result(&self.output, exec_ctx, result.clone())?;
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
