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

mod stream_cursor;

use super::fold::*;
use super::fold_scalar::fold;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::log_instruction;
use crate::trace_to_exec_err;
use stream_cursor::StreamCursor;

use air_parser::ast;
use air_parser::ast::FoldStream;
use air_values::fold_iterable_state::IterableType;
use air_values::fold_iterable_state::IterableValue;
use air_values::stream::Stream;

impl<'i> ExecutableInstruction<'i> for FoldStream<'i> {
    fn execute<VT>(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler<VT>) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);
        exec_ctx.tracker.meet_fold_stream();

        let iterable = &self.iterable;
        let stream = match exec_ctx.streams.get(iterable.name, iterable.position) {
            Some(stream) => stream,
            // it's possible to met streams without variables at the moment in fold, they are treated as empty
            None => return Ok(()),
        };

        let fold_id = exec_ctx.tracker.fold.seen_stream_count;
        trace_to_exec_err!(trace_ctx.meet_fold_start(fold_id), self)?;

        let mut stream_cursor = StreamCursor::new();
        let mut stream_iterable = stream_cursor.construct_iterables(stream);

        let mut result = Ok(true);
        while !stream_iterable.is_empty() {
            // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
            // write operation to this stream to write to this new generation
            add_new_generation_if_non_empty(&self.iterable, exec_ctx);
            result = execute_iterations(stream_iterable, self, fold_id, exec_ctx, trace_ctx);

            // it's needed to get stream again, because RefCell allows only one mutable borrowing at time,
            // and likely that stream could be mutably borrowed in execute_iterations
            let stream = remove_new_generation_if_non_empty(&self.iterable, exec_ctx);
            if should_stop_iteration(&result) {
                break;
            }

            stream_iterable = stream_cursor.construct_iterables(stream)
        }

        trace_to_exec_err!(trace_ctx.meet_fold_end(fold_id), self)?;
        result.map(|_| ())
    }
}

fn execute_iterations<'i, VT>(
    iterables: Vec<IterableValue>,
    fold_stream: &FoldStream<'i>,
    fold_id: u32,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler<VT>,
) -> ExecutionResult<bool> {
    for iterable in iterables {
        let value = match iterable.peek() {
            Some(value) => value,
            // it's ok, because some generation level of a stream on some point inside execution
            // flow could contain zero values
            None => continue,
        };

        let value_pos = value.pos();
        trace_to_exec_err!(trace_ctx.meet_iteration_start(fold_id, value_pos), fold_stream)?;
        let result = fold(
            iterable,
            IterableType::Stream(fold_id),
            fold_stream.iterator.name,
            fold_stream.instruction.clone(),
            exec_ctx,
            trace_ctx,
        );
        trace_to_exec_err!(trace_ctx.meet_generation_end(fold_id), fold_stream)?;

        result?;
        if !exec_ctx.subtree_complete {
            break;
        }
    }

    Ok(exec_ctx.subtree_complete)
}

fn should_stop_iteration(iteration_result: &ExecutionResult<bool>) -> bool {
    match &iteration_result {
        Ok(result) if !result => true,
        Ok(_) => false,
        Err(_) => true,
    }
}

/// Safety: this function should be called iff stream is present in context
fn add_new_generation_if_non_empty(stream: &ast::Stream<'_>, exec_ctx: &mut ExecutionCtx<'_>) {
    let stream = exec_ctx.streams.get_mut(stream.name, stream.position).unwrap();
    stream.add_new_generation_if_non_empty();
}

/// Safety: this function should be called iff stream is present in context
fn remove_new_generation_if_non_empty<'ctx>(
    stream: &ast::Stream<'_>,
    exec_ctx: &'ctx mut ExecutionCtx<'_>,
) -> &'ctx Stream {
    let stream = exec_ctx.streams.get_mut(stream.name, stream.position).unwrap();
    stream.remove_last_generation_if_empty();
    stream
}
