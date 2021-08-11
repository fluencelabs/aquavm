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

use super::call::call_result_setter::set_scalar_result;
use super::call::call_result_setter::set_stream_result;
use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::air::ResolvedCallResult;
use crate::execution_step::boxed_value::Variable;
use crate::execution_step::trace_handler::MergerApResult;
use crate::execution_step::utils::apply_json_path;
use crate::execution_step::Generation;

use air_parser::ast::Ap;
use air_parser::ast::AstVariable;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let merger_ap_result = trace_ctx.meet_ap_start()?;
        try_match_result_to_instr(&merger_ap_result, self)?;

        let src = &self.src;
        let generation = ap_result_to_generation(&merger_ap_result, ApInstrPosition::Source);
        let variable = Variable::from_ast_with_generation(&src.variable, generation);
        let (jvalue, tetraplet) = apply_json_path(variable, &src.path, src.should_flatten, exec_ctx)?;

        let result = ResolvedCallResult::new(Rc::new(jvalue), tetraplet[0].triplet.clone(), trace_ctx.trace_pos());
        match &self.dst {
            AstVariable::Scalar(name) => set_scalar_result(result, name, exec_ctx)?,
            AstVariable::Stream(name) => {
                let generation = ap_result_to_generation(&merger_ap_result, ApInstrPosition::Destination);
                set_stream_result(result, generation, name.to_string(), exec_ctx)?;
            }
        }

        let ap_result = to_ap_result(&merger_ap_result, self, exec_ctx);
        trace_ctx.meet_ap_end(ap_result);

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum ApInstrPosition {
    Source,
    Destination,
}

fn ap_result_to_generation(ap_result: &MergerApResult, position: ApInstrPosition) -> Generation {
    match (position, ap_result) {
        (_, MergerApResult::Empty) => Generation::Last,
        (ApInstrPosition::Source, MergerApResult::ApResult { src_generation, .. }) => {
            Generation::from_option(*src_generation)
        }
        (ApInstrPosition::Destination, MergerApResult::ApResult { dst_generation, .. }) => {
            Generation::from_option(*dst_generation)
        }
    }
}

fn try_match_result_to_instr(merger_ap_result: &MergerApResult, instr: &Ap<'_>) -> ExecutionResult<()> {
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

    match_position(&instr.src.variable, src_generation, merger_ap_result)?;
    match_position(&instr.dst, dst_generation, merger_ap_result)
}

use air_interpreter_data::ApResult;

fn to_ap_result(merger_ap_result: &MergerApResult, instr: &Ap<'_>, exec_ctx: &ExecutionCtx<'_>) -> ApResult {
    fn option_to_vec(value: Option<u32>) -> Vec<u32> {
        match value {
            Some(value) => vec![value],
            None => vec![],
        }
    }

    if let MergerApResult::ApResult {
        src_generation,
        dst_generation,
    } = merger_ap_result
    {
        let src_generations = option_to_vec(*src_generation);
        let dst_generations = option_to_vec(*dst_generation);

        return ApResult {
            src_generations,
            dst_generations,
        };
    }

    let src_generations = match instr.src.variable {
        AstVariable::Scalar(_) => vec![],
        AstVariable::Stream(name) => {
            let stream = exec_ctx.streams.get(name).unwrap();
            vec![stream.borrow().generations_count() as u32]
        }
    };

    let dst_generations = match instr.dst {
        AstVariable::Scalar(_) => vec![],
        AstVariable::Stream(name) => {
            let stream = exec_ctx.streams.get(name).unwrap();
            vec![stream.borrow().generations_count() as u32]
        }
    };

    ApResult {
        src_generations,
        dst_generations,
    }
}
