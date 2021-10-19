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

use super::call::call_result_setter::set_scalar_result;
use super::call::call_result_setter::set_stream_result;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::air::ResolvedCallResult;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::utils::apply_lambda;
use crate::trace_to_exec_err;
use crate::JValue;
use crate::SecurityTetraplet;
use apply_to_arguments::*;
use utils::*;

use air_parser::ast::ApArgument;
use air_parser::ast::AstVariable;
use air_parser::ast::VariableWithLambda;
use air_parser::ast::{Ap, LastErrorPath};
use air_trace_handler::MergerApResult;

use std::cell::RefCell;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let should_touch_trace = should_touch_trace(self);

        let merger_ap_result = if should_touch_trace {
            let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start())?;
            try_match_result_to_instr(&merger_ap_result, self)?;
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
    ap_result_type: &AstVariable<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ResolvedCallResult,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    match ap_result_type {
        AstVariable::Scalar(name) => set_scalar_result(result, name, exec_ctx),
        AstVariable::Stream(name) => {
            let generation = ap_result_to_generation(merger_ap_result);
            set_stream_result(result, generation, name.to_string(), exec_ctx).map(|_| ())
        }
    }
}

fn should_touch_trace(ap: &Ap<'_>) -> bool {
    match (&ap.argument, &ap.result) {
        (_, AstVariable::Stream(_)) => true,
        (ApArgument::VariableWithLambda(vl), _) => match &vl.variable {
            AstVariable::Scalar(_) => false,
            AstVariable::Stream(_) => true,
        },
        _ => false,
    }
}
