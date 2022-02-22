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
use crate::execution_step::boxed_value::Generation;
use crate::log_instruction;
use crate::trace_to_exec_err;

use air_parser::ast::FoldStream;

impl<'i> ExecutableInstruction<'i> for FoldStream<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);
        println!("met fold stream");
        exec_ctx.tracker.meet_fold_stream();

        let iterable = &self.iterable;
        let stream = match exec_ctx.streams.get(iterable.name, iterable.position) {
            Some(stream) if stream.borrow().is_empty() => return Ok(()),
            Some(stream) => stream,
            // it's possible to met streams without variables at the moment in fold, they are treated as empty
            None => return Ok(()),
        };

        let fold_id = exec_ctx.tracker.fold.seen_stream_count;
        trace_to_exec_err!(trace_ctx.meet_fold_start(fold_id), self)?;

        let mut last_generation = stream.borrow().non_empty_generations_count() as u32;
        let mut stream_iterable = construct_stream_iterable_value(stream, Generation::Nth(0), Generation::Last);
        while !stream_iterable.is_empty() {
            // it's safe because it's already checked that stream with such a name and position presence in context
            let stream = exec_ctx.streams.get(iterable.name, iterable.position).unwrap();
            println!("stream {} before iteration: {}", iterable.name, stream.borrow());

            // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
            // write operation to this stream to write to this new generation
            let _generation_added = stream.borrow_mut().add_new_generation_if_non_empty();
            let result = execute_iterations(stream_iterable, self, fold_id, exec_ctx, trace_ctx);

            // it's safe because stream can't be deleted after iterating
            // it's needed to get stream again, because RefCell allows only one mutable borrowing at time,
            // and likely that stream could be mutably borrowed in execute_iterations
            let stream = exec_ctx.streams.get(iterable.name, iterable.position).unwrap();
            println!("stream {} after iteration: {}", iterable.name, stream.borrow());
            stream.borrow_mut().remove_last_generation_if_empty();
            if result.is_err() {
                break;
            }

            stream_iterable = construct_stream_iterable_value(stream, Generation::Nth(last_generation), Generation::Last);
            last_generation = stream.borrow().non_empty_generations_count() as u32;
        };

        trace_to_exec_err!(trace_ctx.meet_fold_end(fold_id), self)?;
        println!("met fold end");
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
    println!("  execute iterations {}", iterables.len());
    for iterable in iterables {
        let value = match iterable.peek() {
            Some(value) => value,
            // it's ok, because some generation level of a stream on some point inside execution
            // flow could contain zero values
            None => continue,
        };
        println!("  execute iteration with {:?}", value);

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
            println!("  subtree incomplete");
            // break;
        }
    }

    Ok(())
}
