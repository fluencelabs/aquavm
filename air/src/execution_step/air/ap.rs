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

mod utils;

use super::call::call_result_setter::set_scalar_result;
use super::call::call_result_setter::set_stream_result;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::air::ResolvedCallResult;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::trace_handler::MergerApResult;
use crate::execution_step::utils::apply_json_path;
use crate::SecurityTetraplet;
use utils::*;

use air_parser::ast::Ap;
use air_parser::ast::ApSource;
use air_parser::ast::AstVariable;

use std::cell::RefCell;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let merger_ap_result = trace_ctx.meet_ap_start()?;
        try_match_result_to_instr(&merger_ap_result, self)?;

        let result = apply(&self.src, &merger_ap_result, exec_ctx, trace_ctx)?;
        save_result(&self.dst, &merger_ap_result, result, exec_ctx)?;

        let ap_result = to_ap_result(&merger_ap_result, self, exec_ctx);
        trace_ctx.meet_ap_end(ap_result);

        Ok(())
    }
}

fn apply(
    ap_source: &ApSource<'_>,
    merger_ap_result: &MergerApResult,
    exec_ctx: &ExecutionCtx<'_>,
    trace_ctx: &TraceHandler,
) -> ExecutionResult<ResolvedCallResult> {
    let generation = ap_result_to_generation(&merger_ap_result, ApInstrPosition::Source);
    let variable = Variable::from_ast_with_generation(&ap_source.variable, generation);
    let (jvalue, mut tetraplets) = apply_json_path(variable, &ap_source.path, ap_source.should_flatten, exec_ctx)?;

    let tetraplet = tetraplets
        .pop()
        .unwrap_or_else(|| Rc::new(RefCell::new(SecurityTetraplet::default())));
    let result = ResolvedCallResult::new(Rc::new(jvalue), tetraplet, trace_ctx.trace_pos());

    Ok(result)
}

fn save_result<'ctx>(
    destination: &AstVariable<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ResolvedCallResult,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    match destination {
        AstVariable::Scalar(name) => set_scalar_result(result, name, exec_ctx),
        AstVariable::Stream(name) => {
            let generation = ap_result_to_generation(&merger_ap_result, ApInstrPosition::Destination);
            set_stream_result(result, generation, name.to_string(), exec_ctx).map(|_| ())
        }
    }
}
