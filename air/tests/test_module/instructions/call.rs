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

use air::{ExecutionCidState, UncatchableError};
use air_interpreter_data::{ExecutionTrace, ServiceResultAggregate};
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

    let test_params = TestRunParameters::from_init_peer_id(vm_peer_id);
    let result = checked_call_vm!(vm, test_params, script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(
        "result from unit_call_service",
        peer = vm_peer_id,
        service = service_id,
        function = function_name
    )];

    assert_eq!(actual_trace, expected_trace);
    assert!(result.next_peer_pks.is_empty());

    let script = f!(r#"
               (call "{vm_peer_id}" ("{service_id}" "{function_name}") [] result_name)
            "#);

    let result = checked_call_vm!(vm, <_>::default(), script.clone(), "", "");

    // test that empty string for data works
    let result_with_empty_string = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result_with_empty_string, result);
}

#[test]
fn call_with_timestamp() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = r#"(call %init_peer_id% ("" "") [%timestamp%] result_name)"#;

    let test_params = TestRunParameters::new(vm_peer_id, 1337, 0);
    let result = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(
        test_params.timestamp,
        peer = vm_peer_id,
        args = [test_params.timestamp]
    )];

    assert_eq!(actual_trace, expected_trace);
}

#[test]
fn call_with_ttl() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id);

    let script = f!(r#"(call "{vm_peer_id}" ("" "") [%ttl%] result_name)"#);

    let test_params = TestRunParameters::from_ttl(1337);
    let result = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(test_params.ttl, peer = vm_peer_id, args = [test_params.ttl])];

    assert_eq!(actual_trace, expected_trace);
}

// Check that specifying remote peer id in call will result its appearing in next_peer_pks.
#[test]
fn remote_peer_id_call() {
    let some_local_peer_id = String::from("some_local_peer_id");
    let mut vm = create_avm(echo_call_service(), &some_local_peer_id);

    let remote_peer_id = String::from("some_remote_peer_id");
    let script = f!(r#"(call "{remote_peer_id}" ("local_service_id" "local_fn_name") ["arg"] result_name)"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::request_sent_by(some_local_peer_id);

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0.into()], expected_state);
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

    let result = checked_call_vm!(set_variable_vm, <_>::default(), script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

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

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let expected_error = UncatchableError::ShadowingIsNotAllowed(variable_name.to_string());
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

    let result = checked_call_vm!(set_variable_vm, <_>::default(), &script, "", "");
    let result = checked_call_vm!(vm, <_>::default(), script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = scalar!(
        "arg1",
        peer = vm_peer_id,
        service = service_id,
        function = function_name,
        args = ["arg1", "arg2", "arg3_value"]
    );

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1.into()], expected_state);
}

#[test]
fn test_invalid_call_service_failed() {
    let peer_id = "init_peer_id";
    let mut cid_state = ExecutionCidState::new();

    // Craft an artificial incorrect error result
    let value = json!("error");
    let value_cid = cid_state.value_tracker.record_value(value).unwrap();
    let tetraplet = SecurityTetraplet::literal_tetraplet(peer_id);
    let tetraplet_cid = cid_state.tetraplet_tracker.record_value(tetraplet).unwrap();
    let service_result_agg = ServiceResultAggregate {
        value_cid,
        argument_hash: "0000000000000".into(),
        tetraplet_cid,
    };
    let service_result_agg_cid = cid_state
        .service_result_agg_tracker
        .record_value(service_result_agg)
        .unwrap();

    let trace = ExecutionTrace::from(vec![ExecutedState::Call(CallResult::Failed(service_result_agg_cid))]);
    let data = raw_data_from_trace(trace, cid_state);

    let mut vm = create_avm(unit_call_service(), peer_id);
    let air = format!(r#"(call "{peer_id}" ("" "") [] var)"#);
    let res = vm.call(&air, vec![], data, TestRunParameters::default()).unwrap();

    assert_eq!(res.ret_code, 20014);
    assert_eq!(
        res.error_message,
        "failed to deserialize invalid type: string \"error\", expected struct CallServiceFailed",
    );
}
