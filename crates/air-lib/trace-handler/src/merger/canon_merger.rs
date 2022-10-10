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

use serde_json::Value as JValue;

const EXPECTED_STATE_NAME: &str = "canon";

#[derive(Debug, Clone)]
pub enum MergerCanonResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CanonResult { canonicalized_element: JValue },
}

pub(crate) fn try_merge_next_state_as_canon(data_keeper: &mut DataKeeper) -> MergeResult<MergerCanonResult> {
    use ExecutedState::Canon;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Canon(prev_canon)), Some(Canon(current_canon))) => prepare_both_canon_result(prev_canon, &current_canon),
        (Some(Canon(prev_canon)), None) => prepare_single_canon_result(prev_canon),
        (None, Some(Canon(current_canon))) => prepare_single_canon_result(current_canon),
        (None, None) => Ok(MergerCanonResult::Empty),
        (prev_state, current_state) => Err(MergeError::incompatible_states(
            prev_state,
            current_state,
            EXPECTED_STATE_NAME,
        )),
    }
}

fn prepare_both_canon_result(
    prev_canon_result: CanonResult,
    current_canon_result: &CanonResult,
) -> MergeResult<MergerCanonResult> {
    check_canon_results(&prev_canon_result, current_canon_result).map_err(MergeError::IncorrectCanonResult)?;
    prepare_single_canon_result(prev_canon_result)
}

fn prepare_single_canon_result(canon_result: CanonResult) -> MergeResult<MergerCanonResult> {
    Ok(MergerCanonResult::CanonResult {
        canonicalized_element: canon_result.canonicalized_element,
    })
}

fn check_canon_results(
    prev_canon_result: &CanonResult,
    current_canon_result: &CanonResult,
) -> Result<(), CanonResultError> {
    if prev_canon_result.canonicalized_element != current_canon_result.canonicalized_element {
        return Err(CanonResultError::incompatible_state(
            prev_canon_result.clone(),
            current_canon_result.clone(),
        ));
    }

    Ok(())
}
