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

pub(super) fn merge_executed<VT: Clone + Eq>(
    prev_value: Value<VT>,
    current_value: Value<VT>,
) -> MergeResult<CallResult<VT>, VT> {
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

fn are_scalars_equal<VT: Clone + Eq>(prev_value: &Value<VT>, current_value: &Value<VT>) -> MergeResult<(), VT> {
    if prev_value == current_value {
        return Ok(());
    }

    Err(CallResultError::not_equal_values(
        prev_value.clone(),
        current_value.clone(),
    ))
}

fn are_streams_equal<VT: Clone + Eq>(
    prev_result_value: &VT,
    current_result_value: &VT,
    prev_value: &Value<VT>,
    current_value: &Value<VT>,
) -> MergeResult<(), VT> {
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
pub(super) fn merge_current_executed<VT: Clone>(
    value: Value<VT>,
    value_type: ValueType<'_>,
    data_keeper: &DataKeeper<VT>,
) -> MergeResult<CallResult<VT>, VT> {
    match (value, value_type) {
        (scalar @ Value::Scalar(_), ValueType::Scalar) => Ok(CallResult::Executed(scalar)),
        (Value::Stream { value, .. }, ValueType::Stream(stream_name)) => {
            let generation = data_keeper.prev_ctx.stream_generation(stream_name).unwrap_or_default();
            let stream = Value::Stream { value, generation };
            Ok(CallResult::Executed(stream))
        }
        (value, value_type) => Err(CallResultError::data_not_match(value, value_type)),
    }
}

pub(super) fn check_equal<VT: Clone + Eq>(
    prev_call: &CallResult<VT>,
    current_call: &CallResult<VT>,
) -> MergeResult<(), VT> {
    if prev_call != current_call {
        Err(CallResultError::incompatible_calls(
            prev_call.clone(),
            current_call.clone(),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn try_match_value_type<VT: Clone>(
    merged_call: &MergerCallResult<VT>,
    value_type: ValueType<'_>,
) -> MergeResult<(), VT> {
    if let MergerCallResult::CallResult { value, .. } = merged_call {
        return match (value, value_type) {
            (CallResult::Executed(value @ Value::Scalar(_)), ValueType::Stream(_)) => {
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
