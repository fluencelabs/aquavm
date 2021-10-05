/*
 * Copyright 2021 Fluence Labs Limited
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

use super::*;
use crate::exec_err;
use crate::execution_step::execution_context::*;
use crate::execution_step::AstVariable;
use crate::execution_step::Generation;
use crate::execution_step::ResolvedCallResult;
use crate::execution_step::Scalar;
use crate::execution_step::Stream;

use air_interpreter_data::CallResult;
use air_interpreter_data::Value;
use air_parser::ast::CallOutputValue;
use air_trace_handler::TraceHandler;

use std::cell::RefCell;
use std::collections::hash_map::Entry::{Occupied, Vacant};

/// Writes result of a local `Call` instruction to `ExecutionCtx` at `output`.
/// Returns call result.
pub(crate) fn set_local_result<'i>(
    executed_result: ResolvedCallResult,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<CallResult> {
    let result_value = executed_result.result.clone();
    match output {
        CallOutputValue::Variable(AstVariable::Scalar(name)) => {
            set_scalar_result(executed_result, name, exec_ctx)?;
            Ok(CallResult::executed_scalar(result_value))
        }
        CallOutputValue::Variable(AstVariable::Stream(name)) => {
            // TODO: refactor this generation handling
            let generation = match exec_ctx.streams.get(*name) {
                Some(stream) => Generation::Nth(stream.borrow().generations_count() as u32 - 1),
                None => Generation::Last,
            };

            let generation = set_stream_result(executed_result, generation, name.to_string(), exec_ctx)?;
            Ok(CallResult::executed_stream(result_value, generation))
        }
        CallOutputValue::None => Ok(CallResult::executed_scalar(result_value)),
    }
}

pub(crate) fn set_result_from_value<'i>(
    value: Value,
    tetraplet: RSecurityTetraplet,
    trace_pos: usize,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    match (output, value) {
        (CallOutputValue::Variable(AstVariable::Scalar(name)), Value::Scalar(value)) => {
            let result = ResolvedCallResult::new(value, tetraplet, trace_pos);
            set_scalar_result(result, name, exec_ctx)?;
        }
        (CallOutputValue::Variable(AstVariable::Stream(name)), Value::Stream { value, generation }) => {
            let result = ResolvedCallResult::new(value, tetraplet, trace_pos);
            let generation = Generation::Nth(generation);
            let _ = set_stream_result(result, generation, name.to_string(), exec_ctx)?;
        }
        // it isn't needed to check there that output and value matches because
        // it's been already checked in trace handler
        _ => {}
    };

    Ok(())
}

#[macro_export]
macro_rules! shadowing_allowed(
    ($exec_ctx:ident, $entry:ident) => { {
        // check that current execution_step flow is inside a fold block
        if $exec_ctx.met_folds.is_empty() {
            // shadowing is allowed only inside fold blocks
            return exec_err!(ExecutionError::MultipleVariablesFound($entry.key().clone()));
        }

        match $entry.get() {
            Scalar::JValueRef(_) => {}
            // shadowing is allowed only for JValue not iterable
            _ => return exec_err!(ExecutionError::IterableShadowing($entry.key().clone())),
        };

        ExecutionResult::Ok(())
    }}
);

// TODO: decouple this function to a separate module
pub(crate) fn set_scalar_result<'i>(
    executed_result: ResolvedCallResult,
    scalar_name: &'i str,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    meet_scalar(scalar_name, executed_result.clone(), exec_ctx)?;

    match exec_ctx.scalars.entry(scalar_name.to_string()) {
        Vacant(entry) => {
            entry.insert(Scalar::JValueRef(executed_result));
        }
        Occupied(mut entry) => {
            // the macro instead of a function because of borrowing
            shadowing_allowed!(exec_ctx, entry)?;
            entry.insert(Scalar::JValueRef(executed_result));
        }
    };

    Ok(())
}

/// Inserts meet variable name into met calls in fold to allow shadowing.
fn meet_scalar<'i>(
    scalar_name: &'i str,
    executed_result: ResolvedCallResult,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    if let Some(fold_block_name) = exec_ctx.met_folds.back() {
        let fold_state = match exec_ctx.scalars.get_mut(*fold_block_name) {
            Some(Scalar::JValueFoldCursor(fold_state)) => fold_state,
            _ => unreachable!("fold block data must be represented as fold cursor"),
        };

        fold_state.met_variables.insert(scalar_name, executed_result);
    }

    Ok(())
}

// TODO: decouple this function to a separate module
pub(crate) fn set_stream_result(
    executed_result: ResolvedCallResult,
    generation: Generation,
    stream_name: String,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<u32> {
    let generation = match exec_ctx.streams.entry(stream_name) {
        Occupied(mut entry) => {
            // if result is an array, insert result to the end of the array
            entry.get_mut().borrow_mut().add_value(executed_result, generation)?
        }
        Vacant(entry) => {
            let stream = Stream::from_value(executed_result);
            entry.insert(RefCell::new(stream));
            0
        }
    };

    Ok(generation)
}

/// Writes an executed state of a particle being sent to remote node.
pub(crate) fn set_remote_call_result<'i>(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_call_result = CallResult::sent_peer_id(exec_ctx.current_peer_id.clone());
    trace_ctx.meet_call_end(new_call_result);
}
