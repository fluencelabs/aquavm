/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::*;
use crate::merger::errors::CanonResultError;

const EXPECTED_STATE_NAME: &str = "canon";

#[derive(Debug, Clone)]
pub enum MergerCanonResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CanonResult(CanonResult),
}

pub(crate) fn try_merge_next_state_as_canon(data_keeper: &mut DataKeeper) -> MergeResult<MergerCanonResult> {
    use ExecutedState::Canon;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Canon(prev_canon)), Some(Canon(current_canon))) => prepare_both_canon_result(prev_canon, current_canon),
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
    current_canon_result: CanonResult,
) -> MergeResult<MergerCanonResult> {
    let canon_result =
        merge_canon_results(prev_canon_result, current_canon_result).map_err(MergeError::IncorrectCanonResult)?;
    prepare_single_canon_result(canon_result)
}

fn prepare_single_canon_result(canon_result: CanonResult) -> MergeResult<MergerCanonResult> {
    let merger_result = MergerCanonResult::CanonResult(canon_result);
    Ok(merger_result)
}

fn merge_canon_results(
    prev_canon_result: CanonResult,
    current_canon_result: CanonResult,
) -> Result<CanonResult, CanonResultError> {
    use CanonResult::*;

    match (&prev_canon_result, &current_canon_result) {
        (Executed(prev), Executed(cur)) if prev != cur => Err(CanonResultError::incompatible_state(
            prev_canon_result,
            current_canon_result,
        )),
        (RequestSentBy(_), Executed(_)) => Ok(current_canon_result),

        (RequestSentBy(_), RequestSentBy(_)) | (Executed(_), RequestSentBy(_)) | (Executed(_), Executed(_)) => {
            Ok(prev_canon_result)
        }
    }
}
