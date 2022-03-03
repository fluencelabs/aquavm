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
use crate::execution_step::Generation;

use air_interpreter_data::ApResult;
use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::MergerApResult;

pub(super) fn ap_result_to_generation(ap_result: &MergerApResult) -> Generation {
    match ap_result {
        MergerApResult::Empty => Generation::Last,
        MergerApResult::ApResult { res_generation, .. } => Generation::from_option(*res_generation),
    }
}

pub(super) fn try_match_trace_to_instr(merger_ap_result: &MergerApResult, instr: &Ap<'_>) -> ExecutionResult<()> {
    let res_generation = match merger_ap_result {
        MergerApResult::ApResult { res_generation } => *res_generation,
        MergerApResult::Empty => return Ok(()),
    };

    match_position_variable(&instr.result, res_generation, merger_ap_result)
}

fn match_position_variable(
    variable: &ast::Variable<'_>,
    generation: Option<u32>,
    ap_result: &MergerApResult,
) -> ExecutionResult<()> {
    use crate::execution_step::UncatchableError::ApResultNotCorrespondToInstr;
    use ast::Variable::*;

    match (variable, generation) {
        (Stream(_), Some(_)) => Ok(()),
        (Scalar(_), None) => Ok(()),
        _ => Err(ApResultNotCorrespondToInstr(ap_result.clone()).into()),
    }
}

pub(super) fn to_ap_result(merger_ap_result: &MergerApResult, instr: &Ap<'_>, exec_ctx: &ExecutionCtx<'_>) -> ApResult {
    if let MergerApResult::ApResult { res_generation } = merger_ap_result {
        let res_generation = option_to_vec(*res_generation);

        return ApResult::new(res_generation);
    }

    let res_generation = variable_to_generations(&instr.result, exec_ctx);
    ApResult::new(res_generation)
}

fn option_to_vec(value: Option<u32>) -> Vec<u32> {
    match value {
        Some(value) => vec![value],
        None => vec![],
    }
}

fn variable_to_generations(variable: &ast::Variable<'_>, exec_ctx: &ExecutionCtx<'_>) -> Vec<u32> {
    use ast::Variable::*;

    match variable {
        Scalar(_) => vec![],
        Stream(stream) => {
            // unwrap here is safe because this function will be called only
            // when this stream's been created
            let stream = exec_ctx.streams.get(stream.name, stream.position).unwrap();
            let generation = match stream.generations_count() {
                0 => 0,
                n => n - 1,
            };

            vec![generation as u32]
        }
    }
}
