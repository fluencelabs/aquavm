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

mod utils;

use super::*;
use crate::TracePos;
use utils::*;

const EXPECTED_STATE_NAME: &str = "call";

#[derive(Debug, Clone)]
pub enum MergerCallResult {
    /// There is no corresponding state in a trace for this call.
    NotMet,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    Met(MetCallResult),
}

#[derive(Debug, Clone)]
pub struct MetCallResult {
    pub result: CallResult,
    pub trace_pos: TracePos,
    pub source: ValueSource,
}

pub(crate) fn try_merge_next_state_as_call(data_keeper: &mut DataKeeper) -> MergeResult<MergerCallResult> {
    use ExecutedState::Call;
    use PreparationScheme::*;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => {
            let (merged_call, scheme) = merge_call_results(prev_call, current_call)?;
            Ok(prepare_call_result(merged_call, scheme, data_keeper))
        }
        (None, Some(Call(current_call))) => Ok(prepare_call_result(current_call, Current, data_keeper)),
        (Some(Call(prev_call)), None) => Ok(prepare_call_result(prev_call, Previous, data_keeper)),
        (None, None) => Ok(MergerCallResult::NotMet),
        (prev_state, current_state) => Err(MergeError::incompatible_states(
            prev_state,
            current_state,
            EXPECTED_STATE_NAME,
        )),
    }
}

fn merge_call_results(prev_call: CallResult, current_call: CallResult) -> MergeResult<(CallResult, PreparationScheme)> {
    use CallResult::*;
    use PreparationScheme::*;

    let (merged_state, scheme) = match (prev_call, current_call) {
        (prev @ Failed(..), current @ Failed(..)) => {
            check_equal(&prev, &current)?;
            (prev, Previous)
        }
        (RequestSentBy(_), current @ Failed(..)) => (current, Current),
        (prev @ Failed(..), RequestSentBy(_)) => (prev, Previous),
        // senders shouldn't be checked for equality, for more info please look at
        // github.com/fluencelabs/aquavm/issues/137
        (previous @ RequestSentBy(_), RequestSentBy(_)) => (previous, Previous),
        (RequestSentBy(_), current @ Executed(_)) => (current, Current),
        (previous @ Executed(..), RequestSentBy(_)) => (previous, Previous),
        (Executed(prev_value), Executed(current_value)) => (merge_executed(prev_value, current_value)?, Both),
        (prev_call, current_call) => return Err(CallResultError::incompatible_calls(prev_call, current_call)),
    };

    Ok((merged_state, scheme))
}

pub(super) fn prepare_call_result(
    call_result: CallResult,
    scheme: PreparationScheme,
    data_keeper: &mut DataKeeper,
) -> MergerCallResult {
    let trace_pos = data_keeper.result_trace_next_pos();
    prepare_positions_mapping(scheme, data_keeper);

    let met_result = MetCallResult::new(call_result, trace_pos, scheme.into());
    MergerCallResult::Met(met_result)
}

impl From<PreparationScheme> for ValueSource {
    fn from(scheme: PreparationScheme) -> Self {
        match scheme {
            PreparationScheme::Previous | PreparationScheme::Both => ValueSource::PreviousData,
            PreparationScheme::Current => ValueSource::CurrentData,
        }
    }
}

impl MetCallResult {
    pub fn new(result: CallResult, trace_pos: TracePos, source: ValueSource) -> Self {
        Self {
            result,
            trace_pos,
            source,
        }
    }
}
