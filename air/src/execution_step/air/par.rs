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

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::Instruction;
use super::TraceHandler;
use crate::execution_step::trace_handler::SubtreeType;
use crate::log_instruction;
use completeness_updater::ParCompletenessUpdater;

use air_parser::ast::Par;

#[rustfmt::skip]
impl<'i> ExecutableInstruction<'i> for Par<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(par, exec_ctx, trace_ctx);

        trace_ctx.meet_par_start()?;
        let mut completeness_updater = ParCompletenessUpdater::new();

        // execute a left subtree of par
        execute_subtree(&self.0, exec_ctx, trace_ctx, &mut completeness_updater, SubtreeType::Left)?;

        // execute a right subtree of par
        execute_subtree(&self.1, exec_ctx, trace_ctx, &mut completeness_updater, SubtreeType::Right)?;

        completeness_updater.set_completeness(exec_ctx);

        Ok(())
    }
}

/// Execute provided subtree and update Par state in trace_ctx.new_trace.
fn execute_subtree<'i>(
    subtree: &Instruction<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    completeness_updater: &mut ParCompletenessUpdater,
    subtree_type: SubtreeType,
) -> ExecutionResult<()> {
    exec_ctx.subtree_complete = determine_subtree_complete(subtree);

    // execute a subtree
    let exec_result = subtree.execute(exec_ctx, trace_ctx);
    completeness_updater.update_completeness(exec_ctx, subtree_type);
    trace_ctx.meet_par_subtree_end(subtree_type)?;

    exec_result
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
