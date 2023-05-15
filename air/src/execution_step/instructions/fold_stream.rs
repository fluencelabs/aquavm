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

pub(crate) mod completeness_updater;
mod stream_cursor;
pub(crate) mod stream_execute_helpers;

use super::fold::*;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::FoldStreamLikeIngredients;
use super::TraceHandler;
use crate::execution_step::boxed_value::Stream;
use crate::log_instruction;
use air_parser::ast::FoldStream;
use air_parser::AirPos;
pub(crate) use stream_execute_helpers::execute_with_stream;

impl<'i> ExecutableInstruction<'i> for FoldStream<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);
        exec_ctx.tracker.meet_fold_stream();

        let iterable = &self.iterable;
        match exec_ctx.streams.get(iterable.name, iterable.position) {
            Some(stream) => stream,
            None => {
                // having empty streams means that it haven't been met yet, and it's needed to wait
                exec_ctx.make_subgraph_incomplete();
                return Ok(());
            }
        };
        execute_with_stream(
            exec_ctx,
            trace_ctx,
            get_mut_stream_from_context,
            get_stream_from_context,
            self,
        )
    }
}

/// Safety: this function should be called iff stream is present in context
fn get_mut_stream_from_context<'c>(name: &str, position: AirPos, exec_ctx: &'c mut ExecutionCtx<'_>) -> &'c mut Stream {
    exec_ctx.streams.get_mut(name, position).unwrap()
}

fn get_stream_from_context<'c>(name: &str, position: AirPos, exec_ctx: &'c mut ExecutionCtx<'_>) -> &'c Stream {
    exec_ctx.streams.get_mut(name, position).unwrap()
}

impl<'i> FoldStreamLikeIngredients for FoldStream<'i> {
    type Item = air_parser::ast::Stream<'i>;

    fn iterable_name(&self) -> &'i str {
        self.iterable.name
    }

    fn iterable_pos(&self) -> air_parser::AirPos {
        self.iterable.position
    }

    fn iterator_name(&self) -> &'i str {
        self.iterator.name
    }

    fn instruction(&self) -> std::rc::Rc<air_parser::ast::Instruction<'i>> {
        self.instruction.clone()
    }

    fn last_instruction(&self) -> Option<std::rc::Rc<air_parser::ast::Instruction<'i>>> {
        self.last_instruction.clone()
    }
}
