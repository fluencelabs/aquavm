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
use super::ExecutionTraceCtx;
use crate::log_instruction;

use air_parser::ast::Xor;

impl<'i> super::ExecutableInstruction<'i> for Xor<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        log_instruction!(xor, exec_ctx, trace_ctx);

        exec_ctx.subtree_complete = true;
        match self.0.execute(exec_ctx, trace_ctx) {
            Err(e) if is_catchable_by_xor(&e) => {
                exec_ctx.subtree_complete = true;
                exec_ctx.last_error_could_be_set = true;
                print_xor_log(&e);

                self.1.execute(exec_ctx, trace_ctx)
            }
            res => res,
        }
    }
}

/// Returns true, if this execution error type should be caught by xor.
fn is_catchable_by_xor(exec_error: &ExecutionError) -> bool {
    // this type of errors related to invalid data and should treat as hard errors.
    !matches!(exec_error, ExecutionError::InvalidExecutedState { .. })
}

fn print_xor_log(e: &ExecutionError) {
    match e {
        // These errors actually aren't real errors, but a way to bubble execution up from match
        // to a corresponding xor. They'll become errors iff there is no such xor and execution is
        // bubble up until the very beginning of current subtree. So the error message shouldn't
        // be print out in order not to confuse users.
        ExecutionError::MatchWithoutXorError | ExecutionError::MismatchWithoutXorError => {}
        e => log::warn!("xor caught an error: {}", e),
    }
}
