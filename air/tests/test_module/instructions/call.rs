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

use air::ExecutionCidState;
use air::UncatchableError;
use air_test_framework::AirScriptExecutor;
use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;
use futures::FutureExt;

// Check that %init_peer_id% alias works correctly (by comparing result with it and explicit peer id).
// Additionally, check that empty string for data does the same as empty call path.
#[tokio::test]
async fn current_peer_id_call() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

    let service_id = "local_service_id";
    let function_name = "local_fn_name";
    let script = format!(
        r#"
               (call %init_peer_id% ("{service_id}" "{function_name}") [] result_name)
            "#
    );

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

    let script = format!(
        r#"
               (call "{vm_peer_id}" ("{service_id}" "{function_name}") [] result_name)
            "#
    );

    let result = checked_call_vm!(vm, <_>::default(), script.clone(), "", "");

    // test that empty string for data works
    let result_with_empty_string = checked_call_vm!(vm, <_>::default(), script, "", "");
    assert_eq!(result_with_empty_string, result);
}

#[tokio::test]
async fn call_with_timestamp() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id).await;

    let script = r#"(call %init_peer_id% ("" "") [%timestamp%] result_name)"#;

    let test_params = TestRunParameters::new(vm_peer_id, 1337, 0, "");
    let result = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(
        test_params.timestamp,
        peer = vm_peer_id,
        args = [test_params.timestamp]
    )];

    assert_eq!(actual_trace, expected_trace);
}

