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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::log_instruction;

use air_parser::ast::New;
use air_parser::ast::Variable;

impl<'i> super::ExecutableInstruction<'i> for New<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(new, exec_ctx, trace_ctx);

        prolog(self, exec_ctx);
        // it should be a lazy error evaluating after execution of epilog block, since it's
        // necessary to return a restricted variable to it's previous state in case of
        // any error. It's highly important to distinguish between global and restricted streams
        // at the end of execution to make a correct data.
        let instruction_result = self.instruction.execute(exec_ctx, trace_ctx);
        epilog(self, exec_ctx);

        instruction_result
    }
}

fn prolog<'i>(new: &New<'i>, exec_ctx: &mut ExecutionCtx<'i>) {
    let position = new.span.left;
    match &new.variable {
        Variable::Stream(stream) => {
            let iteration = exec_ctx.tracker.new_tracker.get_iteration(position);
            exec_ctx.streams.meet_scope_start(stream.name, new.span, iteration);
        }
        // noop
        Variable::Scalar(_) => {}
    }

    exec_ctx.tracker.meet_new(position);
}

fn epilog<'i>(new: &New<'i>, exec_ctx: &mut ExecutionCtx<'i>) {
    let position = new.span.left;
    match &new.variable {
        Variable::Stream(stream) => exec_ctx
            .streams
            .meet_scope_end(stream.name.to_string(), position as u32),
        // noop
        Variable::Scalar(_) => {}
    }
}
