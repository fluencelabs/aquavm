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
use crate::trace_to_exec_err;
use crate::JValue;
use crate::SecurityTetraplet;
use apply_to_arguments::*;
use utils::*;

use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::MergerApResult;

use std::cell::RefCell;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let should_touch_trace = should_touch_trace(self);

        let merger_ap_result = if should_touch_trace {
            let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), self)?;
            try_match_trace_to_instr(&merger_ap_result, self)?;
            merger_ap_result
        } else {
            MergerApResult::Empty
        };

        let result = apply_to_arg(&self.argument, exec_ctx, trace_ctx, should_touch_trace)?;
        save_result(&self.result, &merger_ap_result, result, exec_ctx)?;

        if should_touch_trace {
            // if generations are empty, then this ap instruction operates only with scalars and data
            // shouldn't be updated
            let final_ap_result = to_ap_result(&merger_ap_result, self, exec_ctx);
            trace_ctx.meet_ap_end(final_ap_result);
        }

        Ok(())
    }
}

fn save_result<'ctx>(
    ap_result_type: &ast::Variable<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    use ast::Variable::*;

    match ap_result_type {
        Scalar(scalar) => exec_ctx.scalars.set_value(scalar.name, result).map(|_| ()),
        Stream(stream) => {
            let generation = ap_result_to_generation(merger_ap_result);
            exec_ctx
                .streams
                .add_stream_value(result, generation, stream.name, stream.position)
                .map(|_| ())
        }
    }
}

/// This function is intended to check whether a Ap instruction should produce
/// a new state in data.
fn should_touch_trace(ap: &Ap<'_>) -> bool {
    matches!(ap.result, ast::Variable::Stream(_))
}
