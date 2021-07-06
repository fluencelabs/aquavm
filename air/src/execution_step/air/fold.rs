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

mod fold_state;
mod utils;
mod variable_handler;

pub(crate) use fold_state::FoldState;
use utils::FoldIterable;
use variable_handler::VariableHandler;

use super::AValue;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::Instruction;
use super::ResolvedCallResult;
use super::TraceHandler;
use crate::execution_step::boxed_value::*;
use crate::log_instruction;
use utils::IterableValue;

use air_parser::ast::Fold;

impl<'i> ExecutableInstruction<'i> for Fold<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        match utils::construct_iterable_value(&self.iterable, exec_ctx)? {
            FoldIterable::Empty => Ok(()),
            FoldIterable::Scalar(iterable) => fold_scalar(&self, iterable, exec_ctx, trace_ctx),
            FoldIterable::Stream(iterables) => fold_stream(&self, iterables, exec_ctx, trace_ctx),
        }
    }
}

fn fold_scalar<'i>(
    fold: &Fold<'i>,
    iterable: IterableValue,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let fold_state = FoldState::from_iterable(iterable, fold.instruction.clone(), false);
    let variable_handler = VariableHandler::init(exec_ctx, fold.iterator, fold_state)?;

    fold.instruction.execute(exec_ctx, trace_ctx)?;

    variable_handler.cleanup(exec_ctx);

    Ok(())
}

fn fold_stream<'i>(
    fold: &Fold<'i>,
    stream_iterables: Vec<IterableValue>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    trace_ctx.meet_fold_start()?;

    for iterable in stream_iterables {
        let value = match iterable.peek() {
            Some(value) => value,
            // it's ok, because some generation level of a stream on some point inside execution
            // flow could contain zero values
            None => continue,
        };

        let value = value.as_value_and_pos();
        trace_ctx.meet_generation_start(&value)?;

        let fold_state = FoldState::from_iterable(iterable, fold.instruction.clone(), true);
        let variable_handler = VariableHandler::init(exec_ctx, fold.iterator, fold_state)?;

        let execution_result = fold.instruction.execute(exec_ctx, trace_ctx);

        trace_ctx.meet_generation_end()?;
        variable_handler.cleanup(exec_ctx);

        execution_result?;
    }

    trace_ctx.meet_fold_end()?;

    Ok(())
}