#[tokio::test]
async fn call_with_ttl() {
    let vm_peer_id = "test_peer_id";
    let mut vm = create_avm(echo_call_service(), vm_peer_id).await;

    let script = format!(r#"(call "{vm_peer_id}" ("" "") [%ttl%] result_name)"#);

    let test_params = TestRunParameters::from_ttl(1337);
    let result = checked_call_vm!(vm, test_params.clone(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_trace = vec![scalar!(test_params.ttl, peer = vm_peer_id, args = [test_params.ttl])];

    assert_eq!(actual_trace, expected_trace);
}

// Check that specifying remote peer id in call will result its appearing in next_peer_pks.
#[tokio::test]
async fn remote_peer_id_call() {
    let some_local_peer_id = String::from("some_local_peer_id");
    let mut vm = create_avm(echo_call_service(), &some_local_peer_id).await;

    let remote_peer_id = String::from("some_remote_peer_id");
    let script = format!(r#"(call "{remote_peer_id}" ("local_service_id" "local_fn_name") ["arg"] result_name)"#);

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::request_sent_by(some_local_peer_id);

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0.into()], expected_state);
    assert_eq!(result.next_peer_pks, vec![remote_peer_id]);
}

// Check that setting variables works as expected.
#[tokio::test]
async fn variables() {
    let mut vm = create_avm(unit_call_service(), "remote_peer_id").await;
    let mut set_variable_vm = create_avm(set_variable_call_service(json!("remote_peer_id")), "set_variable").await;

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
#[tokio::test]
async fn duplicate_variables() {
    let peer_id = "peer_id";
    let mut vm = create_avm(unit_call_service(), peer_id).await;

    let variable_name = "modules";
    let script = format!(
        r#"
            (seq
                (call "{peer_id}" ("some_service_id" "local_fn_name") [] {variable_name})
                (call "{peer_id}" ("some_service_id" "local_fn_name") [] {variable_name})
            )
        "#
    );

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let expected_error = UncatchableError::ShadowingIsNotAllowed(variable_name.to_string());
    assert!(check_error(&result, expected_error));
    assert!(result.next_peer_pks.is_empty());
}

// Check that string literals can be used as call parameters.
#[tokio::test]
async fn string_parameters() {
    let call_service: CallServiceClosure =
        Box::new(|mut params|  {
            let result = CallServiceResult::ok(params.arguments.remove(0));
            async move { result }.boxed_local()
        });

    let vm_peer_id = "A";
    let mut vm = create_avm(call_service, vm_peer_id).await;

    let set_variable_vm_peer_id = "set_variable";
    let mut set_variable_vm = create_avm(set_variable_call_service(json!("arg3_value")), set_variable_vm_peer_id).await;

    let service_id = "some_service_id";
    let function_name = "local_fn_name";
    let script = format!(
        r#"
            (seq
                (call "{set_variable_vm_peer_id}" ("{service_id}" "{function_name}") [] arg3)
                (call "{vm_peer_id}" ("{service_id}" "{function_name}") ["arg1" "arg2" arg3] result)
            )
        "#
    );

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

#[tokio::test]
async fn call_canon_stream_map_arg() {
    let vm_1_peer_name = "vm_1_peer_id";
    let vm_1_peer_id = at(vm_1_peer_name);

    let script = format!(
        r#"
        (seq
            (seq
                (ap ("key" "value1") %map)
                (ap (-42 "value2") %map)
            )
            (seq
                (canon "{vm_1_peer_name}" %map #%canon_map)
                (call "{vm_1_peer_name}" ("m" "f") [#%canon_map] scalar) ; behaviour = echo
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_name), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let map_value_1 = json!({"key": "key", "value": "value1"});
    let map_value_2 = json!({"key": -42, "value": "value2"});

    let map_value = json!({
        "-42": ["value2"],
        "key": ["value1"],
    });

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
            {
                "result": map_value_1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_2,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            map_value.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value]
        ),
    ];

    assert_eq!(&*actual_trace, expected_trace,);
}

// WIP add negative
#[tokio::test]
async fn call_peer_id_from_canon_stream_map() {
    let vm_1_peer_name = "vm_1_peer_id";
    let vm_1_peer_id = at(vm_1_peer_name);
    let script = format!(
        r#"
        (seq
            (seq
                (ap ("peerid" @"{vm_1_peer_name}") %map)
                (ap (-42 "value2") %map)
            )
            (seq
                (canon "{vm_1_peer_name}" %map #%canon_map)
                (call #%canon_map.$.peerid.[0] ("m" "f") [#%canon_map] scalar) ; behaviour = echo
            )
        )
        "#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_1_peer_name), &script)
        .await
        .expect("invalid test AIR script");
    let result = executor.execute_all(vm_1_peer_name).await.unwrap();
    let actual_trace = trace_from_result(&result.last().unwrap());

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});

    let map_value_1 = json!({"key": "peerid", "value": vm_1_peer_id});
    let map_value_2 = json!({"key": -42, "value": "value2"});
    let map_value = json!({
        "-42": ["value2"],
        "peerid": [vm_1_peer_id],
    });

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
            {
                "result": map_value_1,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_2,
                "tetraplet": tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            map_value.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m..0",
            function = "f",
            args = [map_value]
        ),
    ];

    assert_eq!(&*actual_trace, expected_trace,);
}

#[tokio::test]
async fn call_module_func_from_canon_stream_map() {
    let vm_1_peer_id = "vm_1_peer_id";
    // There is a bug in testing framework that disallows lenses to be a source of a module name in
    // a call triplet.
    let mut vm = create_avm(echo_call_service(), vm_1_peer_id).await;

    let script = format!(
        r#"
        (seq
            (seq
                (ap ("module" "m") %map)
                (ap ("function" "f") %map)
            )
            (seq
                (canon "{vm_1_peer_id}" %map #%canon_map)
                (call "{vm_1_peer_id}" (#%canon_map.$.module.[0] #%canon_map.$.function.[0]) [#%canon_map] scalar) ; behaviour = echo
            )
        )
        "#
    );

    let result = call_vm!(vm, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);

    let mut cid_tracker: ExecutionCidState = ExecutionCidState::new();
    let tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": vm_1_peer_id, "service_id": ""});
    let empty_tetraplet = json!({"function_name": "", "json_path": "", "peer_pk": "", "service_id": ""});

    let map_value_1 = json!({"key": "module", "value": "m"});
    let map_value_2 = json!({"key": "function", "value": "f"});
    let map_value = json!({
        "function": ["f"],
        "module": ["m"],
    });

    let expected_trace: Vec<ExecutedState> = vec![
        executed_state::ap(0),
        executed_state::ap(0),
        canon_tracked(
            json!({"tetraplet": tetraplet,
            "values": [
            {
                "result": map_value_1,
                "tetraplet": empty_tetraplet,
                "provenance": Provenance::Literal,
            },
            {
                "result": map_value_2,
                "tetraplet": empty_tetraplet,
                "provenance": Provenance::Literal,
            },
            ]}),
            &mut cid_tracker,
        ),
        scalar_tracked!(
            map_value.clone(),
            cid_tracker,
            peer = vm_1_peer_id,
            service = "m",
            function = "f",
            args = [map_value]
        ),
    ];

    assert_eq!(
        actual_trace, expected_trace,
        "{:#?}\n {:#?}",
        actual_trace, expected_trace
    );
}
