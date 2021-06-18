/*
 * Copyright 2020 Fluence Labs Limited
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
use MergeError::IncompatibleCallResults;
use MergeError::IncompatibleExecutedStates;

use air_parser::ast::CallOutputValue;

pub(crate) enum MergerCallResult {
    /// There is no corresponding state in trace for this call.
    Empty,
    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CallResult(CallResult),
}

pub(crate) fn try_merge_next_state_as_call(
    data_keeper: &mut DataKeeper,
    _output_value: &CallOutputValue,
) -> MergeResult<MergerCallResult> {
    use ExecutedState::Call;
    use MergerCallResult::*;

    let prev_state = data_keeper.prev_ctx.slider.next_state();
    let current_state = data_keeper.current_ctx.slider.next_state();

    let (prev_call, current_call) = match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => (prev_call, current_call),
        (None, Some(Call(current_call @ _))) => return Ok(CallResult(current_call)),
        (Some(Call(prev_call @ _)), None) => return Ok(CallResult(prev_call)),
        (None, None) => return Ok(Empty),
        (Some(prev_state), Some(current_state)) => return Err(IncompatibleExecutedStates(prev_state, current_state)),
    };

    let merged_call = merge_call_result(prev_call, current_call)?;
    Ok(CallResult(merged_call))
}

fn merge_call_result(prev_call: CallResult, current_call: CallResult) -> MergeResult<CallResult> {
    use CallResult::*;
    use ExecutedState::Call;

    let merged_state = match (&prev_call, &current_call) {
        (Call(CallServiceFailed(..)), Call(CallServiceFailed(..))) => {
            check_for_equal(&prev_call, &current_call)?;
            current_call
        }
        (Call(RequestSentBy(_)), Call(CallServiceFailed(..))) => current_call,
        (Call(CallServiceFailed(..)), Call(RequestSentBy(_))) => prev_call,
        (Call(RequestSentBy(_)), Call(RequestSentBy(_))) => {
            check_for_equal(&prev_call, &current_call)?;
            prev_call
        }
        (Call(RequestSentBy(_)), Call(Executed(_))) => current_call,
        (Call(Executed(_)), Call(RequestSentBy(_))) => prev_call,
        (Call(Executed(_)), Call(Executed(_))) => {
            check_for_equal(&prev_call, &current_call)?;
            prev_call
        }
        (Executed(_), CallServiceFailed(..)) | (CallServiceFailed(..), Executed(_)) => {
            return Err(IncompatibleCallResults(prev_call.clone(), current_call.clone()))
        }
    };

    Ok(merged_state)
}

fn check_for_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
    if prev_result != current_result {
        Err(IncompatibleCallResults(prev_result.clone(), current_result.clone()))
    } else {
        Ok(())
    }
}
