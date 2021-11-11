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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::log_instruction;

use air_parser::ast::New;
use air_parser::ast::Variable;

impl<'i> super::ExecutableInstruction<'i> for New<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(null, exec_ctx, trace_ctx);

        let position = self.position as u32;
        match &self.variable {
            Variable::Stream(stream) => {
                let iteration = exec_ctx.tracker.new_tracker.get_iteration(self.position);
                exec_ctx.streams.met_scope_start(stream.name, position, iteration);
            }
            Variable::Scalar(_scalar) => exec_ctx.scalars.meet_fold_start(),
        }

        self.instruction.execute(exec_ctx, trace_ctx)?;

        match &self.variable {
            Variable::Stream(stream) => exec_ctx.streams.met_scope_end(stream.name.to_string(), position),
            Variable::Scalar(_scalar) => exec_ctx.scalars.meet_fold_end(),
        }

        exec_ctx.tracker.meet_new(self.position);

        Ok(())
    }
}
