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

use super::ExecutionResult;
use crate::execution_step::Generation;

use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::merger::MergerApResult;

pub(super) fn ap_result_to_generation(ap_result: &MergerApResult) -> Generation {
    use air_trace_handler::merger::ValueSource;

    let met_result = match ap_result {
        MergerApResult::NotMet => return Generation::Last,
        MergerApResult::Met(met_result) => met_result,
    };

    match met_result.value_source {
        ValueSource::PreviousData => Generation::Nth(met_result.generation),
        ValueSource::CurrentData => Generation::Last,
    }
}

pub(super) fn try_match_trace_to_instr(merger_ap_result: &MergerApResult, instr: &Ap<'_>) -> ExecutionResult<()> {
    use crate::execution_step::UncatchableError::ApResultNotCorrespondToInstr;
    use ast::ApResult;

    match (&instr.result, merger_ap_result) {
        (ApResult::Stream(_), MergerApResult::Met(_)) => Ok(()),
        (_, MergerApResult::NotMet) => Ok(()),
        _ => Err(ApResultNotCorrespondToInstr(merger_ap_result.clone()).into()),
    }
}
