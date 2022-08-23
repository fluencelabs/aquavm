/*
 * Copyright 2022 Fluence Labs Limited
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
use crate::merger::errors::CanonResultError;

use bimap::BiHashMap;

const EXPECTED_STATE_NAME: &str = "canon";

#[derive(Debug, Clone)]
pub enum MergerCanonResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    /// Positions correspond to a new data trace.
    CanonResult { stream_elements_pos: Vec<TracePos> },
}

pub(crate) fn try_merge_next_state_as_canon(data_keeper: &mut DataKeeper) -> MergeResult<MergerCanonResult> {
    use ExecutedState::Canon;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Canon(prev_canon)), Some(Canon(current_canon))) => {
            prepare_both_canon_result(&prev_canon, &current_canon, data_keeper)
        }
        (Some(Canon(prev_canon)), None) => prepare_single_canon_result(&prev_canon, &data_keeper.new_to_prev_pos),
        (None, Some(Canon(current_canon))) => {
            prepare_single_canon_result(&current_canon, &data_keeper.new_to_current_pos)
        }
        (None, None) => Ok(MergerCanonResult::Empty),
        (prev_state, current_state) => Err(MergeError::incompatible_states(
            prev_state,
            current_state,
            EXPECTED_STATE_NAME,
        )),
    }
}

fn prepare_both_canon_result(
    prev_canon_result: &CanonResult,
    current_canon_result: &CanonResult,
    data_keeper: &DataKeeper,
) -> MergeResult<MergerCanonResult> {
    check_canon_results(prev_canon_result, current_canon_result, data_keeper)
        .map_err(MergeError::IncorrectCanonResult)?;
    prepare_single_canon_result(prev_canon_result, &data_keeper.new_to_prev_pos)
}

fn prepare_single_canon_result(
    canon_result: &CanonResult,
    new_to_other_pos: &BiHashMap<TracePos, TracePos>,
) -> MergeResult<MergerCanonResult> {
    let new_positions = canon_result
        .stream_elements_pos
        .iter()
        .map(|pos| {
            new_to_other_pos
                .get_by_right(pos)
                .cloned()
                .ok_or_else(|| CanonResultError::not_met_position(canon_result.clone(), *pos))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MergerCanonResult::CanonResult {
        stream_elements_pos: new_positions,
    })
}

fn check_canon_results(
    prev_canon_result: &CanonResult,
    current_canon_result: &CanonResult,
    data_keeper: &DataKeeper,
) -> Result<(), CanonResultError> {
    if prev_canon_result.stream_elements_pos.len() != current_canon_result.stream_elements_pos.len() {
        return Err(CanonResultError::different_lens(
            prev_canon_result.clone(),
            current_canon_result.clone(),
        ));
    }

    let prev_slider = data_keeper.prev_slider();
    let current_slider = data_keeper.current_slider();
    for (position, (prev_idx, current_idx)) in prev_canon_result
        .stream_elements_pos
        .iter()
        .zip(current_canon_result.stream_elements_pos.iter())
        .enumerate()
    {
        let prev_state = prev_slider.state_at_position(*prev_idx);
        let current_state = current_slider.state_at_position(*current_idx);

        match (prev_state, current_state) {
            (Some(ExecutedState::Call(prev_call_result)), Some(ExecutedState::Call(current_call_result)))
                if prev_call_result == current_call_result =>
            {
                continue;
            }
            (Some(ExecutedState::Ap(prev_ap_result)), Some(ExecutedState::Ap(current_ap_result)))
                if prev_ap_result == current_ap_result =>
            {
                continue;
            }
            _ => {
                return Err(CanonResultError::incompatible_state(
                    prev_canon_result.clone(),
                    current_canon_result.clone(),
                    prev_state.cloned(),
                    current_state.cloned(),
                    position,
                ))
            }
        }
    }

    Ok(())
}
