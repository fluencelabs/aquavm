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
use crate::TracePos;
use air_parser::ast::CallOutputValue;
use utils::*;

const EXPECTED_STATE_NAME: &str = "call";

#[derive(Debug, Clone)]
pub enum MergerCallResult {
    /// There is no corresponding state in a trace for this call.
    Empty,
    /// There was a state in at least one of the contexts. If there were two states in
    /// both contexts, they were successfully merged.
    CallResult {
        value: CallResult,
        trace_pos: TracePos,
        scheme: PreparationScheme,
    },
}

pub(crate) fn try_merge_next_state_as_call(
    data_keeper: &mut DataKeeper,
    output_value: &CallOutputValue<'_>,
) -> MergeResult<MergerCallResult> {
    use ExecutedState::Call;
    use PreparationScheme::*;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();
    let value_type = ValueType::from_output_value(output_value);

    let (prev_call, current_call) = match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => (prev_call, current_call),
        // this special case is needed to merge stream generation in a right way
        (None, Some(Call(CallResult::Executed(value)))) => {
            let call_result = merge_current_executed(value, value_type)?;
            return Ok(prepare_call_result(call_result, Current, data_keeper));
        }
        (None, Some(Call(current_call))) => return Ok(prepare_call_result(current_call, Current, data_keeper)),
        (Some(Call(prev_call)), None) => return Ok(prepare_call_result(prev_call, Previous, data_keeper)),
        (None, None) => return Ok(MergerCallResult::Empty),
        (prev_state, current_state) => {
            return Err(MergeError::incompatible_states(
                prev_state,
                current_state,
                EXPECTED_STATE_NAME,
            ))
        }
    };

    let (merged_call, scheme) = merge_call_result(prev_call, current_call, value_type)?;
    let call_result = prepare_call_result(merged_call, scheme, data_keeper);
    try_match_value_type(&call_result, value_type)?;

    Ok(call_result)
}

fn merge_call_result(
    prev_call: CallResult,
    current_call: CallResult,
    value_type: ValueType<'_>,
) -> MergeResult<(CallResult, PreparationScheme)> {
    use CallResult::*;
    use PreparationScheme::*;

    let (merged_state, scheme) = match (prev_call, current_call) {
        (prev @ CallServiceFailed(..), current @ CallServiceFailed(..)) => {
            check_equal(&prev, &current)?;
            (prev, Previous)
        }
        (RequestSentBy(_), current @ CallServiceFailed(..)) => (current, Current),
        (prev @ CallServiceFailed(..), RequestSentBy(_)) => (prev, Previous),
        // senders shouldn't be checked for equality, for more info please look at
        // github.com/fluencelabs/aquavm/issues/137
        (prev @ RequestSentBy(_), RequestSentBy(_)) => (prev, Previous),
        // this special case is needed to merge stream generation in a right way
        (RequestSentBy(_), Executed(value)) => (merge_current_executed(value, value_type)?, Current),
        (prev @ Executed(..), RequestSentBy(_)) => (prev, Previous),
        (Executed(prev_value), Executed(current_value)) => {
            (merge_executed(prev_value, current_value)?, Both)
        }
        (prev_call, current_call) => return Err(CallResultError::incompatible_calls(prev_call, current_call)),
    };

    Ok((merged_state, scheme))
}

pub(super) fn prepare_call_result(
    value: CallResult,
    scheme: PreparationScheme,
    data_keeper: &mut DataKeeper,
) -> MergerCallResult {
    let trace_pos = data_keeper.result_trace_next_pos();
    prepare_positions_mapping(scheme, data_keeper);

    MergerCallResult::CallResult {
        value,
        trace_pos,
        scheme,
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ValueType<'i> {
    Scalar,
    Stream(&'i str),
}

impl<'i> ValueType<'i> {
    pub(self) fn from_output_value(output_value: &'i CallOutputValue<'_>) -> Self {
        match output_value {
            CallOutputValue::Stream(stream) => ValueType::Stream(stream.name),
            _ => ValueType::Scalar,
        }
    }
}

use std::fmt;
impl fmt::Display for ValueType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Scalar => write!(f, "scalar"),
            ValueType::Stream(stream_name) => write!(f, "${}", stream_name),
        }
    }
}
