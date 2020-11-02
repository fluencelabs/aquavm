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

#![allow(unused_unsafe)] // for wasm_bindgen target where calling FFI is safe

use super::utils::find_by_json_path;
use super::utils::is_string_literal;
use super::utils::set_local_call_result;
use super::Call;
use crate::air::ExecutionCtx;
use crate::build_targets::CALL_SERVICE_SUCCESS;
use crate::call_evidence::CallEvidenceCtx;
use crate::call_evidence::CallResult;
use crate::call_evidence::EvidenceState;
use crate::log_targets::EVIDENCE_CHANGING;
use crate::AValue;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use air_parser::ast::{CallOutput, FunctionPart, PeerPart, Value};
use std::borrow::Cow;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ParsedCall<'i> {
    peer_pk: String,
    service_id: String,
    function_name: String,
    function_arg_paths: Vec<Value<'i>>,
    output: CallOutput<'i>,
}

impl<'i> ParsedCall<'i> {
    pub(super) fn new(raw_call: &Call<'i>, exec_ctx: &ExecutionCtx<'i>) -> Result<Self> {
        // 1. peer_part + fn_part => triplet
        // 2. resolve triplet (extract by variable name, take literal, %current_peer_id%, etc)
        let triplet = Triplet::try_from(&raw_call.peer, &raw_call.f)?;
        #[rustfmt::skip]
        let ResolvedTriplet { peer_pk, service_id, function_name } = triplet.resolve(exec_ctx)?;

        Ok(Self {
            peer_pk,
            service_id,
            function_name,
            function_arg_paths: raw_call.args.clone(),
            output: raw_call.output.clone(),
        })
    }

    pub(super) fn execute(self, exec_ctx: &mut ExecutionCtx<'i>, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        let should_execute = self.prepare_evidence_state(exec_ctx, call_ctx)?;
        if !should_execute {
            return Ok(());
        }

        if self.peer_pk != exec_ctx.current_peer_id {
            super::utils::set_remote_call_result(self.peer_pk, exec_ctx, call_ctx);

            return Ok(());
        }

        let function_args = self.function_arg_paths.iter();
        let function_args = function_args
            .map(|v| resolve_jvalue(v, exec_ctx))
            .collect::<Result<Vec<_>>>()?;
        let function_args = JValue::Array(function_args).to_string();

        let result = unsafe { crate::call_service(self.service_id, self.function_name, function_args) };

        if result.ret_code != CALL_SERVICE_SUCCESS {
            call_ctx
                .new_path
                .push_back(EvidenceState::Call(CallResult::CallServiceFailed(
                    result.result.clone(),
                )));
            return Err(AquamarineError::LocalServiceError(result.result));
        }

        let result: JValue = serde_json::from_str(&result.result)
            .map_err(|e| AquamarineError::CallServiceResultDeserializationError(result, e))?;
        let result = Rc::new(result);
        super::utils::set_local_call_result(self.output, exec_ctx, result.clone())?;

        let new_evidence_state = EvidenceState::Call(CallResult::Executed(result));
        log::info!(
            target: EVIDENCE_CHANGING,
            "  adding new call evidence state {:?}",
            new_evidence_state
        );
        call_ctx.new_path.push_back(new_evidence_state);

        Ok(())
    }

