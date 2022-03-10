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

use air::UncatchableError;
use air_test_utils::prelude::*;

// Check that %init_peer_id% alias works correctly (by comparing result with it and explicit peer id).
// Additionally, check that empty string for data does the same as empty call path.
#[test]
fn current_peer_id_call() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let service_id = "local_service_id";
    let function_name = "local_fn_name";
    let script = f!(r#"
               (call %init_peer_id% ("{service_id}" "{function_name}") [] result_name)
            "#);

    let result = checked_call_vm!(vm, vm_peer_id, script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![executed_state::scalar_string("result from unit_call_service")];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());

    let script = f!(r#"
               (call "{vm_peer_id}" ("{service_id}" "{function_name}") [] result_name)
            "#);

    let result = checked_call_vm!(vm, "asd", script.clone(), "", "");

    // test that empty string for data works
    let result_with_empty_string = checked_call_vm!(vm, "asd", script, "", "");
    assert_eq!(result_with_empty_string, result);
}

// Check that specifying remote peer id in call will result its appearing in next_peer_pks.
#[test]
fn remote_peer_id_call() {
    let some_local_peer_id = String::from("some_local_peer_id");
    let mut vm = create_avm(echo_call_service(), &some_local_peer_id);

    let remote_peer_id = String::from("some_remote_peer_id");
    let script = f!(r#"(call "{remote_peer_id}" ("local_service_id" "local_fn_name") ["arg"] result_name)"#);

    let result = checked_call_vm!(vm, "asd", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::request_sent_by(some_local_peer_id);

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0], expected_state);
    assert_eq!(result.next_peer_pks, vec![remote_peer_id]);
}

// Check that setting variables works as expected.
#[test]
fn variables() {
    let mut vm = create_avm(unit_call_service(), "remote_peer_id");
    let mut set_variable_vm = create_avm(set_variable_call_service(json!("remote_peer_id")), "set_variable");

    let script = r#"
            (seq
                (call "set_variable" ("some_service_id" "local_fn_name") [] remote_peer_id)
                (call remote_peer_id ("some_service_id" "local_fn_name") [] result_name)
            )
        "#;

    let result = checked_call_vm!(set_variable_vm, "asd", script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    assert!(result.next_peer_pks.is_empty());
}

// Check that duplicate variables are impossible.
#[test]
fn duplicate_variables() {
    let peer_id = "peer_id";
    let mut vm = create_avm(unit_call_service(), peer_id);

    let variable_name = "modules";
    let script = f!(r#"
            (seq
                (call "{peer_id}" ("some_service_id" "local_fn_name") [] {variable_name})
                (call "{peer_id}" ("some_service_id" "local_fn_name") [] {variable_name})
            )
        "#);

    let result = call_vm!(vm, "asd", script, "", "");

    let expected_error = UncatchableError::MultipleVariablesFound(variable_name.to_string());
    assert!(check_error(&result, expected_error));
    assert!(result.next_peer_pks.is_empty());
}

// Check that string literals can be used as call parameters.
#[test]
fn string_parameters() {
    let call_service: CallServiceClosure =
        Box::new(|mut params| -> CallServiceResult { CallServiceResult::ok(params.arguments.remove(0)) });

    let vm_peer_id = "A";
    let mut vm = create_avm(call_service, vm_peer_id);

    let set_variable_vm_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!("arg3_value")), set_variable_vm_peer_id);

    let service_id = "some_service_id";
    let function_name = "local_fn_name";
    let script = f!(r#"
            (seq
                (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] arg3)
                (call "{vm_peer_id}" ("{service_id}" "{function_name}") ["arg1" "arg2" arg3] result)
            )
        "#);

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("arg1");

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1], expected_state);
}
