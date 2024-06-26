/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::fold::IterableType;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::FoldState;
use super::TraceHandler;
use crate::execution_step::PEEK_ALLOWED_ON_NON_EMPTY;
use crate::log_instruction;
use crate::trace_to_exec_err;

use air_parser::ast::Next;

impl<'i> super::ExecutableInstruction<'i> for Next<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(next, exec_ctx, trace_ctx);

        let iterator_name = &self.iterator.name;
        let fold_state = exec_ctx.scalars.get_iterable_mut(iterator_name)?;
        maybe_meet_iteration_end(self, fold_state, trace_ctx)?;

        // TODO: refactor a body of this if to reduce LOCs count and improve readability
        if !fold_state.iterable.next() {
            maybe_meet_back_iterator(self, fold_state, trace_ctx)?;

            let fold_state = exec_ctx.scalars.get_iterable(iterator_name)?;
            // execute last instruction if any
            if let Some(last_instr) = &fold_state.last_instr_head {
                let last_instr = last_instr.clone();
                exec_ctx.flush_subgraph_completeness(); // it's needed because of determine_subgraph_complete in par
                last_instr.execute(exec_ctx, trace_ctx)?;
            } else {
                // if no last instruction, execute never as a fallback for fold over stream (it'll be removed in future)
                let fold_state = exec_ctx.scalars.get_iterable_mut(iterator_name)?;
                if !fold_state.back_iteration_started && matches!(fold_state.iterable_type, IterableType::Stream(_)) {
                    fold_state.back_iteration_started = true;
                    // this set the last iteration of a next to not executed for fold over streams
                    // for more info see https://github.com/fluencelabs/aquavm/issues/333
                    exec_ctx.make_subgraph_incomplete();
                }
            }

            // just do nothing to exit
            return Ok(());
        }

        let next_instr = fold_state.instr_head.clone();
        maybe_meet_iteration_start(self, fold_state, trace_ctx)?;
        exec_ctx.scalars.meet_next_before();

        let result = next_instr.execute(exec_ctx, trace_ctx);
        exec_ctx.scalars.meet_next_after();
        result?;

        // get the same fold state again because of borrow checker
        let fold_state = exec_ctx.scalars.get_iterable_mut(iterator_name)?;
        fold_state.iterable.prev();
        maybe_meet_back_iterator(self, fold_state, trace_ctx)?;

        Ok(())
    }
}

fn maybe_meet_iteration_start<'i>(
    next: &Next<'i>,
    fold_state: &FoldState<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    if let IterableType::Stream(fold_id) = &fold_state.iterable_type {
        trace_to_exec_err!(
            trace_ctx.meet_iteration_start(
                *fold_id,
                fold_state.iterable.peek().expect(PEEK_ALLOWED_ON_NON_EMPTY).pos()
            ),
            next
        )?;
    }

    Ok(())
}

fn maybe_meet_iteration_end<'i>(
    next: &Next<'i>,
    fold_state: &FoldState<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    if let IterableType::Stream(fold_id) = &fold_state.iterable_type {
        trace_to_exec_err!(trace_ctx.meet_iteration_end(*fold_id), next)?;
    }

    Ok(())
}

fn maybe_meet_back_iterator<'i>(
    next: &Next<'i>,
    fold_state: &FoldState<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    if let IterableType::Stream(fold_id) = &fold_state.iterable_type {
        trace_to_exec_err!(trace_ctx.meet_back_iterator(*fold_id), next)?;
    }

    Ok(())
}
