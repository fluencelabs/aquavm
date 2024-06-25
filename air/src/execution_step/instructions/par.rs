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

mod completeness_updater;

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::Instruction;
use super::TraceHandler;
use crate::log_instruction;
use crate::trace_to_exec_err;
use completeness_updater::ParCompletenessUpdater;

use air_parser::ast::Par;
use air_trace_handler::SubgraphType;

#[rustfmt::skip]
impl<'i> ExecutableInstruction<'i> for Par<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(par, exec_ctx, trace_ctx);

        let mut completeness_updater = ParCompletenessUpdater::new();
        trace_to_exec_err!(trace_ctx.meet_par_start(), self)?;

        // execute a left subgraph of par
        let left_result = execute_subgraph(self, exec_ctx, trace_ctx, &mut completeness_updater, SubgraphType::Left)?;

        // execute a right subgraph of par
        let right_result = execute_subgraph(self, exec_ctx, trace_ctx, &mut completeness_updater, SubgraphType::Right)?;

        completeness_updater.set_completeness(exec_ctx);
        prepare_par_result(left_result, right_result, exec_ctx)
    }
}

/// Execute provided subgraph and update Par state in trace_ctx.new_trace.
fn execute_subgraph<'i>(
    par: &Par<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
    completeness_updater: &mut ParCompletenessUpdater,
    subgraph_type: SubgraphType,
) -> ExecutionResult<SubgraphResult> {
    let subgraph = match subgraph_type {
        SubgraphType::Left => &par.0,
        SubgraphType::Right => &par.1,
    };
    exec_ctx.set_subgraph_completeness(determine_subgraph_complete(subgraph));

    // execute a subgraph
    let result = match subgraph.execute(exec_ctx, trace_ctx) {
        Ok(_) => {
            trace_to_exec_err!(trace_ctx.meet_par_subgraph_end(subgraph_type), par)?;
            SubgraphResult::Succeeded
        }
        Err(e) if e.is_catchable() => {
            exec_ctx.make_subgraph_incomplete();
            trace_to_exec_err!(trace_ctx.meet_par_subgraph_end(subgraph_type), par)?;
            SubgraphResult::Failed(e)
        }
        Err(e) => {
            exec_ctx.make_subgraph_incomplete();
            return Err(e);
        }
    };

    completeness_updater.observe_completeness(exec_ctx, subgraph_type);
    Ok(result)
}

enum SubgraphResult {
    Succeeded,
    Failed(ExecutionError),
}

fn prepare_par_result(
    left_result: SubgraphResult,
    right_result: SubgraphResult,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<()> {
    match (left_result, right_result) {
        (SubgraphResult::Succeeded, _) | (_, SubgraphResult::Succeeded) => {
            exec_ctx.last_error_descriptor.meet_par_successed_end();
            Ok(())
        }
        (SubgraphResult::Failed(_), SubgraphResult::Failed(err)) => Err(err),
    }
}

fn determine_subgraph_complete(next_instruction: &Instruction<'_>) -> bool {
    // this is needed to prevent situation when on such pattern
    // (fold (Iterable i
    //    (par
    //       (call ..)
    //       (next i)
    //    )
    // )
    // par will be completed after the last next that wouldn't change subgraph_complete
    !matches!(next_instruction, Instruction::Next(_))
}
