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
