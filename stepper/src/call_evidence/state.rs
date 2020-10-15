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

use serde::Deserialize;
use serde::Serialize;

use std::collections::VecDeque;

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
    mut prev_states: VecDeque<EvidenceState>,
    mut states: VecDeque<EvidenceState>,
) -> Result<VecDeque<EvidenceState>> {
    let mut merged_call_states = VecDeque::new();

    let prev_subtree_size = prev_states.len();
    let subtree_size = states.len();
    handle_subtree(
        &mut prev_states,
        prev_subtree_size,
        &mut states,
        subtree_size,
        &mut merged_call_states,
    )?;

    Ok(merged_call_states)
}

fn handle_subtree(
    prev_states: &mut VecDeque<EvidenceState>,
    mut prev_subtree_size: usize,
    states: &mut VecDeque<EvidenceState>,
    mut subtree_size: usize,
    result: &mut VecDeque<EvidenceState>,
) -> Result<()> {
    use EvidenceState::Call;
    use EvidenceState::Par;

    loop {
        let prev_state = if prev_subtree_size != 0 {
            prev_subtree_size -= 1;
            prev_states.pop_front()
        } else {
            None
        };

        let state = if subtree_size != 0 {
            subtree_size -= 1;
            states.pop_front()
        } else {
            None
        };

        match (prev_state, state) {
            (Some(Call(prev_call)), Some(Call(call))) => {
                let resulted_call = handle_call(prev_call, call)?;
                result.push_back(Call(resulted_call));
            }
            (Some(Par(prev_left, prev_right)), Some(Par(left, right))) => {
                handle_subtree(prev_states, prev_left, states, left, result)?;
                handle_subtree(prev_states, prev_right, states, right, result)?;
            }
            (None, Some(_)) => {
                result.extend(states.drain(..));
            }
            (Some(_), None) => {
                result.extend(prev_states.drain(..));
            }
            // TODO: return a error
            (Some(Call(..)), Some(Par(..))) => unimplemented!(),
            (Some(Par(..)), Some(Call(..))) => unimplemented!(),
            (None, None) => break,
        }
    }

    Ok(())
}

fn handle_call(prev_call_result: CallResult, call_result: CallResult) -> Result<CallResult> {
    use CallResult::*;

    match (&prev_call_result, &call_result) {
        (CallServiceFailed(prev_err_msg), CallServiceFailed(err_msg)) => {
            if prev_err_msg != err_msg {}
            Ok(call_result)
        }
        (RequestSent, CallServiceFailed(_)) => Ok(call_result),
        (CallServiceFailed(_), RequestSent) => Ok(prev_call_result),
        (RequestSent, Executed) => Ok(call_result),
        (Executed, RequestSent) => Ok(prev_call_result),
        (Executed, Executed) => Ok(prev_call_result),
        // TODO: make them errors
        (CallServiceFailed(_), Executed) => Ok(Executed),
        (Executed, CallServiceFailed(_)) => Ok(Executed),
        _ => Ok(Executed),
    }
}
