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

use super::call_result_setter::*;
use super::prev_result_handler::*;
use super::triplet::Triplet;
use super::*;
use crate::execution_step::trace_handler::MergerCallResult;
use crate::execution_step::trace_handler::TraceHandler;
use crate::execution_step::RSecurityTetraplet;
use crate::execution_step::SecurityTetraplets;
use crate::JValue;
use crate::SecurityTetraplet;

use air_interpreter_data::CallResult;
use air_interpreter_interface::CallRequestParams;
use air_parser::ast::{AstVariable, CallInstrArgValue, CallOutputValue};
use polyplets::ResolvedTriplet;

use std::cell::RefCell;
use std::rc::Rc;

/// Represents Call instruction with resolved internal parts.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct ResolvedCall<'i> {
    tetraplet: RSecurityTetraplet,
    function_arg_paths: Rc<Vec<CallInstrArgValue<'i>>>,
    output: CallOutputValue<'i>,
}

#[derive(Debug, Clone, PartialEq)]
struct ResolvedArguments {
    call_arguments: String,
    tetraplets: Vec<SecurityTetraplets>,
}

impl<'i> ResolvedCall<'i> {
    /// Build `ResolvedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`.
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<Self> {
        let triplet = Triplet::try_from(&raw_call.peer_part, &raw_call.function_part)?;
        let triplet = triplet.resolve(exec_ctx)?;
        let tetraplet = SecurityTetraplet::from_triplet(triplet);
        let tetraplet = Rc::new(RefCell::new(tetraplet));

        check_output_name(&raw_call.output, exec_ctx)?;

        Ok(Self {
            tetraplet,
            function_arg_paths: raw_call.args.clone(),
            output: raw_call.output.clone(),
        })
    }

    /// Executes resolved instruction, updates contexts based on a execution_step result.
    pub(super) fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let should_execute = self.prepare_current_executed_state(exec_ctx, trace_ctx)?;
        if !should_execute {
            return Ok(());
        }

        // call can be executed only on peers with such peer_id
        let tetraplet = &self.tetraplet.borrow().triplet;
        if tetraplet.peer_pk.as_str() != exec_ctx.current_peer_id.as_str() {
            set_remote_call_result(tetraplet.peer_pk.clone(), exec_ctx, trace_ctx);
            return Ok(());
        }

        let request_params = self.prepare_request_params(exec_ctx, tetraplet)?;
        exec_ctx
            .call_requests
            .insert(trace_ctx.trace_pos() as u32, request_params);

        exec_ctx.subtree_complete = false;
        trace_ctx.meet_call_end(CallResult::RequestSentBy(exec_ctx.current_peer_id.clone()));

        Ok(())
    }

    pub(super) fn as_tetraplet(&self) -> RSecurityTetraplet {
        self.tetraplet.clone()
    }

    fn prepare_request_params(
        &self,
        exec_ctx: &ExecutionCtx<'i>,
        triplet: &ResolvedTriplet,
    ) -> ExecutionResult<CallRequestParams> {
        let ResolvedArguments {
            call_arguments,
            tetraplets,
        } = self.resolve_args(exec_ctx)?;

        let serialized_tetraplets = serde_json::to_string(&tetraplets).expect("default serializer shouldn't fail");

        let request_params = CallRequestParams::new(
            triplet.service_id.to_string(),
            triplet.function_name.to_string(),
            call_arguments,
            serialized_tetraplets,
        );

        Ok(request_params)
    }

    /// Determine whether this call should be really called and adjust prev executed trace accordingly.
    fn prepare_current_executed_state(
        &self,
        exec_ctx: &mut ExecutionCtx<'i>,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<bool> {
        let (call_result, trace_pos) = match trace_ctx.meet_call_start(&self.output)? {
            MergerCallResult::CallResult { value, trace_pos } => (value, trace_pos),
            MergerCallResult::Empty => return Ok(true),
        };

        handle_prev_state(
            &self.tetraplet,
            &self.output,
            call_result,
            trace_pos,
            exec_ctx,
            trace_ctx,
        )
    }

    /// Prepare arguments of this call instruction by resolving and preparing their security tetraplets.
    fn resolve_args(&self, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedArguments> {
        use crate::execution_step::utils::resolve_to_args;

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

/// Check output type name for being already in execution context.
// TODO: this check should be moved on a parsing stage
fn check_output_name(output: &CallOutputValue<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<()> {
    use crate::execution_step::boxed_value::Scalar;

    let scalar_name = match output {
        CallOutputValue::Variable(AstVariable::Scalar(ref name)) => *name,
        _ => return Ok(()),
    };

    if exec_ctx.met_folds.is_empty() {
        // shadowing is allowed only inside fold blocks
        return crate::exec_err!(ExecutionError::MultipleVariablesFound(scalar_name.to_string()));
    }

    match exec_ctx.scalars.get(scalar_name) {
        Some(Scalar::JValueRef(_)) => Ok(()),
        _ => return crate::exec_err!(ExecutionError::IterableShadowing(scalar_name.to_string())),
    }
}
