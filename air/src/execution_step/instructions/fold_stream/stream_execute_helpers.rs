/*
 * Copyright 2023 Fluence Labs Limited
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

use super::completeness_updater::FoldGenerationObserver;
use super::stream_cursor::StreamCursor;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::boxed_value::Stream;
use crate::execution_step::instructions::fold::IterableType;
use crate::execution_step::instructions::fold::IterableValue;
use crate::execution_step::instructions::fold_scalar::fold;
use crate::execution_step::instructions::FoldStreamLikeIngredients;
use crate::trace_to_exec_err;
use air_parser::AirPos;

type MutStreamGetterClosure = for<'c> fn(&str, AirPos, &'c mut ExecutionCtx<'_>) -> &'c mut Stream;
type StreamGetterClosure = for<'c> fn(&str, AirPos, &'c mut ExecutionCtx<'_>) -> &'c Stream;

pub(crate) fn execute_with_stream<'i>(
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    get_mut_stream: MutStreamGetterClosure,
    get_stream: StreamGetterClosure,
    fold_stream_like: &'i (impl FoldStreamLikeIngredients + ToString),
) -> ExecutionResult<()> {
    let fold_id = exec_ctx.tracker.fold.seen_stream_count;

    let iterable_name = fold_stream_like.iterable_name();
    let iterable_pos = fold_stream_like.iterable_pos();

    trace_to_exec_err!(trace_ctx.meet_fold_start(fold_id), fold_stream_like)?;

    let mut stream_cursor = StreamCursor::new();
    let mut stream_iterable = stream_cursor.construct_iterables(get_stream(iterable_name, iterable_pos, exec_ctx));
    let mut observer = FoldGenerationObserver::new();
    // this cycle manages recursive streams
    while !stream_iterable.is_empty() {
        // add a new generation to made all consequence "new" (meaning that they are just executed on this peer)
        // write operation to this stream to write to this new generation
        add_new_generation_if_non_empty(get_mut_stream(iterable_name, iterable_pos, exec_ctx));
        execute_iterations(
            stream_iterable,
            &mut observer,
            exec_ctx,
            trace_ctx,
            fold_stream_like,
            fold_id,
        )?;

        // it's needed to get stream again, because RefCell allows only one mutable borrowing at time,
        // and likely that stream could be mutably borrowed in execute_iterations
        let stream = remove_new_generation_if_non_empty(get_mut_stream(iterable_name, iterable_pos, exec_ctx));

        stream_iterable = stream_cursor.construct_iterables(stream)
    }

    observer.update_completeness(exec_ctx);
    trace_to_exec_err!(trace_ctx.meet_fold_end(fold_id), fold_stream_like)?;
    Ok(())
}

/// Executes fold iteration over all generation that stream had at the moment of call.
/// It must return only uncatchable errors (such as ones from TraceHandler), though
/// catchable errors are suppressed and not propagated from this function, because of determinism.
/// The issue with determinism here lies in invariant that all previous executed states
/// must be met.
pub(super) fn execute_iterations<'i>(
    iterables: Vec<IterableValue>,
    generation_observer: &mut FoldGenerationObserver,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    fold_stream_like: &'i (impl FoldStreamLikeIngredients + ToString),
    fold_id: u32,
) -> ExecutionResult<()> {
    // let (fold_impl_to_str, fold_stream_it_name, fold_instruction, fold_last_instruction, fold_id) = fold_related_args;
    let iterator_name = fold_stream_like.iterator_name();
    for iterable in iterables.into_iter() {
        let value = match iterable.peek() {
            Some(value) => value,
            // it's ok, because some generation level of a stream on some point inside execution
            // flow could contain zero values
            None => continue,
        };
        let value_pos = value.pos();
        trace_to_exec_err!(trace_ctx.meet_iteration_start(fold_id, value_pos), fold_stream_like)?;
        let result = fold(
            iterable,
            IterableType::Stream(fold_id),
            iterator_name,
            fold_stream_like.instruction(),
            fold_stream_like.last_instruction(),
            exec_ctx,
            trace_ctx,
        );
        throw_error_if_not_catchable(result)?;
        trace_to_exec_err!(trace_ctx.meet_generation_end(fold_id), fold_stream_like)?;

        generation_observer.observe_completeness(exec_ctx.is_subgraph_complete());
    }

    Ok(())
}

/// Safety: this function should be called iff stream is present in context
pub(super) fn add_new_generation_if_non_empty(stream: &mut Stream) {
    stream.add_new_generation_if_non_empty();
}

/// Safety: this function should be called iff stream is present in context
pub(super) fn remove_new_generation_if_non_empty(stream: &mut Stream) -> &Stream {
    stream.remove_last_generation_if_empty();
    stream
}

/// Fold over streams doesn't throw an error if it's a catchable one, because otherwise it would be
/// not deterministic.
pub(super) fn throw_error_if_not_catchable(result: ExecutionResult<()>) -> ExecutionResult<()> {
    match result {
        Ok(_) => Ok(()),
        Err(error) if error.is_catchable() => Ok(()),
        error @ Err(_) => error,
    }
}
