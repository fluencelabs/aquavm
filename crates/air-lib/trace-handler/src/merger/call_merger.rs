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

mod call_result_constructor;
mod utils;

use super::*;
use air_parser::ast::CallOutputValue;
use call_result_constructor::*;
use utils::*;

#[derive(Debug, Clone)]
pub enum MergerCallResult {
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
    use PrepareScheme::*;

    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();
    println!("  trace handler prev slider: {:?} {:?}", data_keeper.prev_slider().position(), data_keeper.prev_slider().subtrace_len());
    println!("  trace handler current slider: {:?} {:?}", data_keeper.current_slider().position(), data_keeper.current_slider().subtrace_len());
    println!("  trace handler: {:?} {:?}", prev_state, current_state);
    let value_type = ValueType::from_output_value(output_value);

    let (prev_call, current_call) = match (prev_state, current_state) {
        (Some(Call(prev_call)), Some(Call(current_call))) => (prev_call, current_call),
        // this special case is needed to merge stream generation in a right way
        (None, Some(Call(CallResult::Executed(value)))) => {
            let call_result = merge_current_executed(value, value_type, data_keeper)?;
            return Ok(prepare_call_result(call_result, Current, data_keeper));
        }
        (None, Some(Call(current_call))) => return Ok(prepare_call_result(current_call, Current, data_keeper)),
        (Some(Call(prev_call)), None) => return Ok(prepare_call_result(prev_call, Previous, data_keeper)),
        (None, None) => return Ok(MergerCallResult::Empty),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "call")),
    };

    let merged_call = merge_call_result(prev_call, current_call, value_type, data_keeper)?;
    let call_result = prepare_call_result(merged_call, Both, data_keeper);
    try_match_value_type(&call_result, value_type)?;

    Ok(call_result)
}

fn merge_call_result(
    prev_call: CallResult,
    current_call: CallResult,
    value_type: ValueType<'_>,
    data_keeper: &DataKeeper,
) -> MergeResult<CallResult> {
    use CallResult::*;

    let merged_state = match (prev_call, current_call) {
        (prev @ CallServiceFailed(..), current @ CallServiceFailed(..)) => {
            check_equal(&prev, &current)?;
            prev
        }
        (RequestSentBy(_), current @ CallServiceFailed(..)) => current,
        (prev @ CallServiceFailed(..), RequestSentBy(_)) => prev,
        // senders shouldn't be checked for equality, for more info please look at
        // github.com/fluencelabs/aquavm/issues/137
        (prev @ RequestSentBy(_), RequestSentBy(_)) => prev,
        // this special case is needed to merge stream generation in a right way
        (RequestSentBy(_), Executed(value)) => merge_current_executed(value, value_type, data_keeper)?,
        (prev @ Executed(..), RequestSentBy(_)) => prev,
        (Executed(prev_value), Executed(current_value)) => merge_executed(prev_value, current_value)?,
        (prev_call, current_call) => return Err(CallResultError::incompatible_calls(prev_call, current_call)),
    };

    Ok(merged_state)
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ValueType<'i> {
    Scalar,
    Stream(&'i str),
}

impl<'i> ValueType<'i> {
    pub(self) fn from_output_value(output_value: &'i CallOutputValue<'_>) -> Self {
        use air_parser::ast::Variable;

        match output_value {
            CallOutputValue::Variable(Variable::Stream(stream)) => ValueType::Stream(stream.name),
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
