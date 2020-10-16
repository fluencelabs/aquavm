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

use super::utils::is_string_literal;
use super::utils::prepare_evidence_state;
use super::Call;
use super::CURRENT_PEER_ALIAS;
use crate::air::ExecutionCtx;
use crate::air::RESERVED_KEYWORDS;
use crate::call_evidence::CallEvidenceCtx;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ParsedCall {
    peer_pk: String,
    service_id: String,
    function_name: String,
    function_arg_paths: Vec<String>,
    result_variable_name: String,
}

impl ParsedCall {
    pub(super) fn new(raw_call: &Call, exec_ctx: &ExecutionCtx) -> Result<Self> {
        let (peer_pk, service_id, func_name) = Self::prepare_peer_fn_parts(raw_call, exec_ctx)?;
        let result_variable_name = Self::parse_result_variable_name(raw_call)?;

        Ok(Self {
            peer_pk: peer_pk.to_string(),
            service_id: service_id.to_string(),
            function_name: func_name.to_string(),
            function_arg_paths: raw_call.2.clone(),
            result_variable_name: result_variable_name.to_string(),
        })
    }

    pub(super) fn execute(self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        let is_current_peer = self.peer_pk == exec_ctx.current_peer_id;
        let should_executed = prepare_evidence_state(is_current_peer, exec_ctx, call_ctx)?;
        if !should_executed {
            return Ok(());
        }

        if self.peer_pk != exec_ctx.current_peer_id && self.peer_pk != CURRENT_PEER_ALIAS {
            super::utils::set_remote_call_result(self.peer_pk, exec_ctx, call_ctx);

            return Ok(());
        }

        let function_args = self.extract_args_by_paths(exec_ctx)?;
        let function_args = serde_json::to_string(&function_args)
            .map_err(|e| AquamarineError::FuncArgsSerializationError(function_args, e))?;

        let result = unsafe { crate::call_service(self.service_id, self.function_name, function_args) };

        if result.ret_code != crate::CALL_SERVICE_SUCCESS {
            return Err(AquamarineError::LocalServiceError(result.result));
        }

        let result: JValue = serde_json::from_str(&result.result)
            .map_err(|e| AquamarineError::CallServiceResultDeserializationError(result, e))?;
        super::utils::set_local_call_result(self.result_variable_name, exec_ctx, call_ctx, result)
    }

    fn prepare_peer_fn_parts<'a>(
        raw_call: &'a Call,
        exec_ctx: &'a ExecutionCtx,
    ) -> Result<(&'a str, &'a str, &'a str)> {
        use super::FunctionPart::*;
        use super::PeerPart::*;

        let (peer_pk, service_id, func_name) = match (&raw_call.0, &raw_call.1) {
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

        let peer_pk = if peer_pk != CURRENT_PEER_ALIAS {
            Self::prepare_call_arg(peer_pk, exec_ctx)?
        } else {
            peer_pk
        };

        let service_id = Self::prepare_call_arg(service_id, exec_ctx)?;
        let func_name = Self::prepare_call_arg(func_name, exec_ctx)?;

        Ok((peer_pk, service_id, func_name))
    }

    fn extract_args_by_paths(&self, ctx: &ExecutionCtx) -> Result<JValue> {
        let mut result = Vec::with_capacity(self.function_arg_paths.len());

        for arg_path in self.function_arg_paths.iter() {
            if is_string_literal(arg_path) {
                result.push(JValue::String(arg_path[1..arg_path.len() - 1].to_string()));
            } else {
                let arg = Self::get_args_by_path(arg_path, ctx)?;
                result.extend(arg.into_iter().cloned());
            }
        }

        Ok(JValue::Array(result))
    }

    fn parse_result_variable_name(call: &Call) -> Result<&str> {
        let result_variable_name = &call.3;

        if result_variable_name.is_empty() {
            return Err(AquamarineError::InstructionError(String::from(
                "result name of a call instruction must be non empty",
            )));
        }

        if RESERVED_KEYWORDS.contains(result_variable_name.as_str()) {
            return Err(AquamarineError::ReservedKeywordError(result_variable_name.to_string()));
        }

        if is_string_literal(result_variable_name) {
            return Err(AquamarineError::InstructionError(String::from(
                "result name of a call instruction must be non string literal",
            )));
        }

        Ok(result_variable_name)
    }

    fn get_args_by_path<'args_path, 'ctx>(
        args_path: &'args_path str,
        ctx: &'ctx ExecutionCtx,
    ) -> Result<Vec<&'ctx JValue>> {
        let mut split_arg: Vec<&str> = args_path.splitn(2, '.').collect();
        let arg_path_head = split_arg.remove(0);

        let value_by_head = match (ctx.data.get(arg_path_head), ctx.folds.get(arg_path_head)) {
            (_, Some(fold_state)) => match ctx.data.get(&fold_state.iterable_name) {
                Some(JValue::Array(values)) => &values[fold_state.cursor],
                Some(v) => {
                    return Err(AquamarineError::IncompatibleJValueType(
                        v.clone(),
                        String::from("array"),
                    ))
                }
                None => return Err(AquamarineError::VariableNotFound(fold_state.iterable_name.clone())),
            },
            (Some(value), None) => value,
            (None, None) => return Err(AquamarineError::VariableNotFound(arg_path_head.to_string())),
        };

        if split_arg.is_empty() {
            return Ok(vec![value_by_head]);
        }

        let json_path = split_arg.remove(0);
        let values = jsonpath_lib::select(value_by_head, json_path)
            .map_err(|e| AquamarineError::VariableNotInJsonPath(value_by_head.clone(), String::from(json_path), e))?;

        Ok(values)
    }

    fn prepare_call_arg<'a>(arg_path: &'a str, ctx: &'a ExecutionCtx) -> Result<&'a str> {
        if RESERVED_KEYWORDS.contains(arg_path) {
            return Err(AquamarineError::ReservedKeywordError(arg_path.to_string()));
        }

        if is_string_literal(arg_path) {
            return Ok(&arg_path[1..arg_path.len() - 1]);
        }

        let args = Self::get_args_by_path(arg_path, ctx)?;
        if args.is_empty() {
            return Err(AquamarineError::VariableNotFound(arg_path.to_string()));
        }

        if args.len() != 1 {
            return Err(AquamarineError::MultipleValuesInJsonPath(arg_path.to_string()));
        }

        match args[0] {
            JValue::String(str) => Ok(str),
            v => Err(AquamarineError::IncompatibleJValueType(
                v.clone(),
                String::from("string"),
            )),
        }
    }
}
