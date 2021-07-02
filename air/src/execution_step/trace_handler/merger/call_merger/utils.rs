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

pub(super) fn merge_executed(
    prev_result: CallResult,
    current_result: CallResult,
    value_type: ValueType<'_>,
) -> MergeResult<CallResult> {
    match value_type {
        ValueType::Stream(_) => {
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
pub(super) fn merge_current_executed(
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

pub(super) fn check_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
    if prev_result != current_result {
        Err(IncompatibleCallResults(prev_result.clone(), current_result.clone()))
    } else {
        Ok(())
    }
}

pub(super) fn check_stream_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
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
