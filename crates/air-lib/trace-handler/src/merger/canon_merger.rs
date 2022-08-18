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

const EXPECTED_STATE_NAME: &str = "canon";

#[derive(Debug, Clone)]
pub enum MergerCanonResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CanonResult { stream_elements_pos: Vec<TracePos> },
}

pub(crate) fn try_merge_next_state_as_canon(data_keeper: &mut DataKeeper) -> MergeResult<MergerCanonResult> {
    use ExecutedState::Canon;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Canon(prev_canon)), Some(Canon(current_canon))) => prepare_canon_result(&prev_canon, &current_canon),
        (Some(Canon(canon)), None) | (None, Some(Canon(canon))) => Ok(MergerCanonResult::CanonResult {
            stream_elements_pos: canon.stream_elements_pos,
        }),
        (None, None) => Ok(MergerCanonResult::Empty),
        (prev_state, current_state) => Err(MergeError::incompatible_states(
            prev_state,
            current_state,
            EXPECTED_STATE_NAME,
        )),
    }
}

fn prepare_canon_result(
    prev_canon_result: &CanonResult,
    current_canon_result: &CanonResult,
) -> MergeResult<MergerCanonResult> {
    use crate::merger::errors::CanonResultError;

    if prev_canon_result.len() != current_canon_result.len() {
        return Err(MergeError::IncorrectCanonResult(
            CanonResultError::CanonResultsIncompatible {
                prev_canon_result: prev_canon_result.clone(),
                current_canon_result: current_canon_result.clone(),
            },
        ));
    }

    Ok(MergerCanonResult::CanonResult {
        stream_elements_pos: prev_canon_result.stream_elements_pos.clone(),
    })
}
