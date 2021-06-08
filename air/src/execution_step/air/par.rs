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

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use super::Instruction;
use crate::contexts::execution_trace::ExecutedState;
use crate::contexts::execution_trace::ParResult;
use crate::log_instruction;
use crate::log_targets::EXECUTED_STATE_CHANGING;

use air_parser::ast::Par;

enum SubtreeType {
    Left,
    Right,
}

impl std::fmt::Display for SubtreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

impl<'i> ExecutableInstruction<'i> for Par<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        use SubtreeType::*;

        log_instruction!(par, exec_ctx, trace_ctx);

        let (left_subtree_size, right_subtree_size) = extract_subtree_sizes(trace_ctx)?;

        let par_pos = trace_ctx.new_trace.len();
        trace_ctx.new_trace.push_back(ExecutedState::Par(ParResult::default()));

        // execute a left subtree of this par
        execute_subtree(&self.0, left_subtree_size, exec_ctx, trace_ctx, par_pos, Left)?;
        let left_subtree_complete = exec_ctx.subtree_complete;

        // execute a right subtree of this par
        execute_subtree(&self.1, right_subtree_size, exec_ctx, trace_ctx, par_pos, Right)?;
        let right_subtree_complete = exec_ctx.subtree_complete;

        // par is completed if at least one of its subtrees is completed
        exec_ctx.subtree_complete = left_subtree_complete || right_subtree_complete;

        Ok(())
    }
}

fn extract_subtree_sizes(trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<(usize, usize)> {
    use super::ExecutionError::InvalidExecutedState;

    if trace_ctx.current_subtree_size == 0 {
        return Ok((0, 0));
    }

    trace_ctx.current_subtree_size -= 1;

    log::trace!(
        target: EXECUTED_STATE_CHANGING,
        "  previous call executed state was found {:?}",
        trace_ctx.current_trace[0]
    );

    // unwrap is safe here because of length's been checked
    match trace_ctx.current_trace.pop_front().unwrap() {
        ExecutedState::Par(ParResult(left, right)) => Ok((left, right)),
        state => crate::exec_err!(InvalidExecutedState(String::from("par"), state)),
    }
}

/// Execute provided subtree and update Par state in trace_ctx.new_trace.
fn execute_subtree<'i>(
    subtree: &Instruction<'i>,
    subtree_size: usize,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut ExecutionTraceCtx,
    current_par_pos: usize,
    subtree_type: SubtreeType,
) -> ExecutionResult<()> {
    use super::ExecutionError::LocalServiceError;

    let before_subtree_size = trace_ctx.current_subtree_size;
    trace_ctx.current_subtree_size = subtree_size;
    let before_new_path_len = trace_ctx.new_trace.len();

    exec_ctx.subtree_complete = determine_subtree_complete(&subtree);

    // execute a subtree
    match subtree.execute(exec_ctx, trace_ctx) {
        res @ Ok(_) => {
            update_par_state(trace_ctx, subtree_type, current_par_pos, before_new_path_len);
            trace_ctx.current_subtree_size = before_subtree_size - subtree_size;
            res
        }
        // if there is a service error, update already added Par state
        // and then bubble the error up
        Err(err) if matches!(&*err, LocalServiceError(..)) => {
            update_par_state(trace_ctx, subtree_type, current_par_pos, before_new_path_len);
            trace_ctx.current_subtree_size = before_subtree_size - subtree_size;
            Err(err)
        }
        err @ Err(_) => err,
    }
}

fn determine_subtree_complete(next_instruction: &Instruction<'_>) -> bool {
    // this is needed to prevent situation when on such pattern
    // (fold (Iterable i
    //    (par
    //       (call ..)
    //       (next i)
    //    )
    // )
    // par will be completed after the last next that wouldn't change subtree_complete
    !matches!(next_instruction, Instruction::Next(_))
}

/// Set left or right fields of a Par identified by current_par_pos.
fn update_par_state(
    trace_ctx: &mut ExecutionTraceCtx,
    subtree_type: SubtreeType,
    current_par_pos: usize,
    before_new_path_len: usize,
) {
    let new_subtree_size = trace_ctx.new_trace.len() - before_new_path_len;

    // unwrap is safe here, because this par is added at the beginning of this par instruction.
    let par_state = trace_ctx.new_trace.get_mut(current_par_pos).unwrap();
    match par_state {
        ExecutedState::Par(ParResult(left, right)) => {
            if let SubtreeType::Left = subtree_type {
                *left = new_subtree_size;
            } else {
                *right = new_subtree_size;
            }

            log::trace!(
                target: EXECUTED_STATE_CHANGING,
                "  set {} par subtree size to {}",
                subtree_type,
                new_subtree_size
            );
        }
        _ => unreachable!("current_pas_pos must point to a par state"),
    }
}
