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
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::value_types::IterableValue;
use crate::execution_step::value_types::RecursiveCursorState;
use crate::execution_step::value_types::RecursiveStreamCursor;
use crate::execution_step::value_types::Stream;
use crate::execution_step::instructions::fold::IterableType;
use crate::execution_step::instructions::fold_scalar::fold;
use crate::trace_to_exec_err;

use air_parser::ast::Instruction;

use std::rc::Rc;

struct FoldStreamIngredients<'i> {
    iterable_name: &'i str,
    instruction: Rc<Instruction<'i>>,
    last_instruction: Option<Rc<Instruction<'i>>>,
    fold_id: u32,
}

impl<'i> FoldStreamIngredients<'i> {
    fn new(
        iterable_name: &'i str,
        instruction: Rc<Instruction<'i>>,
        last_instruction: Option<Rc<Instruction<'i>>>,
        fold_id: u32,
    ) -> Self {
        Self {
            iterable_name,
            instruction,
            last_instruction,
            fold_id,
        }
    }
}

pub(crate) fn execute_with_stream<'i>(
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    get_mut_stream: impl for<'ctx> Fn(&'ctx mut ExecutionCtx<'_>) -> &'ctx mut Stream,
    fold_to_string: &impl ToString,
    iterable_name: &'i str,
    instruction: Rc<Instruction<'i>>,
    last_instruction: Option<Rc<Instruction<'i>>>,
) -> ExecutionResult<()> {
    let fold_id = exec_ctx.tracker.meet_fold_stream();

    trace_to_exec_err!(trace_ctx.meet_fold_start(fold_id), fold_to_string)?;

    let mut recursive_stream = RecursiveStreamCursor::new();
    let mut cursor_state = recursive_stream.met_fold_start(get_mut_stream(exec_ctx));
    let mut observer = FoldGenerationObserver::new();

    // this cycle manages recursive streams
    while let RecursiveCursorState::Continue(iterables) = cursor_state {
        let ingredients =
            FoldStreamIngredients::new(iterable_name, instruction.clone(), last_instruction.clone(), fold_id);
        execute_iterations(
            iterables,
            fold_to_string,
            ingredients,
            &mut observer,
            exec_ctx,
            trace_ctx,
        )?;

        cursor_state = recursive_stream.met_iteration_end(get_mut_stream(exec_ctx));
    }

    observer.update_completeness(exec_ctx);
    trace_to_exec_err!(trace_ctx.meet_fold_end(fold_id), fold_to_string)?;
    Ok(())
}

/// Executes fold iteration over all generation that stream had at the moment of call.
/// It must return only uncatchable errors (such as ones from TraceHandler), though
/// catchable errors are suppressed and not propagated from this function, because of determinism.
/// The issue with determinism here lies in invariant that all previous executed states
/// must be met.
fn execute_iterations<'i>(
    iterables: Vec<IterableValue>,
    fold_to_string: &impl ToString,
    ingredients: FoldStreamIngredients<'i>,
    generation_observer: &mut FoldGenerationObserver,
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
        trace_to_exec_err!(
            trace_ctx.meet_iteration_start(ingredients.fold_id, value_pos),
            fold_to_string
        )?;
        let result = fold(
            iterable,
            IterableType::Stream(ingredients.fold_id),
            ingredients.iterable_name,
            ingredients.instruction.clone(),
            ingredients.last_instruction.clone(),
            exec_ctx,
            trace_ctx,
        );
        throw_error_if_not_catchable(result)?;
        trace_to_exec_err!(trace_ctx.meet_generation_end(ingredients.fold_id), fold_to_string)?;

        generation_observer.observe_completeness(exec_ctx.is_subgraph_complete());
    }

    Ok(())
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
