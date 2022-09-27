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
use super::triplet::resolve;
use super::*;
use crate::execution_step::RcSecurityTetraplet;
use crate::execution_step::RcSecurityTetraplets;
use crate::execution_step::UncatchableError;
use crate::trace_to_exec_err;
use crate::JValue;
use crate::SecurityTetraplet;

use air_interpreter_data::CallResult;
use air_interpreter_data::TracePos;
use air_interpreter_interface::CallRequestParams;
use air_parser::ast;
use air_trace_handler::CurrentStreamValue;
use air_trace_handler::MergerCallResult;
use air_trace_handler::TraceHandler;
use air_utils::measure;

use std::rc::Rc;

/// Represents Call instruction with resolved internal parts.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct ResolvedCall<'i> {
    tetraplet: RcSecurityTetraplet,
    function_arg_paths: Rc<Vec<ast::Value<'i>>>,
    output: ast::CallOutputValue<'i>,
}

#[derive(Debug, Clone, PartialEq)]
struct ResolvedArguments {
    call_arguments: String,
    tetraplets: Vec<RcSecurityTetraplets>,
}

#[derive(Debug, Clone)]
pub(crate) enum CurrentOrPrevValue<'i> {
    CurrentStreamValue(CurrentStreamValue<'i>),
    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CallResult {
        value: CallResult,
        trace_pos: TracePos,
    },
}

impl<'i> ResolvedCall<'i> {
    /// Build `ResolvedCall` from `Call` by transforming `PeerPart` & `FunctionPart` into `ResolvedTriplet`.
    #[tracing::instrument(level = "trace", skip_all)]
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<Self> {
        let triplet = resolve(&raw_call.triplet, exec_ctx)?;
        let tetraplet = SecurityTetraplet::from_triplet(triplet);
        let tetraplet = Rc::new(tetraplet);

        check_output_name(&raw_call.output, exec_ctx)?;

        Ok(Self {
            tetraplet,
            function_arg_paths: raw_call.args.clone(),
            output: raw_call.output.clone(),
        })
    }

    /// Executes resolved instruction, updates contexts based on a execution_step result.
    #[tracing::instrument(level = "trace", skip_all)]
    pub(super) fn execute(
        &self,
        raw_call: &Call<'i>,
        exec_ctx: &mut ExecutionCtx<'i>,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<()> {
        // it's necessary to check arguments before accessing state,
        // because it would be undeterministic otherwise, for more details see
        // https://github.com/fluencelabs/aquavm/issues/214
        // also note that if there is a non-join error then the corresponding state
        // won't be saved to data
        self.check_args(exec_ctx)?;

        let state = self.prepare_current_executed_state(raw_call, exec_ctx, trace_ctx)?;
        if !state.should_execute() {
            state.maybe_set_prev_state(trace_ctx);
            return Ok(());
        }

        // call can be executed only on peers with such peer_id
        let tetraplet = &self.tetraplet;
        if tetraplet.peer_pk.as_str() != exec_ctx.run_parameters.current_peer_id.as_str() {
            set_remote_call_result(tetraplet.peer_pk.clone(), exec_ctx, trace_ctx);
            return Ok(());
        }

        let request_params = match self.prepare_request_params(exec_ctx, tetraplet) {
            Ok(params) => params,
            Err(e) if e.is_joinable() => {
                // to keep states on join behaviour
                state.maybe_set_prev_state(trace_ctx);
                return Err(e);
            }
            Err(e) => {
                return Err(e);
            }
        };
        let call_id = exec_ctx.next_call_request_id();
        exec_ctx.call_requests.insert(call_id, request_params);

        exec_ctx.subgraph_complete = false;
        trace_ctx.meet_call_end(CallResult::sent_peer_id_with_call_id(
            exec_ctx.run_parameters.current_peer_id.clone(),
            call_id,
        ));

        Ok(())
    }

