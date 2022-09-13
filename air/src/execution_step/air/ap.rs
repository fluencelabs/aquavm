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
use crate::execution_step::air::ValueAggregate;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::resolver::apply_lambda;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::JValue;
use crate::SecurityTetraplet;
use apply_to_arguments::*;
use utils::*;

use air_interpreter_data as trace;
use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::MergerApResult;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = apply_to_arg(&self.argument, exec_ctx, trace_ctx)?;

        let merger_ap_result = try_meet_ap_start(self, trace_ctx)?;
        let final_ap_result = handle_ap(&self.result, &merger_ap_result, result, exec_ctx)?;
        trace_ctx.meet_ap_stream_end(final_ap_result);

        Ok(())
    }
}

fn try_meet_ap_start(instr: &Ap<'_>, trace_ctx: &mut TraceHandler) -> ExecutionResult<MergerApResult> {
    use crate::UncatchableError::ApResultNotCorrespondToInstr;

    let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
    match (&merger_ap_result, &instr.result) {
        (MergerApResult::Scalar, ast::ApResult::Scalar(_)) => Ok(merger_ap_result),
        (MergerApResult::Stream(_), ast::ApResult::Stream(_)) => Ok(merger_ap_result),
        (MergerApResult::Empty, _) => Ok(merger_ap_result),
        _ => Err(ApResultNotCorrespondToInstr(merger_ap_result.clone()).into()),
    }
}

fn handle_ap<'ctx>(
    ap_result_type: &ast::ApResult<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<trace::ApResult> {
    match ap_result_type {
        ast::ApResult::Scalar(scalar) => {
            exec_ctx.scalars.set_scalar_value(scalar.name, result)?;
            Ok(trace::ApResult::scalar())
        }
        ast::ApResult::Stream(stream) => {
            let generation = ap_result_to_generation(merger_ap_result)?;
            let generation = exec_ctx
                .streams
                .add_stream_value(result, generation, stream.name, stream.position)?;
            Ok(trace::ApResult::stream(generation))
        }
    }
}
