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

use super::compare_matchable::are_matchable_eq;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::CatchableError;
use crate::execution_step::Joinable;
use crate::joinable;
use crate::log_instruction;

use air_parser::ast::MisMatch;

impl<'i> super::ExecutableInstruction<'i> for MisMatch<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(mismatch_, exec_ctx, trace_ctx);

        let are_values_equal = joinable!(
            are_matchable_eq(&self.left_value, &self.right_value, exec_ctx),
            exec_ctx,
            ()
        )?;

        if are_values_equal {
            return Err(CatchableError::MismatchValuesEqual.into());
        }

        self.instruction.execute(exec_ctx, trace_ctx)
    }
}
