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

use super::CallEvidenceContext;
use super::CallResult;
use super::EvidenceState;
use super::ExecutionContext;
use super::NewEvidenceState;
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

impl super::ExecutableInstruction for Call {
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<()> {
        log::info!("call {:?} is called with context {:?}", self, ctx);

        let should_be_executed = Self::should_be_executed(&ctx.call_evidence_ctx);

        // TODO: check for overflow
        ctx.call_evidence_ctx.left += 1;

        // bubble call service errors up
        let should_be_executed = should_be_executed?;
        if !should_be_executed {
            return Ok(());
        }

        let (peer_pk, service_id, func_name) = self.prepare_peer_fn_parts(ctx)?;

        let function_args = self.extract_args_by_paths(ctx)?;
        let function_args = serde_json::to_string(&function_args)
            .map_err(|e| AquamarineError::FuncArgsSerdeError(function_args, e))?;

        if peer_pk == ctx.current_peer_id || peer_pk == CURRENT_PEER_ALIAS {
            let result = unsafe {
                crate::call_service(service_id.to_string(), func_name.to_string(), function_args)
            };
            if result.ret_code != crate::CALL_SERVICE_SUCCESS {
                return Err(AquamarineError::LocalServiceError(result.result));
            }

            let result: JValue = serde_json::from_str(&result.result)
                .map_err(|e| AquamarineError::CallServiceSerdeError(result, e))?;
            self.set_local_result(ctx, result)?;
        } else {
            let peer_pk = peer_pk.to_string();
            ctx.next_peer_pks.push(peer_pk);

            let evidence_state = EvidenceState::Call(CallResult::RequestSent);
            ctx.call_evidence_ctx
                .new_states
                .push(NewEvidenceState::EvidenceState(evidence_state));
        }

        Ok(())
    }
}

impl Call {
    #[rustfmt::skip]
    fn prepare_peer_fn_parts<'a>(&'a self, ctx: &'a ExecutionContext) -> Result<(&'a str, &'a str, &'a str)> {
        let (peer_pk, service_id, func_name) = match (&self.0, &self.1) {
            (PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id), FunctionPart::ServiceIdWithFuncName(_service_id, func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
            },
            (PeerPart::PeerPkWithPkServiceId(peer_pk, peer_service_id), FunctionPart::FuncName(func_name)) => {
                Ok((peer_pk, peer_service_id, func_name))
            },
            (PeerPart::PeerPk(peer_pk), FunctionPart::ServiceIdWithFuncName(service_id, func_name)) => {
                Ok((peer_pk, service_id, func_name))
            }
            (PeerPart::PeerPk(_), FunctionPart::FuncName(_)) => Err(AquamarineError::InstructionError(
                String::from("call should have service id specified by peer part or function part"),
            )),
        }?;

        let peer_pk = if peer_pk != CURRENT_PEER_ALIAS {
            Self::prepare_call_arg(peer_pk, ctx)?
        } else {
            peer_pk
        };

        let service_id = Self::prepare_call_arg(service_id, ctx)?;
        let func_name = Self::prepare_call_arg(func_name, ctx)?;

        Ok((peer_pk, service_id, func_name))
    }

    fn extract_args_by_paths(&self, ctx: &ExecutionContext) -> Result<JValue> {
        let mut result = Vec::with_capacity(self.2.len());

        for arg_path in self.2.iter() {
            if is_string_literal(arg_path) {
                result.push(JValue::String(arg_path[1..arg_path.len() - 1].to_string()));
            } else {
                let arg = Self::get_args_by_path(arg_path, ctx)?;
                result.extend(arg.into_iter().cloned());
            }
        }

        Ok(JValue::Array(result))
    }

    fn parse_result_variable_name(&self) -> Result<&str> {
        let result_variable_name = &self.3;

        if result_variable_name.is_empty() {
            return Err(AquamarineError::InstructionError(String::from(
                "result name of a call instruction must be non empty",
            )));
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
        ctx: &'ctx ExecutionContext,
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

    fn prepare_call_arg<'a>(arg_path: &'a str, ctx: &'a ExecutionContext) -> Result<&'a str> {
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

    fn set_local_result(&self, ctx: &mut ExecutionContext, result: JValue) -> Result<()> {
        use std::collections::hash_map::Entry;

        let result_variable_name = self.parse_result_variable_name()?;

        let evidence_state = EvidenceState::Call(CallResult::Executed);
        ctx.call_evidence_ctx
            .new_states
            .push(NewEvidenceState::EvidenceState(evidence_state));

        let is_array = result_variable_name.ends_with("[]");
        if !is_array {
            // if result is not an array, simply insert it into data
            if ctx
                .data
                .insert(result_variable_name.to_string(), result)
                .is_some()
            {
                return Err(AquamarineError::MultipleVariablesFound(
                    result_variable_name.to_string(),
                ));
            }

            return Ok(());
        }

        // if result is an array, insert result to the end of the array
        match ctx
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

        Ok(())
    }

    fn should_be_executed(call_evidence_ctx: &CallEvidenceContext) -> Result<bool> {
        let left = call_evidence_ctx.left;
        let right = call_evidence_ctx.right;

        if left >= right || left >= call_evidence_ctx.current_states.len() {
            return Ok(true);
        }

        let state = &call_evidence_ctx.current_states[left];
        match state {
            EvidenceState::Call(CallResult::CallServiceFailed(err_msg)) => {
                Err(AquamarineError::LocalServiceError(err_msg.clone()))
            }
            EvidenceState::Call(_) => Ok(false),
            EvidenceState::Par(..) => Err(AquamarineError::VariableNotFound(String::new())),
        }
    }
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
        let mut vm = create_aqua_vm(echo_string_call_service());

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
        let mut vm = create_aqua_vm(echo_string_call_service());
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
        let mut vm = create_aqua_vm(echo_string_call_service());

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

        let mut vm = create_aqua_vm(call_service);

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
