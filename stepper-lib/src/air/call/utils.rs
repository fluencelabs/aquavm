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

use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn set_local_call_result(
    result_variable_name: String,
    exec_ctx: &mut ExecutionCtx,
    result: Rc<JValue>,
) -> Result<()> {
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use AquamarineError::*;

    let stripped_result_name = result_variable_name.strip_suffix("[]");
    if stripped_result_name.is_none() {
        // if result is not an array, simply insert it into data
        match exec_ctx.data_cache.entry(result_variable_name) {
            Vacant(entry) => entry.insert(AValue::JValueRef(result)),
            Occupied(entry) => return Err(MultipleVariablesFound(entry.key().clone())),
        };
        return Ok(());
    }

    // unwrap is safe because it's been checked for []
    match exec_ctx.data_cache.entry(stripped_result_name.unwrap().to_string()) {
        Occupied(mut entry) => match entry.get_mut() {
            // if result is an array, insert result to the end of the array
            AValue::JValueAccumulatorRef(values) => values.borrow_mut().push(result),
            v => return Err(IncompatibleAValueType(format!("{:?}", v), String::from("Array"))),
        },
        Vacant(entry) => {
            entry.insert(AValue::JValueAccumulatorRef(RefCell::new(vec![result])));
        }
    }

    Ok(())
}

pub(super) fn set_remote_call_result(peer_pk: String, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent(exec_ctx.current_peer_id.clone()));
    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_path.push_back(new_evidence_state);
}

pub(super) fn find_by_json_path<'jvalue, 'json_path>(
    jvalue: &'jvalue JValue,
    json_path: &'json_path str,
) -> Result<Vec<&'jvalue JValue>> {
    use AquamarineError::VariableNotInJsonPath as JsonPathError;

    jsonpath_lib::select(jvalue, json_path).map_err(|e| JsonPathError(jvalue.clone(), String::from(json_path), e))
}

pub(super) fn is_string_literal(value: &str) -> bool {
    value.starts_with('"') && value.ends_with('"')
}
