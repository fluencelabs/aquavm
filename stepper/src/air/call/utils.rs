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
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

pub(super) fn prepare_evidence_state(
    is_current_peer: bool,
    exec_ctx: &mut ExecutionCtx<'_>,
    call_ctx: &mut CallEvidenceCtx,
) -> Result<bool> {
    if call_ctx.current_subtree_elements_count == 0 {
        log::info!("call evidence: previous state wasn't found");
        return Ok(true);
    }

    call_ctx.current_subtree_elements_count -= 1;
    // unwrap is safe here, because current_subtree_elements_count depends on current_path len,
    // and it's been checked previously
    let prev_state = call_ctx.current_path.pop_front().unwrap();

    log::info!("call evidence: previous state found {:?}", prev_state);

    match &prev_state {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        EvidenceState::Call(CallResult::CallServiceFailed(err_msg)) => {
            let err_msg = err_msg.clone();
            call_ctx.new_path.push_back(prev_state);
            exec_ctx.subtree_complete = false;
            Err(AquamarineError::LocalServiceError(err_msg))
        }
        EvidenceState::Call(CallResult::RequestSent(..)) => {
            // check whether current node can execute this call
            if is_current_peer {
                Ok(true)
            } else {
                exec_ctx.subtree_complete = false;
                call_ctx.new_path.push_back(prev_state);
                Ok(false)
            }
        }
        // this instruction's been already executed
        EvidenceState::Call(CallResult::Executed(..)) => {
            call_ctx.new_path.push_back(prev_state);
            Ok(false)
        }
        // state has inconsistent order - return a error, call shouldn't be executed
        par_state @ EvidenceState::Par(..) => Err(AquamarineError::InvalidEvidenceState(
            par_state.clone(),
            String::from("call"),
        )),
    }
}

pub(super) fn set_local_call_result(
    result_variable_name: String,
    exec_ctx: &mut ExecutionCtx<'_>,
    call_ctx: &mut CallEvidenceCtx,
    result: JValue,
) -> Result<()> {
    use std::collections::hash_map::Entry::{Occupied, Vacant};

    let is_array = result_variable_name.ends_with("[]");
    let result_ref = &result;
    let new_evidence_state = EvidenceState::Call(CallResult::Executed(result_variable_name.clone(), result));

    if !is_array {
        // if result is not an array, simply insert it into data
        if exec_ctx
            .data_cache
            .insert(result_variable_name.as_str(), AValue::JValueRef(result_ref))
            .is_some()
        {
            return Err(AquamarineError::MultipleVariablesFound(result_variable_name));
        }

        log::info!("call evidence: adding new state {:?}", new_evidence_state);
        call_ctx.new_path.push_back(new_evidence_state);

        return Ok(());
    }

    // unwrap is safe because it's been checked for []
    let result_variable_name = result_variable_name.strip_suffix("[]").unwrap().to_string();
    // if result is an array, insert result to the end of the array
    match exec_ctx.data_cache.entry(result_variable_name.as_str()) {
        Occupied(mut entry) => match entry.get_mut() {
            AValue::JValueAccumulatorRef(values) => values.push(result_ref),
            _v => {
                unimplemented!("return a error");
                /*
                return Err(AquamarineError::IncompatibleJValueType(
                    v.clone(),
                    String::from("Array"),
                ))

                 */
            }
        },
        Vacant(entry) => {
            entry.insert(AValue::JValueAccumulatorRef(vec![result_ref]));
        }
    }

    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_path.push_back(new_evidence_state);

    Ok(())
}

pub(super) fn set_remote_call_result(peer_pk: String, exec_ctx: &mut ExecutionCtx<'_>, call_ctx: &mut CallEvidenceCtx) {
    exec_ctx.next_peer_pks.push(peer_pk.clone());
    exec_ctx.subtree_complete = false;

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent(peer_pk));
    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_path.push_back(new_evidence_state);
}

pub(super) fn is_string_literal(value: &str) -> bool {
    value.starts_with('"') && value.ends_with('"')
}
