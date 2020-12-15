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
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AquamarineError;
use crate::JValue;
use crate::ResolvedCallResult;
use crate::Result;

use air_parser::ast::CallOutput;
use plets::ResolvedTriplet;

use std::rc::Rc;

/// Writes result of a local `Call` instruction to `ExecutionCtx` at `output`.
pub(super) fn set_local_call_result<'i>(
    result: Rc<JValue>,
    triplet: Rc<ResolvedTriplet>,
    output: &CallOutput<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> Result<()> {
    use crate::AValue;
    use std::cell::RefCell;
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use AquamarineError::*;

    let executed_result = ResolvedCallResult { result, triplet };

    match output {
        CallOutput::Scalar(name) => {
            if let Some(fold_block_name) = exec_ctx.met_folds.back() {
                let fold_state = match exec_ctx.data_cache.get_mut(*fold_block_name) {
                    Some(AValue::JValueFoldCursor(fold_state)) => fold_state,
                    _ => unreachable!("fold block data must be represented as fold cursor"),
                };

                fold_state.met_variables.insert(name, executed_result.clone());
            }

            match exec_ctx.data_cache.entry(name.to_string()) {
                Vacant(entry) => {
                    entry.insert(AValue::JValueRef(executed_result));
                }
                Occupied(mut entry) => {
                    // check that current execution flow is inside a fold block
                    if exec_ctx.met_folds.is_empty() {
                        // shadowing is allowed only inside fold blocks
                        return Err(MultipleVariablesFound(entry.key().clone()));
                    }

                    match entry.get() {
                        AValue::JValueRef(_) => {}
                        // shadowing is allowed only for scalar values
                        _ => return Err(ShadowingError(entry.key().clone())),
                    };

                    entry.insert(AValue::JValueRef(executed_result));
                }
            };
        }
        CallOutput::Accumulator(name) => {
            match exec_ctx.data_cache.entry(name.to_string()) {
                Occupied(mut entry) => match entry.get_mut() {
                    // if result is an array, insert result to the end of the array
                    AValue::JValueAccumulatorRef(values) => values.borrow_mut().push(executed_result),
                    v => return Err(IncompatibleAValueType(format!("{}", v), String::from("Array"))),
                },
                Vacant(entry) => {
                    entry.insert(AValue::JValueAccumulatorRef(RefCell::new(vec![executed_result])));
                }
            };
        }
        CallOutput::None => {}
    }

    Ok(())
}

/// Writes evidence of a particle being sent to remote node
pub(super) fn set_remote_call_result<'i>(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx<'i>,
    call_ctx: &mut CallEvidenceCtx,
) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent(exec_ctx.current_peer_id.clone()));
    log::trace!(
        target: EVIDENCE_CHANGING,
        "  adding new call evidence state {:?}",
        new_evidence_state
    );
    call_ctx.new_path.push_back(new_evidence_state);
}

pub(super) fn handle_prev_state<'i>(
    triplet: &Rc<ResolvedTriplet>,
    output: &CallOutput<'i>,
    prev_state: EvidenceState,
    exec_ctx: &mut ExecutionCtx<'i>,
    call_ctx: &mut CallEvidenceCtx,
) -> Result<bool> {
    use crate::call_evidence::CallResult::*;
    use crate::call_evidence::EvidenceState::*;

    match &prev_state {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        Call(CallServiceFailed(err_msg)) => {
            let err_msg = err_msg.clone();
            call_ctx.new_path.push_back(prev_state);
            exec_ctx.subtree_complete = false;
            Err(AquamarineError::LocalServiceError(err_msg))
        }
        Call(RequestSent(..)) => {
            let peer_pk = triplet.peer_pk.as_str();
            // check whether current node can execute this call
            let is_current_peer = peer_pk == exec_ctx.current_peer_id;
            if is_current_peer {
                Ok(true)
            } else {
                exec_ctx.subtree_complete = false;
                call_ctx.new_path.push_back(prev_state);
                Ok(false)
            }
        }
        // this instruction's been already executed
        Call(Executed(result)) => {
            set_local_call_result(result.clone(), triplet.clone(), output, exec_ctx)?;
            call_ctx.new_path.push_back(prev_state);
            Ok(false)
        }
        // state has inconsistent order - return a error, call shouldn't be executed
        par_state @ Par(..) => Err(AquamarineError::InvalidEvidenceState(
            par_state.clone(),
            String::from("call"),
        )),
    }
}
