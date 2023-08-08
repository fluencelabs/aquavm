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

pub(super) mod completeness_updater;
pub(super) mod stream_execute_helpers;

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::instructions::fold_stream::stream_execute_helpers::execute_with_stream;
use crate::execution_step::value_types::Stream;
use crate::log_instruction;

use air_parser::ast::FoldStream;

impl<'i> ExecutableInstruction<'i> for FoldStream<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        let iterable = &self.iterable;
        if exec_ctx.streams.get(iterable.name, iterable.position).is_none() {
            // having empty streams means that it haven't been met yet, and it's needed to wait
            exec_ctx.make_subgraph_incomplete();
            return Ok(());
        }

        let get_mut_stream: &dyn for<'ctx> Fn(&'ctx mut ExecutionCtx<'_>) -> &'ctx mut Stream =
            &|exec_ctx: &mut ExecutionCtx<'_>| -> &mut Stream {
                exec_ctx.streams.get_mut(iterable.name, iterable.position).unwrap()
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
