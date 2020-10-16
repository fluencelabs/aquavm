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

use super::ExecutionCtx;
use crate::call_evidence::CallEvidenceCtx;
use crate::call_evidence::CallResult;
use crate::call_evidence::EvidenceState;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

pub(super) fn set_local_call_result(
    result_variable_name: String,
    exec_ctx: &mut ExecutionCtx,
    call_ctx: &mut CallEvidenceCtx,
    result: JValue,
) -> Result<()> {
    use std::collections::hash_map::Entry;

    let new_evidence_state = EvidenceState::Call(CallResult::Executed);
    let is_array = result_variable_name.ends_with("[]");

    if !is_array {
        // if result is not an array, simply insert it into data
        if exec_ctx.data.insert(result_variable_name.clone(), result).is_some() {
            return Err(AquamarineError::MultipleVariablesFound(result_variable_name));
        }

        log::info!("call evidence: adding new state {:?}", new_evidence_state);
        call_ctx.new_path.push_back(new_evidence_state);

        return Ok(());
    }

    // if result is an array, insert result to the end of the array
    match exec_ctx
        .data
        // unwrap is safe because it's been checked for []
        .entry(result_variable_name.strip_suffix("[]").unwrap().to_string())
    {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            JValue::Array(values) => values.push(result),
            v => {
                return Err(AquamarineError::IncompatibleJValueType(
                    v.clone(),
                    String::from("Array"),
                ))
            }
        },
        Entry::Vacant(entry) => {
            entry.insert(JValue::Array(vec![result]));
        }
    }

    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_path.push_back(new_evidence_state);

    Ok(())
}

pub(super) fn set_remote_call_result(peer_pk: String, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) {
    exec_ctx.next_peer_pks.push(peer_pk);

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent);
    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_path.push_back(new_evidence_state);
}

pub(super) fn is_string_literal(value: &str) -> bool {
    value.starts_with('"') && value.ends_with('"')
}
