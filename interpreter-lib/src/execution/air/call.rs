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

mod resolved_call;
mod triplet;
mod utils;

use resolved_call::ResolvedCall;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::ExecutionTraceCtx;
use crate::contexts::execution::LastErrorDescriptor;
use crate::log_instruction;
use crate::SecurityTetraplet;

use air_parser::ast::Call;

use std::rc::Rc;

/// This macro converts joinable errors to Ok and sets subtree complete to true.
macro_rules! joinable {
    ($cmd:expr, $exec_ctx:expr) => {
        match $cmd {
            Err(e) if is_joinable_error_type(&e) => {
                $exec_ctx.subtree_complete = false;
                return Ok(());
            }
            v => v,
        }
    };
}

impl<'i> super::ExecutableInstruction<'i> for Call<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut ExecutionTraceCtx) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);

        let resolved_call = joinable!(ResolvedCall::new(self, exec_ctx), exec_ctx).map_err(|e| {
            set_last_error(self, exec_ctx, e.clone(), None);
            e
        })?;

        let triplet = resolved_call.as_triplet();
        joinable!(resolved_call.execute(exec_ctx, trace_ctx), exec_ctx).map_err(|e| {
            let tetraplet = SecurityTetraplet::from_triplet(triplet);
            set_last_error(self, exec_ctx, e.clone(), Some(tetraplet));

            e
        })
    }
}

fn set_last_error<'i>(
    call: &Call<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    e: Rc<ExecutionError>,
    tetraplet: Option<SecurityTetraplet>,
) {
    log::warn!("call failed with an error: {}", e);

    let instruction = format!("{}", call);
    let last_error = LastErrorDescriptor::new(e, instruction, tetraplet);
    exec_ctx.last_error = Some(last_error);
    exec_ctx.last_error_could_be_set = false;
}

macro_rules! log_join {
    ($($args:tt)*) => {
        log::trace!(target: crate::log_targets::JOIN_BEHAVIOUR, $($args)*)
    }
}

