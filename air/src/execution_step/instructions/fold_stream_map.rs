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

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::instructions::fold_stream::stream_execute_helpers::execute_with_stream;
use crate::execution_step::Stream;
use crate::log_instruction;

use air_parser::ast::FoldStreamMap;

impl<'i> ExecutableInstruction<'i> for FoldStreamMap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        let iterable = &self.iterable;
        if exec_ctx.stream_maps.get(iterable.name, iterable.position).is_none() {
            // having empty streams means that it haven't been met yet, and it's needed to wait
            exec_ctx.make_subgraph_incomplete();
            return Ok(());
        }

        let get_mut_stream: &dyn for<'ctx> Fn(&'ctx mut ExecutionCtx<'_>) -> &'ctx mut Stream =
            &|exec_ctx: &mut ExecutionCtx<'_>| -> &mut Stream {
                exec_ctx
                    .stream_maps
                    .get_mut(iterable.name, iterable.position)
                    .unwrap()
                    .get_mut_stream_ref()
            };

        execute_with_stream(
            exec_ctx,
            trace_ctx,
            get_mut_stream,
            self,
            self.iterator.name,
            self.instruction.clone(),
            self.last_instruction.clone(),
        )
    }
}
