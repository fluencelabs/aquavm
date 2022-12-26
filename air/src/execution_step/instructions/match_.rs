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

use super::compare_matchable::are_matchable_eq;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::CatchableError;
use crate::execution_step::Joinable;
use crate::joinable;
use crate::log_instruction;

use air_parser::ast::Match;

impl<'i> super::ExecutableInstruction<'i> for Match<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(match_, exec_ctx, trace_ctx);

        let are_values_equal = joinable!(
            are_matchable_eq(&self.left_value, &self.right_value, exec_ctx),
            exec_ctx,
            ()
        )?;

        if !are_values_equal {
            return Err(CatchableError::MatchValuesNotEqual.into());
        }

        self.instruction.execute(exec_ctx, trace_ctx)
    }
}