/// Returns true, if supplied error is related to variable not found errors type.
/// Print log if this is joinable error type.
#[rustfmt::skip::macros(log_join)]
fn is_joinable_error_type(exec_error: &ExecutionError) -> bool {
    use ExecutionError::*;

    match exec_error {
        VariableNotFound(var_name) => {
            log_join!("  call is waiting for an argument with name '{}'", var_name);
            true
        }
        JValueJsonPathError(value, json_path, _) => {
            log_join!("  call is waiting for an argument with path '{}' on jvalue '{:?}'", json_path, value);
            true
        }
        JValueAccJsonPathError(acc, json_path, _) => {
            log_join!("  call is waiting for an argument with path '{}' on accumulator '{:?}'", json_path, acc);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::execution_trace::ExecutionTrace;
    use crate::JValue;

    use aqua_test_utils::call_vm;
    use aqua_test_utils::create_aqua_vm;
    use aqua_test_utils::echo_string_call_service;
    use aqua_test_utils::set_variable_call_service;
    use aqua_test_utils::unit_call_service;
    use aqua_test_utils::CallServiceClosure;
    use aqua_test_utils::IValue;
    use aqua_test_utils::NEVec;

    use std::rc::Rc;

    // Check that %init_peer_id% alias works correctly (by comparing result with it and explicit peer id).
    // Additionally, check that empty string for data does the same as empty call path.
    #[test]
    fn current_peer_id_call() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let vm_peer_id = String::from("test_peer_id");
        let mut vm = create_aqua_vm(unit_call_service(), vm_peer_id.clone());

        let service_id = String::from("local_service_id");
        let function_name = String::from("local_fn_name");
        let script = format!(
            r#"
               (call %init_peer_id% ("{}" "{}") [] result_name)
            "#,
            service_id, function_name
        );

        let res = call_vm!(vm, vm_peer_id.clone(), script.clone(), "[]", "[]");
        let call_path: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be a valid json");

        let executed_call_state = Call(Executed(Rc::new(JValue::String(String::from("test")))));
        assert_eq!(call_path.len(), 1);
        assert_eq!(call_path[0], executed_call_state);
        assert!(res.next_peer_pks.is_empty());

        let script = format!(
            r#"
               (call "{}" ("{}" "{}") [] result_name)
            "#,
            vm_peer_id, service_id, function_name
        );

        let res = call_vm!(vm, "asd", script.clone(), "[]", "[]");

        // test that empty string for data works
        let res_with_empty_string = call_vm!(vm, "asd", script, "", "");
        assert_eq!(res_with_empty_string, res);
    }

    // Check that specifying remote peer id in call will result its appearing in next_peer_pks.
    #[test]
    fn remote_peer_id_call() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let some_local_peer_id = String::from("some_local_peer_id");
        let mut vm = create_aqua_vm(echo_string_call_service(), some_local_peer_id.clone());

        let remote_peer_id = String::from("some_remote_peer_id");
        let script = format!(
            r#"(call "{}" ("local_service_id" "local_fn_name") [value] result_name)"#,
            remote_peer_id
        );

        let res = call_vm!(vm, "asd", script, "[]", "[]");
        let call_path: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 1);
        assert_eq!(call_path[0], Call(RequestSentBy(some_local_peer_id)));
        assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
    }

    // Check that setting variables works as expected.
    #[test]
    fn variables() {
        let mut vm = create_aqua_vm(unit_call_service(), "remote_peer_id");
        let mut set_variable_vm = create_aqua_vm(set_variable_call_service(r#""remote_peer_id""#), "set_variable");

        let script = format!(
            r#"
            (seq 
                (call "set_variable" ("some_service_id" "local_fn_name") [] remote_peer_id)
                (call remote_peer_id ("some_service_id" "local_fn_name") [] result_name)
            )
        "#,
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "[]", "[]");
        let res = call_vm!(vm, "asd", script, "[]", res.data);

        assert!(res.next_peer_pks.is_empty());
    }

    // Check that string literals can be used as call parameters.
    #[test]
    fn string_parameters() {
        use crate::contexts::execution_trace::CallResult::*;
        use crate::contexts::execution_trace::ExecutedState::*;

        let call_service: CallServiceClosure = Box::new(|_, args| -> Option<IValue> {
            let arg = match &args[2] {
                IValue::String(str) => str,
                _ => unreachable!(),
            };

            Some(IValue::Record(
                NEVec::new(vec![IValue::S32(0), IValue::String(arg.clone())]).unwrap(),
            ))
        });

        let vm_peer_id = String::from("A");
        let mut vm = create_aqua_vm(call_service, vm_peer_id.clone());

        let set_variable_vm_peer_id = String::from("set_variable");
        let mut set_variable_vm = create_aqua_vm(
            set_variable_call_service(r#""arg3_value""#),
            set_variable_vm_peer_id.clone(),
        );

        let service_id = String::from("some_service_id");
        let function_name = String::from("local_fn_name");
        let script = format!(
            r#"
            (seq 
                (call "{}" ("{}" "{}") [] arg3)
                (call "{}" ("{}" "{}") ["arg1" "arg2" arg3] result)
            )
        "#,
            set_variable_vm_peer_id, service_id, function_name, vm_peer_id, service_id, function_name
        );

        let res = call_vm!(set_variable_vm, "asd", script.clone(), "[]", "[]");
        let res = call_vm!(vm, "asd", script, "[]", res.data);
        let call_path: ExecutionTrace = serde_json::from_slice(&res.data).expect("should be a valid json");

        assert_eq!(call_path.len(), 2);
        assert_eq!(
            call_path[1],
            Call(Executed(Rc::new(JValue::Array(vec![
                JValue::String(String::from("arg1")),
                JValue::String(String::from("arg2")),
                JValue::String(String::from("arg3_value")),
            ]))))
        );
    }
}
