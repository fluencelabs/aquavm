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
use super::ExecutionError;
use super::ExecutionResult;
use crate::contexts::execution::ResolvedCallResult;
use crate::contexts::execution_trace::*;
use crate::exec_err;
use crate::execution::Variable;
use crate::log_targets::EXECUTED_STATE_CHANGING;
use crate::JValue;

use air_parser::ast::CallOutputValue;
use polyplets::ResolvedTriplet;

use std::rc::Rc;

/// Writes result of a local `Call` instruction to `ExecutionCtx` at `output`.
pub(super) fn set_local_call_result<'i>(
    result: Rc<JValue>,
    triplet: Rc<ResolvedTriplet>,
    output: &CallOutputValue<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
) -> ExecutionResult<()> {
    use crate::contexts::execution::AValue;
    use std::cell::RefCell;
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use ExecutionError::*;

    let executed_result = ResolvedCallResult { result, triplet };

    match output {
        CallOutputValue::Variable(Variable::Scalar(name)) => {
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
                        return exec_err!(MultipleVariablesFound(entry.key().clone()));
                    }

                    match entry.get() {
                        AValue::JValueRef(_) => {}
                        // shadowing is allowed only for scalar values
                        _ => return exec_err!(ShadowingError(entry.key().clone())),
                    };

                    entry.insert(AValue::JValueRef(executed_result));
                }
            };
        }
        CallOutputValue::Variable(Variable::Stream(name)) => {
            match exec_ctx.data_cache.entry(name.to_string()) {
                Occupied(mut entry) => match entry.get_mut() {
                    // if result is an array, insert result to the end of the array
                    AValue::JValueStreamRef(values) => values.borrow_mut().push(executed_result),
                    v => return exec_err!(IncompatibleAValueType(format!("{}", v), String::from("Array"))),
                },
                Vacant(entry) => {
                    entry.insert(AValue::JValueStreamRef(RefCell::new(vec![executed_result])));
                }
            };
        }
        CallOutputValue::None => {}
    }

    Ok(())
}

/// Writes an executed state of a particle being sent to remote node
pub(super) fn set_remote_call_result<'i>(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut ExecutionTraceCtx,
) {
    exec_ctx.next_peer_pks.push(peer_pk);
    exec_ctx.subtree_complete = false;

    let new_executed_state = ExecutedState::Call(CallResult::RequestSentBy(exec_ctx.current_peer_id.clone()));
    log::trace!(
        target: EXECUTED_STATE_CHANGING,
        "  adding new call executed state {:?}",
        new_executed_state
    );
    trace_ctx.new_trace.push_back(new_executed_state);
}

/// This function looks at the existing call state, validates it,
/// and returns Ok(true) if the call should be executed further.
pub(super) fn handle_prev_state<'i>(
    triplet: &Rc<ResolvedTriplet>,
    output: &CallOutputValue<'i>,
    prev_state: ExecutedState,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut ExecutionTraceCtx,
) -> ExecutionResult<bool> {
    use CallResult::*;
    use ExecutedState::*;

    match &prev_state {
        // this call was failed on one of the previous executions,
        // here it's needed to bubble this special error up
        Call(CallServiceFailed(ret_code, err_msg)) => {
            let ret_code = *ret_code;
            let err_msg = err_msg.clone();
            trace_ctx.new_trace.push_back(prev_state);
            exec_ctx.subtree_complete = false;
            exec_err!(ExecutionError::LocalServiceError(ret_code, err_msg))
        }
        Call(RequestSentBy(..)) => {
            let peer_pk = triplet.peer_pk.as_str();
            // check whether current node can execute this call
            let is_current_peer = peer_pk == exec_ctx.current_peer_id;
            if is_current_peer {
                Ok(true)
            } else {
                exec_ctx.subtree_complete = false;
                trace_ctx.new_trace.push_back(prev_state);
                Ok(false)
            }
        }
        // TODO: use value_type
        // this instruction's been already executed
        Call(Executed(result, _value_type)) => {
            set_local_call_result(result.clone(), triplet.clone(), output, exec_ctx)?;
            trace_ctx.new_trace.push_back(prev_state);
            Ok(false)
        }
        // state has inconsistent order - return a error, call shouldn't be executed
        par_state @ Par(..) => exec_err!(ExecutionError::InvalidExecutedState(
            String::from("call"),
            par_state.clone(),
        )),
        // state has inconsistent order - return a error, call shouldn't be executed
        fold_state @ Fold(..) => exec_err!(ExecutionError::InvalidExecutedState(
            String::from("fold"),
            fold_state.clone(),
        )),
    }
}
