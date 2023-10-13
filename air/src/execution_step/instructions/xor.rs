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
use super::ExecutionError;
use super::ExecutionResult;
use super::TraceHandler;
use crate::log_instruction;

use air_parser::ast::Xor;

impl<'i> super::ExecutableInstruction<'i> for Xor<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(xor, exec_ctx, trace_ctx);

        exec_ctx.flush_subgraph_completeness();
        match self.0.execute(exec_ctx, trace_ctx) {
            Err(e) if e.is_catchable() => {
                print_xor_log(&e);

                exec_ctx.flush_subgraph_completeness();
                exec_ctx.last_error_descriptor.meet_xor_right_branch();

                exec_ctx.error_descriptor.set_original_execution_error(&e);
                exec_ctx.error_descriptor.enable_error_setting();

                let right_subgraph_result = self.1.execute(exec_ctx, trace_ctx);
                // This sets :error: to a no-error state.
                // Please note the right_subgraph_result might be an Error that bubbles up to an :error:
                // above this execute().
                exec_ctx.error_descriptor.clear_error_object_if_needed();

                if right_subgraph_result.is_ok() {
                    exec_ctx.error_descriptor.enable_error_setting();
                }

                right_subgraph_result
            }
            res => res,
        }
    }
}

fn print_xor_log(e: &ExecutionError) {
    if e.is_match_or_mismatch() {
        // These errors actually aren't real errors, but a way to bubble execution_step up from match
        // to a corresponding xor. They'll become errors iff there is no such xor and execution_step is
        // bubble up until the very beginning of current subgraph. So the error message shouldn't
        // be print out in order not to confuse users.
        return;
    }

    log::trace!("xor caught an error: {}", e);
}
