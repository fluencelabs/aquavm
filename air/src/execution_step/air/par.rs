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

mod completeness_updater;

use super::Catchable;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::Instruction;
use super::TraceHandler;
use crate::execution_step::trace_handler::SubtreeType;
use crate::log_instruction;
use completeness_updater::ParCompletenessUpdater;

use air_parser::ast::Par;
use std::rc::Rc;

#[rustfmt::skip]
impl<'i> ExecutableInstruction<'i> for Par<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(par, exec_ctx, trace_ctx);

        let mut completeness_updater = ParCompletenessUpdater::new();
        trace_ctx.meet_par_start()?;

        // execute a left subtree of par
        let left_result = execute_subtree(&self.0, exec_ctx, trace_ctx, &mut completeness_updater, SubtreeType::Left)?;

        // execute a right subtree of par
        let right_result = execute_subtree(&self.1, exec_ctx, trace_ctx, &mut completeness_updater, SubtreeType::Right)?;

        completeness_updater.set_completeness(exec_ctx);
        prepare_par_result(left_result, right_result, exec_ctx)
    }
}

/// Execute provided subtree and update Par state in trace_ctx.new_trace.
fn execute_subtree<'i>(
    subtree: &Instruction<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    completeness_updater: &mut ParCompletenessUpdater,
    subtree_type: SubtreeType,
) -> ExecutionResult<SubtreeResult> {
    exec_ctx.subtree_complete = determine_subtree_complete(subtree);

    // execute a subtree
    let result = match subtree.execute(exec_ctx, trace_ctx) {
        Ok(_) => {
            trace_ctx.meet_par_subtree_end(subtree_type)?;
            SubtreeResult::Succeeded
        }
        Err(e) if !e.is_catchable() => {
            return Err(e);
        }
        Err(e) => {
            trace_ctx.meet_par_subtree_end(subtree_type)?;
            SubtreeResult::Failed(e)
        }
    };

    completeness_updater.update_completeness(exec_ctx, subtree_type);
    Ok(result)
}

enum SubtreeResult {
    Succeeded,
    Failed(Rc<ExecutionError>),
}

fn prepare_par_result(
    left_result: SubtreeResult,
    right_result: SubtreeResult,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<()> {
    match (left_result, right_result) {
        (SubtreeResult::Succeeded, _) | (_, SubtreeResult::Succeeded) => {
            // clear the last error in case of par succeeded
            exec_ctx.last_error = None;
            Ok(())
        }
        (SubtreeResult::Failed(_), SubtreeResult::Failed(err)) => Err(err),
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
