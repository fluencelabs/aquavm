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

use super::*;

#[derive(Debug, Clone)]
pub(crate) enum MergerApResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    ApResult {
        src_generation: Option<u32>,
        dst_generation: Option<u32>,
    },
}

pub(crate) fn try_merge_next_state_as_ap(data_keeper: &mut DataKeeper) -> MergeResult<MergerApResult> {
    use ExecutedState::Ap;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    let ap = match (prev_state, current_state) {
        (Some(Ap(prev_ap)), _) => prev_ap,
        // this special case is needed to merge stream generation in a right way
        (None, Some(Ap(current_ap))) => current_ap,
        (None, None) => return Ok(MergerApResult::Empty),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "call")),
    };

    to_merger_result(ap)
}

macro_rules! to_maybe_generation {
    ($ap_result:ident, $generations:expr, $error_ty:ident) => {
        match $generations.len() {
            0 => None,
            1 => Some($generations[0]),
            _ => {
                let ap_error = super::ApResultError::$error_ty($ap_result);
                return Err(super::MergeError::IncorrectApResult(ap_error));
            }
        }
    };
}

fn to_merger_result(ap_result: ApResult) -> MergeResult<MergerApResult> {
    const MAXIMUM_GENERATIONS_COUNT: usize = 1;

    let src_generation = to_maybe_generation!(ap_result, &ap_result.src_generations, TooManySrcGenerations);
    let dst_generation = to_maybe_generation!(ap_result, &ap_result.dst_generations, TooManyDstGenerations);

    let ap_result = MergerApResult::ApResult {
        src_generation,
        dst_generation,
    };

    Ok(ap_result)
}