    pub(super) fn as_tetraplet(&self) -> RcSecurityTetraplet {
        self.tetraplet.clone()
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn prepare_request_params(
        &self,
        exec_ctx: &ExecutionCtx<'_>,
        tetraplet: &SecurityTetraplet,
    ) -> ExecutionResult<CallRequestParams> {
        let ResolvedArguments {
            call_arguments,
            tetraplets,
        } = self.resolve_args(exec_ctx)?;

        let serialized_tetraplets = measure!(
            serde_json::to_string(&tetraplets).expect("default serializer shouldn't fail"),
            tracing::Level::INFO,
            "serde_json::to_string(tetraplets)",
        );

        let request_params = CallRequestParams::new(
            tetraplet.service_id.to_string(),
            tetraplet.function_name.to_string(),
            call_arguments,
            serialized_tetraplets,
        );

        Ok(request_params)
    }

    /// Determine whether this call should be really called and adjust prev executed trace accordingly.
    fn prepare_current_executed_state(
        &self,
        raw_call: &Call<'i>,
        exec_ctx: &mut ExecutionCtx<'i>,
        trace_ctx: &mut TraceHandler,
    ) -> ExecutionResult<StateDescriptor> {
        // TODO it seems we are to move this block into handle_prev_state and beyond
        let new_or_old_value = match trace_to_exec_err!(trace_ctx.meet_call_start(&self.output), raw_call)? {
            MergerCallResult::CallResult { value, trace_pos } => CurrentOrPrevValue::CallResult { value, trace_pos },
            // TODO It might be resolved elsewhere instead.
            MergerCallResult::CurrentStreamValue(current_stream_value) => {
                CurrentOrPrevValue::CurrentStreamValue(current_stream_value)
            }
            MergerCallResult::Empty => return Ok(StateDescriptor::no_previous_state()),
        };

        handle_prev_state(&self.tetraplet, &self.output, new_or_old_value, exec_ctx, trace_ctx)
    }

    /// Prepare arguments of this call instruction by resolving and preparing their security tetraplets.
    fn resolve_args(&self, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<ResolvedArguments> {
        use crate::execution_step::resolver::resolve_to_args;

        let function_args = self.function_arg_paths.iter();
        let mut call_arguments = Vec::with_capacity(function_args.len());
        let mut tetraplets = Vec::with_capacity(function_args.len());
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

    /// Lightweight version of resolve_args function that intended to check arguments of
    /// a call instruction. It suppresses joinable errors.
    fn check_args(&self, exec_ctx: &ExecutionCtx<'i>) -> ExecutionResult<()> {
        // TODO: make this function more lightweight
        use crate::execution_step::resolver::resolve_to_args;

        self.function_arg_paths
            .iter()
            .try_for_each(|arg_path| match resolve_to_args(arg_path, exec_ctx) {
                Ok(_) => Ok(()),
                Err(e) if e.is_joinable() => Ok(()),
                Err(e) => Err(e),
            })
    }
}

/// Check output type name for being already in execution context.
// TODO: this check should be moved on a parsing stage
fn check_output_name(output: &ast::CallOutputValue<'_>, exec_ctx: &ExecutionCtx<'_>) -> ExecutionResult<()> {
    use crate::execution_step::boxed_value::ScalarRef;

    let scalar_name = match output {
        ast::CallOutputValue::Scalar(scalar) => scalar.name,
        _ => return Ok(()),
    };

    match exec_ctx.scalars.get_value(scalar_name) {
        Ok(ScalarRef::Value(_)) => {
            if exec_ctx.scalars.variable_could_be_set(scalar_name) {
                Ok(())
            } else {
                Err(UncatchableError::ShadowingIsNotAllowed(scalar_name.to_string()).into())
            }
        }
        Ok(ScalarRef::IterableValue(_)) => Err(UncatchableError::IterableShadowing(scalar_name.to_string()).into()),
        Err(_) => Ok(()),
    }
}
