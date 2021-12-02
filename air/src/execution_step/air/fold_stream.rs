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

use super::fold::*;
use super::fold_scalar::fold;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::Joinable;
use crate::joinable;
use crate::log_instruction;
use crate::trace_to_exec_err;

use air_parser::ast::FoldStream;

impl<'i> ExecutableInstruction<'i> for FoldStream<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        let stream_iterable = joinable!(construct_stream_iterable_value(&self.iterable, exec_ctx), exec_ctx)?;
        let iterables = match stream_iterable {
            FoldIterableStream::Empty => return Ok(()),
            FoldIterableStream::Stream(iterables) => iterables,
        };

        let fold_id = exec_ctx.tracker.fold.seen_stream_count;
        trace_to_exec_err!(trace_ctx.meet_fold_start(fold_id))?;
        exec_ctx.scalars.meet_fold_start();

        // here it's necessary to early exit from a call not calling trace handler,
        // because error handling is done by macro execute_fold_stream!
        execute_iterations(iterables, self, fold_id, exec_ctx, trace_ctx)?;

        trace_to_exec_err!(trace_ctx.meet_fold_end(fold_id))?;
        exec_ctx.scalars.meet_fold_end();

        Ok(())
    }
}

fn execute_iterations<'i>(
    iterables: Vec<IterableValue>,
    fold_stream: &FoldStream<'i>,
    fold_id: u32,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    for iterable in iterables {
        let value = match iterable.peek() {
            Some(value) => value,
            // it's ok, because some generation level of a stream on some point inside execution
            // flow could contain zero values
            None => continue,
        };

        let value_pos = value.pos();
        trace_to_exec_err!(trace_ctx.meet_iteration_start(fold_id, value_pos))?;
        fold(
            iterable,
            IterableType::Stream(fold_id),
            fold_stream.iterator.name,
            fold_stream.instruction.clone(),
            exec_ctx,
            trace_ctx,
        )?;
        trace_to_exec_err!(trace_ctx.meet_generation_end(fold_id))?;

        if !exec_ctx.subtree_complete {
            break;
        }
    }

    Ok(())
}
