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

mod utils;

use super::*;
use utils::*;
use MergeError::IncompatibleCallResults;

use air_parser::ast::CallOutputValue;

#[derive(Debug, Clone)]
pub(crate) enum MergerCallResult {
    /// There is no corresponding state in a trace for this call.
    Empty,

    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CallResult { value: CallResult, trace_pos: usize },
}

pub(crate) fn try_merge_next_state_as_call(
    data_keeper: &mut DataKeeper,
    output_value: &CallOutputValue<'_>,
) -> MergeResult<MergerCallResult> {
    use ExecutedState::Call;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();
    let value_type = ValueType::from_output_value(output_value);

    let (prev_call, current_call) = match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => (prev_call, current_call),
        // this special case is needed to merge stream generation in a right way
        (None, Some(Call(call @ CallResult::Executed(..)))) => {
            let call_result = merge_current_executed(call, value_type, data_keeper)?;
            return Ok(MergerCallResult::call_result(call_result, data_keeper));
        }
        (None, Some(Call(current_call))) => return Ok(MergerCallResult::call_result(current_call, data_keeper)),
        (Some(Call(prev_call)), None) => return Ok(MergerCallResult::call_result(prev_call, data_keeper)),
        (None, None) => return Ok(MergerCallResult::Empty),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "call")),
    };

    let merged_call = merge_call_result(prev_call, current_call, value_type, data_keeper)?;

    Ok(MergerCallResult::call_result(merged_call, data_keeper))
}

fn merge_call_result(
    prev_call: CallResult,
    current_call: CallResult,
    value_type: ValueType<'_>,
    data_keeper: &DataKeeper,
) -> MergeResult<CallResult> {
    use CallResult::*;

    let merged_state = match (&prev_call, &current_call) {
        (CallServiceFailed(..), CallServiceFailed(..)) => {
            check_equal(&prev_call, &current_call)?;
            current_call
        }
        (RequestSentBy(_), CallServiceFailed(..)) => current_call,
        (CallServiceFailed(..), RequestSentBy(_)) => prev_call,
        (RequestSentBy(_), RequestSentBy(_)) => {
            check_equal(&prev_call, &current_call)?;
            prev_call
        }
        // this special case is needed to merge stream generation in a right way
        (RequestSentBy(_), Executed(..)) => merge_current_executed(current_call, value_type, data_keeper)?,
        (Executed(..), RequestSentBy(_)) => prev_call,
        (Executed(..), Executed(..)) => merge_executed(prev_call, current_call, value_type)?,
        (Executed(..), CallServiceFailed(..)) | (CallServiceFailed(..), Executed(..)) => {
            return Err(IncompatibleCallResults(prev_call.clone(), current_call.clone()))
        }
    };

    Ok(merged_state)
}

impl MergerCallResult {
    pub(self) fn call_result(value: CallResult, data_keeper: &DataKeeper) -> Self {
        Self::CallResult {
            value,
            trace_pos: data_keeper.result_states_count(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ValueType<'i> {
    Scalar,
    Stream(&'i str),
}

impl<'i> ValueType<'i> {
    pub(self) fn from_output_value(output_value: &'i CallOutputValue<'_>) -> Self {
        use air_parser::ast::Variable;

        match output_value {
            CallOutputValue::Variable(Variable::Stream(stream_name)) => ValueType::Stream(stream_name),
            _ => ValueType::Scalar,
        }
    }
}
