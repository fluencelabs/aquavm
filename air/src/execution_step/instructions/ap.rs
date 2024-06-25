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

pub(super) mod apply_to_arguments;
pub(super) mod utils;

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::Joinable;
use crate::execution_step::ValueAggregate;
use crate::joinable;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::JValue;

use apply_to_arguments::apply_to_arg;
use utils::*;

use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::merger::MergerApResult;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(ap, exec_ctx, trace_ctx);
        let should_touch_trace = should_touch_trace(self);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = joinable!(
            apply_to_arg(&self.argument, exec_ctx, trace_ctx, should_touch_trace),
            exec_ctx,
            ()
        )?;

        let merger_ap_result = to_merger_ap_result(self, trace_ctx)?;
        populate_context(&self.result, &merger_ap_result, result, exec_ctx)?;
        maybe_update_trace(should_touch_trace, trace_ctx);

        Ok(())
    }
}

/// This function is intended to check whether a Ap instruction should produce
/// a new state in data.
fn should_touch_trace(ap: &Ap<'_>) -> bool {
    matches!(ap.result, ast::ApResult::Stream(_))
}

fn to_merger_ap_result(instr: &Ap<'_>, trace_ctx: &mut TraceHandler) -> ExecutionResult<MergerApResult> {
    match instr.result {
        ast::ApResult::Scalar(_) => Ok(MergerApResult::NotMet),
        ast::ApResult::Stream(_) => {
            let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
            Ok(merger_ap_result)
        }
    }
}

fn populate_context<'ctx>(
    ap_result: &ast::ApResult<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    match ap_result {
        ast::ApResult::Scalar(scalar) => exec_ctx.scalars.set_scalar_value(scalar.name, result).map(|_| ()),
        ast::ApResult::Stream(stream) => {
            let value_descriptor = generate_value_descriptor(result, stream, merger_ap_result);
            exec_ctx.streams.add_stream_value(value_descriptor)
        }
    }
}

fn maybe_update_trace(should_touch_trace: bool, trace_ctx: &mut TraceHandler) {
    use air_interpreter_data::ApResult;

    if should_touch_trace {
        trace_ctx.meet_ap_end(ApResult::stub());
    }
}