    pub(super) fn prepare_evidence_state(
        &self,
        exec_ctx: &mut ExecutionCtx<'i>,
        call_ctx: &mut CallEvidenceCtx,
    ) -> Result<bool> {
        use crate::call_evidence::CallResult::*;
        use crate::call_evidence::EvidenceState::*;

        if call_ctx.current_subtree_elements_count == 0 {
            log::info!(target: EVIDENCE_CHANGING, "  previous call evidence state wasn't found");
            return Ok(true);
        }

        call_ctx.current_subtree_elements_count -= 1;
        // unwrap is safe here, because current_subtree_elements_count depends on current_path len,
        // and it's been checked previously
        let prev_state = call_ctx.current_path.pop_front().unwrap();

        log::info!(
            target: EVIDENCE_CHANGING,
            "  previous call evidence state found {:?}",
            prev_state
        );

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
                // check whether current node can execute this call
                let is_current_peer = self.peer_pk == exec_ctx.current_peer_id;
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
                set_local_call_result(self.output.clone(), exec_ctx, result.clone())?;
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
}

struct Triplet<'a, 'i> {
    peer_pk: &'a Value<'i>,
    service_id: &'a Value<'i>,
    function_name: &'a Value<'i>,
}

struct ResolvedTriplet {
    peer_pk: String,
    service_id: String,
    function_name: String,
}

impl<'a, 'i> Triplet<'a, 'i> {
    pub fn try_from(peer: &'a PeerPart<'i>, f: &'a FunctionPart<'i>) -> Result<Self> {
        use air_parser::ast::FunctionPart::*;
        use air_parser::ast::PeerPart::*;

        let (peer_pk, service_id, function_name) = match (peer, f) {
            (PeerPkWithServiceId(peer_pk, peer_service_id), ServiceIdWithFuncName(_service_id, func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
            }
            (PeerPkWithServiceId(peer_pk, peer_service_id), FuncName(func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
            }
            (PeerPk(peer_pk), ServiceIdWithFuncName(service_id, func_name)) => Ok((peer_pk, service_id, func_name)),
            (PeerPk(_), FuncName(_)) => Err(AquamarineError::InstructionError(String::from(
                "call should have service id specified by peer part or function part",
            ))),
        }?;

        Ok(Self {
            peer_pk,
            service_id,
            function_name,
        })
    }

    pub fn resolve(self, ctx: &'a ExecutionCtx<'i>) -> Result<ResolvedTriplet> {
        let Triplet {
            peer_pk,
            service_id,
            function_name,
        } = self;
        let peer_pk = resolve_value(peer_pk, ctx)?.as_ref().to_string();
        let service_id = resolve_value(service_id, ctx)?.as_ref().to_string();
        let function_name = resolve_value(function_name, ctx)?.as_ref().to_string();

        Ok(ResolvedTriplet {
            peer_pk,
            service_id,
            function_name,
        })
    }
}

/// Takes variable's value from `ExecutionCtx::data_cache`
/// TODO: maybe return &'i JValue?
fn resolve_variable<'exec_ctx, 'i>(variable: &'i str, ctx: &'exec_ctx ExecutionCtx<'i>) -> Result<JValue> {
    use AquamarineError::VariableNotFound;

    let value = ctx
        .data_cache
        .get(variable)
        .ok_or_else(|| VariableNotFound(variable.to_string()))?;

    match value {
        AValue::JValueFoldCursor(fold_state) => {
            if let JValue::Array(array) = fold_state.iterable.as_ref() {
                Ok(array[fold_state.cursor].clone())
            } else {
                unreachable!("fold state must be well-formed because it is changed only by stepper")
            }
        }
        AValue::JValueRef(value) => Ok(value.as_ref().clone()),
        AValue::JValueAccumulatorRef(acc) => {
            let owned_acc = acc.borrow().iter().map(|v| v.as_ref()).cloned().collect::<Vec<_>>();
            Ok(JValue::Array(owned_acc))
        }
    }
}

fn apply_json_path<'i>(jvalue: JValue, json_path: &'i str) -> Result<JValue> {
    let values = find_by_json_path(&jvalue, json_path)?;
    if values.is_empty() {
        return Err(AquamarineError::VariableNotFound(json_path.to_string()));
    }

    if values.len() != 1 {
        return Err(AquamarineError::MultipleValuesInJsonPath(json_path.to_string()));
    }

    // TODO: sure need this clone?
    Ok(values[0].clone())
}

fn require_string(value: JValue) -> Result<String> {
    if let JValue::String(s) = value {
        Ok(s)
    } else {
        Err(AquamarineError::IncompatibleJValueType(value, "string".to_string()))
    }
}

/// Resolve value to string by either resolving variable from `ExecutionCtx`, taking literal value, or etc
fn resolve_value<'i, 'a: 'i>(value: &'a Value<'i>, ctx: &'a ExecutionCtx<'i>) -> Result<Cow<'i, str>> {
    let resolved = match value {
        Value::CurrentPeerId => Cow::Borrowed(ctx.current_peer_id.as_str()),
        Value::Literal(value) => Cow::Borrowed(*value),
        Value::Variable(name) => {
            let resolved = resolve_variable(name, ctx)?;
            let resolved = require_string(resolved)?;
            Cow::Owned(resolved)
        }
        Value::JsonPath { variable, path } => {
            let resolved = resolve_variable(variable, ctx)?;
            let resolved = apply_json_path(resolved, path)?;
            let resolved = require_string(resolved)?;
            Cow::Owned(resolved)
        }
    };

    Ok(resolved)
}

/// Resolve value to JValue, similar to `resolve_value`
fn resolve_jvalue<'i>(value: &Value<'i>, ctx: &ExecutionCtx<'i>) -> Result<JValue> {
    let value = match value {
        Value::CurrentPeerId => JValue::String(ctx.current_peer_id.clone()),
        Value::Literal(value) => JValue::String(value.to_string()),
        Value::Variable(name) => resolve_variable(name, ctx)?,
        Value::JsonPath { variable, path } => {
            let value = resolve_variable(variable, ctx)?;
            apply_json_path(value, path)?
        }
    };

    Ok(value)
}
