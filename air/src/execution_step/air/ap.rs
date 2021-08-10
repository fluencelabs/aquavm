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

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::trace_handler::MergerApResult;
use crate::execution_step::utils::apply_json_path;
use crate::execution_step::utils::variable_name;
use crate::execution_step::Generation;

use air_parser::ast::Ap;
use air_parser::ast::CallInstrArgValue;
use air_parser::ast::AstVariable;

use crate::execution_step::air::ResolvedCallResult;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let ap_result = trace_ctx.meet_ap()?;
        try_match_result_to_instr(&ap_result, self)?;

        let src = &self.src;
        let (value, tetraplet) = apply_json_path(&src.variable, &src.path, src.should_flatten, exec_ctx)?;

        let dst_variable = &self.dst;
        match exec_ctx.streams.get(variable_name) {
            Some(stream) => {
                let resolved_call = ResolvedCallResult::new(Rc::new(jvalue), tetraplet[0].triplet.clone(), 0);
                stream.borrow_mut().add_value(resolved_call, Generation::Last)?;
            }
            _ => unreachable!("return a error"),
        };

        Ok(())
    }
}

fn try_match_result_to_instr(merger_ap_result: &MergerApResult, ap: &Ap<'_>) -> ExecutionResult<()> {
    fn match_position(
        variable: &AstVariable<'_>,
        generation: Option<u32>,
        ap_result: &MergerApResult,
    ) -> ExecutionResult<()> {
        match (variable, generation) {
            (AstVariable::Stream(_), Some(_)) => Ok(()),
            (AstVariable::Scalar(_), None) => Ok(()),
            _ => return crate::exec_err!(ExecutionError::ApResultNotCorrespondToInstr(ap_result.clone())),
        }
    }

    let (src_generation, dst_generation) = match merger_ap_result {
        MergerApResult::ApResult {
            src_generation,
            dst_generation,
        } => (*src_generation, *dst_generation),
        MergerApResult::Empty => return Ok(()),
    };

    match_position(&ap.src.variable, src_generation, merger_ap_result)?;
    match_position(&ap.dst, dst_generation, merger_ap_result)
}
