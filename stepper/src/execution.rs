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

use super::StepperOutcome;
use crate::air::ExecutableInstruction;
use crate::air::ExecutionCtx;
use crate::air::Instruction;
use crate::call_evidence::CallEvidenceCtx;
use crate::call_evidence::CallResult;
use crate::call_evidence::EvidenceState;
use crate::get_current_peer_id;
use crate::AquaData;
use crate::AquamarineError;
use crate::Result;

use std::collections::VecDeque;

const CALL_EVIDENCE_CTX_KEY: &str = "__call";

pub(crate) fn execute_aqua(
    init_user_id: String,
    aqua: String,
    prev_data: String,
    data: String,
) -> StepperOutcome {
    log::info!(
        "stepper invoked with user_id = {}, aqua = {:?}, prev_data = {:?}, data = {:?}",
        init_user_id,
        aqua,
        prev_data,
        data
    );

    execute_aqua_impl(init_user_id, aqua, prev_data, data).unwrap_or_else(Into::into)
}

fn execute_aqua_impl(
    _init_user_id: String,
    aqua: String,
    prev_data: String,
    data: String,
) -> Result<StepperOutcome> {
    let parsed_prev_data: AquaData =
        serde_json::from_str(&prev_data).map_err(AquamarineError::DataDeserializationError)?;
    let parsed_data: AquaData =
        serde_json::from_str(&data).map_err(AquamarineError::DataDeserializationError)?;

    let formatted_aqua = format_aqua(aqua);
    let parsed_aqua = serde_sexpr::from_str::<Instruction>(&formatted_aqua)?;

    log::info!(
        "\nparsed aqua: {:?}\nparsed prev_data: {:?}\nparsed data: {:?}",
        parsed_aqua,
        parsed_prev_data,
        parsed_data
    );

    let (mut execution_ctx, mut call_evidence_ctx) = make_contexts(parsed_prev_data, parsed_data)?;

    parsed_aqua.execute(&mut execution_ctx, &mut call_evidence_ctx)?;

    let serialized_call_ctx = serde_json::to_value(call_evidence_ctx.new_states)
        .map_err(AquamarineError::CallEvidenceSerializationError)?;
    execution_ctx
        .data
        .insert(CALL_EVIDENCE_CTX_KEY.to_string(), serialized_call_ctx);

    let data = serde_json::to_string(&execution_ctx.data)
        .map_err(AquamarineError::DataSerializationError)?;

    Ok(StepperOutcome {
        ret_code: 0,
        data,
        next_peer_pks: dedup(execution_ctx.next_peer_pks),
    })
}

/// Formats aqua script in a form of S-expressions to a form compatible with the serde_sexpr crate.
fn format_aqua(aqua: String) -> String {
    use std::iter::FromIterator;

    let mut formatted_aqua = Vec::with_capacity(aqua.len());
    // whether to skip the next whitespace
    let mut skip_next_whitespace = false;
    // whether c was a closing brace
    let mut was_cbr = false;

    for c in aqua.chars() {
        let is_whitespace = c == ' ';
        if (skip_next_whitespace && is_whitespace) || c == '\n' {
            continue;
        }

        let is_cbr = c == ')';

        skip_next_whitespace = is_whitespace || c == '(' || is_cbr;
        if was_cbr && !is_cbr {
            formatted_aqua.push(' ');
        }

        was_cbr = is_cbr;
        formatted_aqua.push(c)
    }

    String::from_iter(formatted_aqua.into_iter())
}

fn make_contexts(
    mut prev_data: AquaData,
    mut data: AquaData,
) -> Result<(ExecutionCtx, CallEvidenceCtx)> {
    let current_peer_id = get_current_peer_id()
        .map_err(|e| AquamarineError::CurrentPeerIdEnvError(e, String::from("CURRENT_PEER_ID")))?;

    let prev_states: VecDeque<EvidenceState> = match prev_data.remove(CALL_EVIDENCE_CTX_KEY) {
        Some(jvalue) => serde_json::from_value(jvalue)
            .map_err(AquamarineError::CallEvidenceDeserializationError)?,
        None => VecDeque::new(),
    };

    let states: VecDeque<EvidenceState> = match data.remove(CALL_EVIDENCE_CTX_KEY) {
        Some(jvalue) => serde_json::from_value(jvalue)
            .map_err(AquamarineError::CallEvidenceDeserializationError)?,
        None => VecDeque::new(),
    };

    let data = merge_data(prev_data, data);
    let current_states = merge_call_states(prev_states, states)?;

    let execution_ctx = ExecutionCtx::new(data, current_peer_id);
    let call_evidence_ctx = CallEvidenceCtx::new(current_states);

    Ok((execution_ctx, call_evidence_ctx))
}

fn merge_data(prev_data: AquaData, mut data: AquaData) -> AquaData {
    // TODO: check for different values for one key in maps
    data.extend(prev_data);
    data
}

fn merge_call_states(
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
                result.push_back(EvidenceState::Call(resulted_call));
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
        // TODO: make it errors
        (CallServiceFailed(_), Executed) => Ok(Executed),
        (Executed, CallServiceFailed(_)) => Ok(Executed),
        _ => Ok(Executed),
    }
}

use std::hash::Hash;

fn dedup<T: Eq + Hash>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_aqua_test() {
        let aqua = format!(
            r#"(( ((  (seq (
            (call (%current_peer_id% (add_module ||) (module) module))
            (seq (
                (call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id))
                (seq (
                    (call (%current_peer_id% (create ||) (blueprint_id) service_id))
                    (call ({} (|| ||) (service_id) client_result))
                )  )
            ) )
        ))"#,
            "abc"
        );

        let aqua = super::format_aqua(aqua);
        let formatted_aqua = String::from("(((((seq ((call (%current_peer_id% (add_module ||) (module) module)) (seq ((call (%current_peer_id% (add_blueprint ||) (blueprint) blueprint_id)) (seq ((call (%current_peer_id% (create ||) (blueprint_id) service_id)) (call (abc (|| ||) (service_id) client_result))))))))");

        assert_eq!(aqua, formatted_aqua);
    }
}
