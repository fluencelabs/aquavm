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

use crate::call_evidence::CallEvidencePath;
use crate::call_evidence::EvidenceState;
use crate::call_evidence::StateMergingError;

use std::rc::Rc;

pub(super) type MergeResult<T> = Result<T, StateMergingError>;

fn merge_call_paths(
    mut prev_path: CallEvidencePath,
    mut current_path: CallEvidencePath,
) -> MergeResult<CallEvidencePath> {
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

    log::trace!(target: EVIDENCE_PATH_MERGE, "merged path: {:?}", merged_path);

    Ok(merged_path)
}

fn merge_subtree(
    prev_path: &mut CallEvidencePath,
    mut prev_subtree_size: usize,
    current_path: &mut CallEvidencePath,
    mut current_subtree_size: usize,
    result_path: &mut CallEvidencePath,
) -> MergeResult<()> {
    use EvidenceState::Call;
    use EvidenceState::Par;
    use StateMergingError::EvidencePathTooSmall;
    use StateMergingError::IncompatibleEvidenceStates;

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
                let par_position = result_path.len();
                // place temporary Par value to avoid insert in the middle
                result_path.push_back(Par(0, 0));

                let before_result_len = result_path.len();

                merge_subtree(prev_path, prev_left, current_path, current_left, result_path)?;
                let left_par_size = result_path.len() - before_result_len;

                merge_subtree(prev_path, prev_right, current_path, current_right, result_path)?;
                let right_par_size = result_path.len() - left_par_size - before_result_len;

                // update temporary Par with final values
                result_path[par_position] = Par(left_par_size, right_par_size);

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

fn merge_call(prev_call_result: CallResult, current_call_result: CallResult) -> MergeResult<CallResult> {
    use crate::AquamarineError::IncompatibleCallResults;
    use CallResult::*;

    match (&prev_call_result, &current_call_result) {
        (CallServiceFailed(prev_err_msg), CallServiceFailed(err_msg)) => {
            if prev_err_msg != err_msg {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }
            Ok(current_call_result)
        }
        (RequestSent(_), CallServiceFailed(_)) => Ok(current_call_result),
        (CallServiceFailed(_), RequestSent(_)) => Ok(prev_call_result),
        (RequestSent(prev_sender), RequestSent(sender)) => {
            if prev_sender != sender {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }

            Ok(prev_call_result)
        }
        (RequestSent(_), Executed(..)) => Ok(current_call_result),
        (Executed(..), RequestSent(_)) => Ok(prev_call_result),
        (Executed(prev_result), Executed(result)) => {
            if prev_result != result {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }

            Ok(prev_call_result)
        }
        (CallServiceFailed(_), Executed(..)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
        (Executed(..), CallServiceFailed(_)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
    }
}

#[cfg(test)]
mod tests {
    use crate::call_evidence::CallResult;
    use crate::call_evidence::EvidenceState;
    use crate::call_evidence::{merge_call_paths, CallEvidencePath};
    use crate::JValue;

    use std::rc::Rc;

    #[test]
    fn merge_call_states_1() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent(String::from("peer_1"))));
        prev_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent(String::from("peer_3"))));
        prev_path.push_back(Call(Executed(Rc::new(JValue::Null))));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_2"))));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_4"))));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut expected_merged_path = CallEvidencePath::new();
        expected_merged_path.push_back(Par(1, 1));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Par(1, 1));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));

        assert_eq!(merged_path, expected_merged_path);
    }

    #[test]
    fn merge_call_states_2() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Par(1, 0));
        prev_path.push_back(Call(RequestSent(String::from("peer_1"))));
        prev_path.push_back(Par(1, 1));
        prev_path.push_back(Call(RequestSent(String::from("peer_2"))));
        prev_path.push_back(Call(Executed(Rc::new(JValue::Null))));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Par(2, 2));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_1"))));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_2"))));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut expected_merged_path = CallEvidencePath::new();
        expected_merged_path.push_back(Par(2, 2));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(RequestSent(String::from("peer_1"))));
        expected_merged_path.push_back(Par(1, 1));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));

        assert_eq!(merged_path, expected_merged_path);
    }

    #[test]
    fn merge_call_states_3() {
        use CallResult::*;
        use EvidenceState::*;

        let mut prev_path = CallEvidencePath::new();
        prev_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        prev_path.push_back(Par(2, 0));
        prev_path.push_back(Par(1, 0));
        prev_path.push_back(Call(RequestSent(String::from("peer_1"))));
        prev_path.push_back(Par(1, 2));
        prev_path.push_back(Call(RequestSent(String::from("peer_1"))));
        prev_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        prev_path.push_back(Call(RequestSent(String::from("peer_1"))));

        let mut current_path = CallEvidencePath::new();
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Par(3, 3));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_1"))));
        current_path.push_back(Par(1, 1));
        current_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        current_path.push_back(Call(RequestSent(String::from("peer_1"))));

        let merged_path = merge_call_paths(prev_path, current_path).expect("merging should be successful");

        let mut expected_merged_path = CallEvidencePath::new();
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Par(3, 3));
        expected_merged_path.push_back(Par(1, 1));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Par(1, 1));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(RequestSent(String::from("peer_1"))));
        expected_merged_path.push_back(Par(1, 2));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(Executed(Rc::new(JValue::Null))));
        expected_merged_path.push_back(Call(RequestSent(String::from("peer_1"))));

        assert_eq!(merged_path, expected_merged_path);
    }
}
