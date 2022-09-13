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

const EXPECTED_STATE_NAME: &str = "ap";

#[derive(Debug, Clone)]
pub enum MergerApResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    Stream(StreamApResult),

    Scalar,
}

#[derive(Debug, Clone)]
pub enum StreamApResult {
    PrevState(u32),
    CurrentState,
}

pub(crate) fn try_merge_next_state_as_ap(data_keeper: &mut DataKeeper) -> MergeResult<MergerApResult> {
    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    if let Some(stream_ap_result) = try_merge_as_stream_ap(&prev_state, &current_state, data_keeper) {
        return stream_ap_result;
    };

    if let Some(scalar_ap_result) = try_merge_as_scalar_ap(&prev_state, &current_state, data_keeper) {
        return Ok(scalar_ap_result);
    };

    Err(MergeError::incompatible_states(
        prev_state,
        current_state,
        EXPECTED_STATE_NAME,
    ))
}

fn try_merge_as_stream_ap(
    prev_state: &Option<ExecutedState>,
    current_state: &Option<ExecutedState>,
    data_keeper: &mut DataKeeper,
) -> Option<MergeResult<MergerApResult>> {
    use ExecutedState::Ap;
    use PreparationScheme::*;

    let result = match (prev_state, current_state) {
        (Some(Ap(prev_ap)), Some(Ap(current_ap))) if prev_ap.is_stream() && current_ap.is_stream() => {
            prepare_prev_stream_result(prev_ap, Both, data_keeper)
        }
        (Some(Ap(prev_ap)), None) if prev_ap.is_stream() => prepare_prev_stream_result(prev_ap, Previous, data_keeper),
        // check that current state is Ap, but it's impossible to use it, because prev_data
        // could not have streams with such generations
        (None, Some(Ap(current_ap))) if current_ap.is_stream() => prepare_current_stream_result(data_keeper),
        (None, None) => Ok(MergerApResult::Empty),
        _ => return None,
    };

    Some(result)
}

fn prepare_prev_stream_result(
    prev_ap_result: &ApResult,
    scheme: PreparationScheme,
    data_keeper: &mut DataKeeper,
) -> MergeResult<MergerApResult> {
    prepare_positions_mapping(scheme, data_keeper);
    // it should be checked on the call site that res_generation contains exactly one state
    let res_generation = prev_ap_result.res_generations[0];

    let ap_result = MergerApResult::Stream(StreamApResult::PrevState(res_generation));
    Ok(ap_result)
}

fn prepare_current_stream_result(data_keeper: &mut DataKeeper) -> MergeResult<MergerApResult> {
    prepare_positions_mapping(PreparationScheme::Current, data_keeper);
    Ok(MergerApResult::Stream(StreamApResult::CurrentState))
}

fn try_merge_as_scalar_ap(
    prev_state: &Option<ExecutedState>,
    current_state: &Option<ExecutedState>,
    data_keeper: &mut DataKeeper,
) -> Option<MergerApResult> {
    use ExecutedState::Ap;
    use PreparationScheme::*;

    let scalar_result = match (prev_state, current_state) {
        // ApScalar is an empty state and it's needed only to check that states are matched
        (Some(Ap(prev_state)), Some(Ap(current_state))) if prev_state.is_scalar() && current_state.is_scalar() => {
            prepare_scalar_result(Both, data_keeper)
        }
        (Some(Ap(prev_state)), None) if prev_state.is_scalar() => prepare_scalar_result(Previous, data_keeper),
        (None, Some(Ap(current_state))) if current_state.is_scalar() => prepare_scalar_result(Current, data_keeper),
        (None, None) => MergerApResult::Empty,
        _ => return None,
    };

    Some(scalar_result)
}

fn prepare_scalar_result(scheme: PreparationScheme, data_keeper: &mut DataKeeper) -> MergerApResult {
    prepare_positions_mapping(scheme, data_keeper);
    MergerApResult::Scalar
}
