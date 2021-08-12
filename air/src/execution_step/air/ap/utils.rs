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
use super::ExecutionResult;
use crate::execution_step::trace_handler::MergerApResult;
use crate::execution_step::Generation;

use air_interpreter_data::ApResult;
use air_parser::ast::AstVariable;
use air_parser::ast::{Ap, ApArgument};

#[derive(Clone, Copy, Debug)]
pub(super) enum ApInstrPosition {
    Source,
    Destination,
}

pub(super) fn ap_result_to_generation(ap_result: &MergerApResult, position: ApInstrPosition) -> Generation {
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

pub(super) fn try_match_result_to_instr(merger_ap_result: &MergerApResult, instr: &Ap<'_>) -> ExecutionResult<()> {
    let (src_generation, dst_generation) = match merger_ap_result {
        MergerApResult::ApResult {
            src_generation,
            dst_generation,
        } => (*src_generation, *dst_generation),
        MergerApResult::Empty => return Ok(()),
    };

    match_position_arg(&instr.argument, src_generation, merger_ap_result)?;
    match_position_variable(&instr.result, dst_generation, merger_ap_result)
}

fn match_position_arg(
    variable: &ApArgument<'_>,
    generation: Option<u32>,
    ap_result: &MergerApResult,
) -> ExecutionResult<()> {
    use crate::execution_step::ExecutionError::ApResultNotCorrespondToInstr;

    match (variable, generation) {
        (ApArgument::ScalarVariable(_), None) => Ok(()),
        (ApArgument::JsonPath(json_path), _) => match_position_variable(&json_path.variable, generation, ap_result),
        _ => return crate::exec_err!(ApResultNotCorrespondToInstr(ap_result.clone())),
    }
}

fn match_position_variable(
    variable: &AstVariable<'_>,
    generation: Option<u32>,
    ap_result: &MergerApResult,
) -> ExecutionResult<()> {
    use crate::execution_step::ExecutionError::ApResultNotCorrespondToInstr;

    match (variable, generation) {
        (AstVariable::Stream(_), Some(_)) => Ok(()),
        (AstVariable::Scalar(_), None) => Ok(()),
        _ => return crate::exec_err!(ApResultNotCorrespondToInstr(ap_result.clone())),
    }
}

pub(super) fn to_ap_result(merger_ap_result: &MergerApResult, instr: &Ap<'_>, exec_ctx: &ExecutionCtx<'_>) -> ApResult {
    if let MergerApResult::ApResult {
        src_generation,
        dst_generation,
    } = merger_ap_result
    {
        let src_generations = option_to_vec(*src_generation);
        let dst_generations = option_to_vec(*dst_generation);

        return ApResult::new(src_generations, dst_generations);
    }

    let src_generations = arg_to_generations(&instr.argument, exec_ctx);
    let dst_generations = variable_to_generations(&instr.result, exec_ctx);

    ApResult::new(src_generations, dst_generations)
}

fn option_to_vec(value: Option<u32>) -> Vec<u32> {
    match value {
        Some(value) => vec![value],
        None => vec![],
    }
}

fn arg_to_generations(arg: &ApArgument<'_>, exec_ctx: &ExecutionCtx<'_>) -> Vec<u32> {
    match arg {
        ApArgument::ScalarVariable(_) => return vec![],
        ApArgument::JsonPath(json_path) => variable_to_generations(&json_path.variable, exec_ctx),
    }
}

fn variable_to_generations(variable: &AstVariable<'_>, exec_ctx: &ExecutionCtx<'_>) -> Vec<u32> {
    match variable {
        AstVariable::Scalar(_) => vec![],
        AstVariable::Stream(name) => {
            // unwrap here is safe because this function will be called only
            // when this stream's been created
            let stream = exec_ctx.streams.get(*name).unwrap();
            let generation = match stream.borrow().generations_count() {
                0 => 0,
                n => n - 1,
            };

            vec![generation as u32]
        }
    }
}
