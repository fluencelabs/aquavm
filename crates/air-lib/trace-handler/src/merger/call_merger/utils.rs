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

pub(crate) type JValue = serde_json::Value;

use std::rc::Rc;

pub(super) fn merge_executed(prev_value: Value, current_value: Value) -> MergeResult<CallResult> {
    match (&prev_value, &current_value) {
        (Value::Scalar(_), Value::Scalar(_)) => {
            are_scalars_equal(&prev_value, &current_value)?;
            Ok(CallResult::Executed(prev_value))
        }
        (Value::Stream { value: pr, .. }, Value::Stream { value: cr, .. }) => {
            are_streams_equal(pr, cr, &prev_value, &current_value)?;
            Ok(CallResult::Executed(prev_value))
        }
        _ => Err(CallResultError::not_equal_values(prev_value, current_value)),
    }
}

fn are_scalars_equal(prev_value: &Value, current_value: &Value) -> MergeResult<()> {
    if prev_value == current_value {
        return Ok(());
    }

    Err(CallResultError::not_equal_values(
        prev_value.clone(),
        current_value.clone(),
    ))
}

fn are_streams_equal(
    prev_result_value: &Rc<JValue>,
    current_result_value: &Rc<JValue>,
    prev_value: &Value,
    current_value: &Value,
) -> MergeResult<()> {
    // values from streams could have different generations and it's ok
    if prev_result_value == current_result_value {
        return Ok(());
    }

    Err(CallResultError::not_equal_values(
        prev_value.clone(),
        current_value.clone(),
    ))
}

/// Merging of value from only current data to a stream is a something special, because it's
/// needed to choose generation not from current data, but a maximum from streams on a current peer.
/// Maximum versions are tracked in data in a special field called streams.
pub(super) fn merge_current_executed<'i>(
    value: Value,
    value_type: ValueType<'i>,
    scheme: PreparationScheme,
    data_keeper: &mut DataKeeper,
) -> MergeResult<MergerCallResult<'i>> {
    match (value, value_type) {
        (scalar @ Value::Scalar(_), ValueType::Scalar) => {
            Ok(prepare_call_result(CallResult::Executed(scalar), scheme, data_keeper))
        }
        (Value::Stream { value, .. }, ValueType::Stream(stream_name, stream_pos)) => Ok(prepare_new_stream_result(
            value,
            stream_name,
            stream_pos,
            scheme,
            data_keeper,
        )),
        (value, value_type) => Err(CallResultError::data_not_match(value, value_type)),
    }
}

pub(super) fn check_equal(prev_call: &CallResult, current_call: &CallResult) -> MergeResult<()> {
    if prev_call != current_call {
        Err(CallResultError::incompatible_calls(
            prev_call.clone(),
            current_call.clone(),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn try_match_value_type(merged_call: &MergerCallResult<'_>, value_type: ValueType<'_>) -> MergeResult<()> {
    if let MergerCallResult::CallResult { value, .. } = merged_call {
        return match (value, value_type) {
            (CallResult::Executed(value @ Value::Scalar(_)), ValueType::Stream(_, _)) => {
                Err(CallResultError::data_not_match(value.clone(), value_type))
            }
            (CallResult::Executed(value @ Value::Stream { .. }), ValueType::Scalar) => {
                Err(CallResultError::data_not_match(value.clone(), value_type))
            }
            _ => Ok(()),
        };
    }

    Ok(())
}
