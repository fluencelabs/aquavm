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

use crate::AquamarineError::IncompatibleCallResults;
use crate::AquamarineError::IncompatibleEvidenceStates;
use crate::Result;

use serde::Deserialize;
use serde::Serialize;

pub(crate) type CallEvidencePath = std::collections::VecDeque<EvidenceState>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CallResult {
    /// Request was sent to a target node and it shouldn't be called again.
    RequestSent,

    /// A corresponding call's been already executed.
    Executed,

    /// call_service ended with a service error.
    CallServiceFailed(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceState {
    Par(usize, usize),
    Call(CallResult),
}

pub(crate) fn merge_call_states(
    mut prev_path: CallEvidencePath,
    mut current_path: CallEvidencePath,
) -> Result<CallEvidencePath> {
    let mut merged_path = CallEvidencePath::new();

    let prev_subtree_size = prev_path.len();
    let current_subtree_size = current_path.len();

    handle_subtree(
        &mut prev_path,
        prev_subtree_size,
        &mut current_path,
        current_subtree_size,
        &mut merged_path,
    )?;

    Ok(merged_path)
}

fn handle_subtree(
    prev_path: &mut CallEvidencePath,
    mut prev_subtree_size: usize,
    current_path: &mut CallEvidencePath,
    mut current_subtree_size: usize,
    result_path: &mut CallEvidencePath,
) -> Result<()> {
    use EvidenceState::Call;
    use EvidenceState::Par;

    loop {
        let prev_state = if prev_subtree_size != 0 {
            prev_subtree_size -= 1;
            prev_path.pop_front()
        } else {
            None
        };

        let state = if current_subtree_size != 0 {
            current_subtree_size -= 1;
            current_path.pop_front()
        } else {
            None
        };

        match (prev_state, state) {
            (Some(Call(prev_call)), Some(Call(call))) => {
                let resulted_call = handle_call(prev_call, call)?;
                result_path.push_back(Call(resulted_call));
            }
            (Some(Par(prev_left, prev_right)), Some(Par(current_left, current_right))) => {
                handle_subtree(prev_path, prev_left, current_path, current_left, result_path)?;
                handle_subtree(prev_path, prev_right, current_path, current_right, result_path)?;
            }
            (None, Some(_)) => {
                result_path.extend(current_path.drain(..current_subtree_size));
                break;
            }
            (Some(_), None) => {
                result_path.extend(prev_path.drain(..prev_subtree_size));
                break;
            }
            (None, None) => break,
            // this match arn represents (Call, Par) and (Par, Call) states
            (Some(prev_state), Some(current_state)) => {
                return Err(IncompatibleEvidenceStates(prev_state, current_state))
            }
        }
    }

    Ok(())
}

fn handle_call(prev_call_result: CallResult, current_call_result: CallResult) -> Result<CallResult> {
    use CallResult::*;

    match (&prev_call_result, &current_call_result) {
        (CallServiceFailed(prev_err_msg), CallServiceFailed(err_msg)) => {
            if prev_err_msg != err_msg {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }
            Ok(current_call_result)
        }
        (RequestSent, CallServiceFailed(_)) => Ok(current_call_result),
        (CallServiceFailed(_), RequestSent) => Ok(prev_call_result),
        (RequestSent, RequestSent) => Ok(prev_call_result),
        (RequestSent, Executed) => Ok(current_call_result),
        (Executed, RequestSent) => Ok(prev_call_result),
        (Executed, Executed) => Ok(prev_call_result),
        (CallServiceFailed(_), Executed) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
        (Executed, CallServiceFailed(_)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
    }
}
