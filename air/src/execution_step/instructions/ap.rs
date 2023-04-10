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

mod apply_to_arguments;
mod utils;

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::instructions::ValueAggregate;
use crate::execution_step::resolver::apply_lambda;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::JValue;
use crate::SecurityTetraplet;
use apply_to_arguments::*;
use utils::*;

use air_interpreter_data::GenerationIdx;
use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::merger::MergerApResult;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        let should_touch_trace = should_touch_trace(self);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = apply_to_arg(&self.argument, exec_ctx, trace_ctx, should_touch_trace)?;

        let merger_ap_result = to_merger_ap_result(self, trace_ctx)?;
        let maybe_generation = populate_context(&self.result, &merger_ap_result, result, exec_ctx)?;
        maybe_update_trace(maybe_generation, trace_ctx);

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
) -> ExecutionResult<Option<GenerationIdx>> {
    match ap_result {
        ast::ApResult::Scalar(scalar) => exec_ctx.scalars.set_scalar_value(scalar.name, result).map(|_| None),
        ast::ApResult::Stream(stream) => {
            let value_descriptor = generate_value_descriptor(result, stream, merger_ap_result);
            exec_ctx.streams.add_stream_value(value_descriptor).map(Some)
        }
    }
}

fn maybe_update_trace(maybe_generation: Option<GenerationIdx>, trace_ctx: &mut TraceHandler) {
    use air_interpreter_data::ApResult;

    if let Some(generation) = maybe_generation {
        let final_ap_result = ApResult::new(generation);
        trace_ctx.meet_ap_end(final_ap_result);
    }
}
