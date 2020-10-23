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

use crate::Result;
use crate::JValue;

use serde::Deserialize;
use serde::Serialize;
use std::cmp::max;

pub(crate) type CallEvidencePath = std::collections::VecDeque<EvidenceState>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CallResult {
    /// Request was sent to a target node by node with such public key and it shouldn't be called again.
    RequestSent(String),

    /// A corresponding call's been already executed with such value and result.
    Executed(String, JValue),

    /// call_service ended with a service error.
    CallServiceFailed(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceState {
    Par(usize, usize),
    Call(CallResult),
}

pub(crate) fn merge_call_paths(
    mut prev_path: CallEvidencePath,
    mut current_path: CallEvidencePath,
) -> Result<CallEvidencePath> {
    let mut merged_path = CallEvidencePath::new();

    let prev_subtree_size = prev_path.len();
    let current_subtree_size = current_path.len();

    merge_subtree(
        &mut prev_path,
        prev_subtree_size,
        &mut current_path,
        current_subtree_size,
        &mut merged_path,
    )?;

    log::info!("merged path: {:?}", merged_path);

    Ok(merged_path)
}

fn merge_subtree(
    prev_path: &mut CallEvidencePath,
    mut prev_subtree_size: usize,
    current_path: &mut CallEvidencePath,
    mut current_subtree_size: usize,
    result_path: &mut CallEvidencePath,
) -> Result<()> {
    use crate::AquamarineError::EvidencePathTooSmall;
    use crate::AquamarineError::IncompatibleEvidenceStates;
    use EvidenceState::Call;
    use EvidenceState::Par;

    loop {
        let prev_state = if prev_subtree_size != 0 {
            prev_subtree_size -= 1;
            prev_path.pop_front()
        } else {
            None
        };

        let current_state = if current_subtree_size != 0 {
            current_subtree_size -= 1;
            current_path.pop_front()
        } else {
            None
        };

        match (prev_state, current_state) {
            (Some(Call(prev_call)), Some(Call(call))) => {
                let resulted_call = merge_call(prev_call, call)?;
                result_path.push_back(Call(resulted_call));
            }
            (Some(Par(prev_left, prev_right)), Some(Par(current_left, current_right))) => {
                result_path.push_back(Par(max(prev_left, current_left), max(prev_right, current_right)));

                merge_subtree(prev_path, prev_left, current_path, current_left, result_path)?;
                merge_subtree(prev_path, prev_right, current_path, current_right, result_path)?;

                prev_subtree_size -= prev_left + prev_right;
                current_subtree_size -= current_left + current_right;
            }
            (None, Some(s)) => {
                if current_path.len() < current_subtree_size {
                    return Err(EvidencePathTooSmall(current_path.len(), current_subtree_size));
                }

                result_path.push_back(s);
                result_path.extend(current_path.drain(..current_subtree_size));
                break;
            }
            (Some(s), None) => {
                if prev_path.len() < prev_subtree_size {
                    return Err(EvidencePathTooSmall(prev_path.len(), prev_subtree_size));
                }

                result_path.push_back(s);
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

fn merge_call(prev_call_result: CallResult, current_call_result: CallResult) -> Result<CallResult> {
    use crate::AquamarineError::IncompatibleCallResults;
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

#[cfg(test)]
mod tests {
    use crate::call_evidence::CallResult;
    use crate::call_evidence::EvidenceState;
    use crate::call_evidence::{merge_call_paths, CallEvidencePath};

    #[test]
    fn merge_call_states_1() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Call(Executed));
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Call(Executed));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut right_merged_path = CallEvidencePath::new();
        right_merged_path.push_back(Par(1, 1));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Par(1, 1));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));

        assert_eq!(merged_path, right_merged_path);
    }

    #[test]
    fn merge_call_states_2() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Par(1, 0));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Call(Executed));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Par(2, 2));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut right_merged_path = CallEvidencePath::new();
        right_merged_path.push_back(Par(2, 2));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(RequestSent));
        right_merged_path.push_back(Par(1, 1));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));

        assert_eq!(merged_path, right_merged_path);
    }

    #[test]
    fn merge_call_states_3() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Call(Executed));
        prev_path.push_back(Par(2, 0));
        prev_path.push_back(Par(1, 0));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Par(1, 2));
        prev_path.push_back(Call(RequestSent));
        prev_path.push_back(Call(Executed));
        prev_path.push_back(Call(RequestSent));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Call(RequestSent));
        current_path.push_back(Par(3, 3));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(Executed));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed));
        current_path.push_back(Call(RequestSent));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut right_merged_path = CallEvidencePath::new();
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Par(3, 3));
        right_merged_path.push_back(Par(1, 1));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Par(1, 1));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(RequestSent));
        right_merged_path.push_back(Par(1, 2));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(Executed));
        right_merged_path.push_back(Call(RequestSent));

        assert_eq!(merged_path, right_merged_path);
    }
}
