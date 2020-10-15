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

use super::CallEvidenceCtx;
use super::CallResult;
use super::EvidenceState;
use super::ExecutionCtx;
use crate::AquamarineError;
use crate::JValue;
use crate::Result;

use serde_derive::Deserialize;
use serde_derive::Serialize;

const CURRENT_PEER_ALIAS: &str = "%current_peer_id%";

/*
   (current)
   (pk $pk)
   (pk $pk $srv_id)
   PEER_PART: resolves to (peer_pk) \/ (peer_pk, pk_srv_id)

   (fn $name)
   (fn $name $srv_id)
   FN_PART: resolves to (fn_name) \/ (fn_srv_id, fn_name)
*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum PeerPart {
    PeerPk(String),
    PeerPkWithPkServiceId(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum FunctionPart {
    FuncName(String),
    ServiceIdWithFuncName(String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Call(PeerPart, FunctionPart, Vec<String>, String);

#[derive(Debug, PartialEq, Eq)]
struct ParsedCall {
    peer_pk: String,
    service_id: String,
    function_name: String,
    function_arg_paths: Vec<String>,
    result_variable_name: String,
}

impl super::ExecutableInstruction for Call {
    fn execute(&self, exec_ctx: &mut ExecutionCtx, call_ctx: &mut CallEvidenceCtx) -> Result<()> {
        log::info!(
            "call {:?} is called with contexts: {:?} {:?}",
            self,
            exec_ctx,
            call_ctx
        );

        let parsed_call = ParsedCall::new(self, exec_ctx)?;
        parsed_call.execute(exec_ctx, call_ctx)
    }
}

impl ParsedCall {
    pub fn new(call: &Call, exec_ctx: &ExecutionCtx) -> Result<Self> {
        let (peer_pk, service_id, func_name) = Self::prepare_peer_fn_parts(call, exec_ctx)?;
        let result_variable_name = Self::parse_result_variable_name(call)?;

        Ok(Self {
            peer_pk: peer_pk.to_string(),
            service_id: service_id.to_string(),
            function_name: func_name.to_string(),
            function_arg_paths: call.2.clone(),
            result_variable_name: result_variable_name.to_string(),
        })
    }

    pub fn execute(
        self,
        exec_ctx: &mut ExecutionCtx,
        call_ctx: &mut CallEvidenceCtx,
    ) -> Result<()> {
        let should_executed = self.prepare_evidence_state(call_ctx, &exec_ctx.current_peer_id)?;
        if !should_executed {
            return Ok(());
        }

        if self.peer_pk != exec_ctx.current_peer_id && self.peer_pk != CURRENT_PEER_ALIAS {
            set_remote_call_result(self.peer_pk, exec_ctx, call_ctx);

            return Ok(());
        }

        let function_args = self.extract_args_by_paths(exec_ctx)?;
        let function_args = serde_json::to_string(&function_args)
            .map_err(|e| AquamarineError::FuncArgsSerializationError(function_args, e))?;

        let result =
            unsafe { crate::call_service(self.service_id, self.function_name, function_args) };

        if result.ret_code != crate::CALL_SERVICE_SUCCESS {
            return Err(AquamarineError::LocalServiceError(result.result));
        }

        let result: JValue = serde_json::from_str(&result.result)
            .map_err(|e| AquamarineError::CallServiceResultDeserializationError(result, e))?;
        set_local_call_result(self.result_variable_name, exec_ctx, call_ctx, result)
    }

    fn prepare_peer_fn_parts<'a>(
        raw_call: &'a Call,
        exec_ctx: &'a ExecutionCtx,
    ) -> Result<(&'a str, &'a str, &'a str)> {
        let (peer_pk, service_id, func_name) = match (&raw_call.0, &raw_call.1) {
            (
                PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id),
                FunctionPart::ServiceIdWithFuncName(_service_id, func_name),
            ) => Ok((peer_pk, peer_service_id, func_name)),
            (
                PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id),
                FunctionPart::FuncName(func_name),
            ) => Ok((peer_pk, peer_service_id, func_name)),
            (
                PeerPart::PeerPk(peer_pk),
                FunctionPart::ServiceIdWithFuncName(service_id, func_name),
            ) => Ok((peer_pk, service_id, func_name)),
            (PeerPart::PeerPk(_), FunctionPart::FuncName(_)) => {
                Err(AquamarineError::InstructionError(String::from(
                    "call should have service id specified by peer part or function part",
                )))
            }
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

        if super::RESERVED_KEYWORDS.contains(result_variable_name.as_str()) {
            return Err(AquamarineError::ReservedKeywordError(
                result_variable_name.to_string(),
            ));
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
                None => {
                    return Err(AquamarineError::VariableNotFound(
                        fold_state.iterable_name.clone(),
                    ))
                }
            },
            (Some(value), None) => value,
            (None, None) => {
                return Err(AquamarineError::VariableNotFound(arg_path_head.to_string()))
            }
        };

        if split_arg.is_empty() {
            return Ok(vec![value_by_head]);
        }

        let json_path = split_arg.remove(0);
        let values = jsonpath_lib::select(value_by_head, json_path).map_err(|e| {
            AquamarineError::VariableNotInJsonPath(
                value_by_head.clone(),
                String::from(json_path),
                e,
            )
        })?;

        Ok(values)
    }

    fn prepare_call_arg<'a>(arg_path: &'a str, ctx: &'a ExecutionCtx) -> Result<&'a str> {
        if super::RESERVED_KEYWORDS.contains(arg_path) {
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
            return Err(AquamarineError::MultipleValuesInJsonPath(
                arg_path.to_string(),
            ));
        }

        match args[0] {
            JValue::String(str) => Ok(str),
            v => Err(AquamarineError::IncompatibleJValueType(
                v.clone(),
                String::from("string"),
            )),
        }
    }

    fn prepare_evidence_state(
        &self,
        call_ctx: &mut CallEvidenceCtx,
        current_peer_id: &str,
    ) -> Result<bool> {
        if call_ctx.unused_subtree_elements_count == 0 {
            log::info!("call evidence: previous state wasn't found");
            return Ok(true);
        }

        call_ctx.unused_subtree_elements_count -= 1;
        // unwrap is safe here, because current_states length's been checked
        let prev_state = call_ctx.current_states.pop_front().unwrap();

        log::info!("call evidence: previous state found {:?}", prev_state);

        match &prev_state {
            // this call was failed on one of the previous executions,
            // here it's needed to bubble this special error up
            EvidenceState::Call(CallResult::CallServiceFailed(err_msg)) => {
                let err_msg = err_msg.clone();
                call_ctx.new_states.push(prev_state);
                Err(AquamarineError::LocalServiceError(err_msg))
            }
            EvidenceState::Call(CallResult::RequestSent) => {
                // check whether current node can execute this call
                if self.peer_pk == current_peer_id {
                    Ok(true)
                } else {
                    call_ctx.new_states.push(prev_state);
                    Ok(false)
                }
            }
            // this instruction's been already executed
            EvidenceState::Call(CallResult::Executed) => {
                call_ctx.new_states.push(prev_state);
                Ok(false)
            }
            // state has inconsistent order - return a error, call shouldn't be executed
            par_state @ EvidenceState::Par(..) => Err(AquamarineError::InvalidEvidenceState(
                par_state.clone(),
                String::from("call"),
            )),
        }
    }
}

fn set_local_call_result(
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
        if exec_ctx
            .data
            .insert(result_variable_name.clone(), result)
            .is_some()
        {
            return Err(AquamarineError::MultipleVariablesFound(
                result_variable_name,
            ));
        }

        log::info!("call evidence: adding new state {:?}", new_evidence_state);
        call_ctx.new_states.push(new_evidence_state);

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
    call_ctx.new_states.push(new_evidence_state);

    Ok(())
}

fn set_remote_call_result(
    peer_pk: String,
    exec_ctx: &mut ExecutionCtx,
    call_ctx: &mut CallEvidenceCtx,
) {
    exec_ctx.next_peer_pks.push(peer_pk);

    let new_evidence_state = EvidenceState::Call(CallResult::RequestSent);
    log::info!("call evidence: adding new state {:?}", new_evidence_state);
    call_ctx.new_states.push(new_evidence_state);
}

fn is_string_literal(value: &str) -> bool {
    value.starts_with('"') && value.ends_with('"')
}

#[cfg(test)]
mod tests {
    use crate::JValue;

    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;
    use aquamarine_vm::vec1::Vec1;
    use aquamarine_vm::HostExportedFunc;
    use aquamarine_vm::IValue;

    use serde_json::json;

    #[test]
    fn current_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "test_peer_id");

        let script = String::from(
            r#"
               (call (%current_peer_id% ("local_service_id" "local_fn_name") (value) result_name))
            "#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));

        let script = String::from(
            r#"
               (call ("test_peer_id" ("local_service_id" "local_fn_name") (value) result_name))
            "#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        let res: JValue = serde_json::from_str(&res.data).unwrap();

        assert_eq!(res.get("result_name").unwrap(), &json!("test"));
    }

    #[test]
    fn remote_peer_id_call() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "");
        let remote_peer_id = String::from("some_remote_peer_id");

        let script = format!(
            r#"(call ("{}" ("local_service_id" "local_fn_name") (value) result_name))"#,
            remote_peer_id
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"value\": \"test\"}"),
            ]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }

    #[test]
    fn variables() {
        let mut vm = create_aqua_vm(echo_string_call_service(), "");

        let script = format!(
            r#"(call (remote_peer_id ("some_service_id" "local_fn_name") ("param") result_name))"#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                String::from("{\"remote_peer_id\": \"some_peer_id\"}"),
            ]))
            .expect("call should be successful");

        assert_eq!(res.next_peer_pks, vec![String::from("some_peer_id")]);
    }

    #[test]
    fn string_parameters() {
        let call_service: HostExportedFunc = Box::new(|_, args| -> Option<IValue> {
            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            Some(IValue::Record(
                Vec1::new(vec![IValue::S32(0), IValue::String(arg.clone())]).unwrap(),
            ))
        });

        let mut vm = create_aqua_vm(call_service, "");

        let script = format!(
            r#"(call (%current_peer_id% ("some_service_id" "local_fn_name") ("arg1" "arg2" arg3) result_name))"#,
        );

        let res = vm
            .call(json!([
                String::from("asd"),
                script,
                json!({"arg3": "arg3_value"}).to_string(),
            ]))
            .expect("call should be successful");

        let jdata: JValue = serde_json::from_str(&res.data).expect("should be valid json");

        assert_eq!(jdata["result_name"], json!(["arg1", "arg2", "arg3_value"]));
    }
}
