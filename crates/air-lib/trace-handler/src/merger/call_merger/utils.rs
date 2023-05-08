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

use air_interpreter_cid::CID;

pub(super) fn merge_executed(prev_value: ValueRef, current_value: ValueRef) -> MergeResult<CallResult> {
    match (&prev_value, &current_value) {
        (ValueRef::Scalar(_), ValueRef::Scalar(_)) => {
            are_scalars_equal(&prev_value, &current_value)?;
            Ok(CallResult::Executed(prev_value))
        }
        (ValueRef::Stream { cid: pr, .. }, ValueRef::Stream { cid: cr, .. }) => {
            are_streams_equal(pr, cr, &prev_value, &current_value)?;
            Ok(CallResult::Executed(prev_value))
        }
        (ValueRef::Unused(_), ValueRef::Unused(_)) => {
            are_scalars_equal(&prev_value, &current_value)?;
            Ok(CallResult::Executed(prev_value))
        }
        _ => Err(CallResultError::not_equal_values(prev_value, current_value)),
    }
}

fn are_scalars_equal(prev_value: &ValueRef, current_value: &ValueRef) -> MergeResult<()> {
    if prev_value == current_value {
        return Ok(());
    }

    Err(CallResultError::not_equal_values(
        prev_value.clone(),
        current_value.clone(),
    ))
}

fn are_streams_equal(
    prev_result_value: &CID<ServiceResultCidAggregate>,
    current_result_value: &CID<ServiceResultCidAggregate>,
    prev_value: &ValueRef,
    current_value: &ValueRef,
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
