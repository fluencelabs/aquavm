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
use MergeError::IncompatibleCallResults;
use MergeError::IncompatibleExecutedStates;

use air_parser::ast::CallOutputValue;

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

    let prev_state = data_keeper.prev_ctx.slider.next_state();
    let current_state = data_keeper.current_ctx.slider.next_state();
    let value_type = ValueType::from_output_value(output_value);

    let (prev_call, current_call) = match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => (prev_call, current_call),
        // this special case is needed to merge stream generation in a right way
        (None, Some(Call(call @ CallResult::Executed(..)))) => {
            let call_result = merge_current_executed(call, value_type, data_keeper)?;
            return Ok(MergerCallResult::call_result(call_result, data_keeper));
        }
        (None, Some(Call(current_call @ _))) => return Ok(MergerCallResult::call_result(current_call, data_keeper)),
        (Some(Call(prev_call @ _)), None) => return Ok(MergerCallResult::call_result(prev_call, data_keeper)),
        (None, None) => return Ok(MergerCallResult::Empty),
        (Some(prev_state), Some(current_state)) => return Err(IncompatibleExecutedStates(prev_state, current_state)),
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

fn merge_executed(
    prev_result: CallResult,
    current_result: CallResult,
    value_type: ValueType<'_>,
) -> MergeResult<CallResult> {
    match value_type {
        ValueType::Stream(name) => {
            // values from streams could have different generations and it's ok
            check_stream_equal(&prev_result, &current_result)?;
            Ok(prev_result)
        }
        ValueType::Scalar => {
            check_equal(&prev_result, &current_result)?;
            Ok(prev_result)
        }
    }
}

/// Merging of value from only current data to a stream is a something special, because it's
/// needed to choose generation not from current data, but a maximum from streams on a current peer.
/// Maximum versions are tracked in data in a special field called streams.
fn merge_current_executed(
    current_result: CallResult,
    value_type: ValueType<'_>,
    data_keeper: &DataKeeper,
) -> MergeResult<CallResult> {
    match value_type {
        ValueType::Stream(stream_name) => {
            let generation = data_keeper.prev_ctx.stream_generation(stream_name)?;
            let value = match current_result {
                CallResult::Executed(value, _) => value,
                _ => unreachable!(
                    "this function should be called only when it's checked that call results are executed states"
                ),
            };
            let call_result = CallResult::Executed(value, generation);
            Ok(call_result)
        }
        ValueType::Scalar => Ok(current_result),
    }
}

fn check_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
    if prev_result != current_result {
        Err(IncompatibleCallResults(prev_result.clone(), current_result.clone()))
    } else {
        Ok(())
    }
}

fn check_stream_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
    match (prev_result, current_result) {
        (CallResult::Executed(prev_value, _), CallResult::Executed(current_value, _)) => {
            if prev_value != current_value {
                Err(IncompatibleCallResults(prev_result.clone(), current_result.clone()))
            } else {
                Ok(())
            }
        }
        _ => {
            unreachable!("this function should be called only when it's checked that call results are executed states")
        }
    }
}

impl MergerCallResult {
    pub(self) fn call_result(value: CallResult, data_keeper: &DataKeeper) -> Self {
        Self::CallResult {
            value,
            trace_pos: data_keeper.result_trace.len(),
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
