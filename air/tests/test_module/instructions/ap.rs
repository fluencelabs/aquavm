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

use air_test_utils::*;

#[test]
fn ap_with_scalars() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(set_variable_call_service(r#"{"field": "scalar_2"}"#), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_string_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar.$.field! scalar_2)
            )
            (call "{}" ("" "") [scalar_2] scalar_3)
        )
        "#,
        vm_1_peer_id, vm_2_peer_id
    );

    let result = checked_call_vm!(vm_1, "", &script, "", "");
    let result = checked_call_vm!(vm_2, "", script, "", result.data);

    print_trace(&result, "result trace");

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string("test");

    assert_eq!(actual_trace.len(), 1);
    assert_eq!(actual_trace[0], expected_state);
    assert!(result.next_peer_pks.is_empty());
}

// Check that specifying remote peer id in call will result its appearing in next_peer_pks.
#[test]
fn remote_peer_id_call() {
    let some_local_peer_id = String::from("some_local_peer_id");
    let mut vm = create_avm(echo_string_call_service(), &some_local_peer_id);

    let remote_peer_id = String::from("some_remote_peer_id");
    let script = format!(
        r#"(call "{}" ("local_service_id" "local_fn_name") ["arg"] result_name)"#,
        remote_peer_id
    );

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
    let mut set_variable_vm = create_avm(set_variable_call_service(r#""remote_peer_id""#), "set_variable");

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
    let mut vm = create_avm(unit_call_service(), "some_peer_id");

    let script = r#"
            (seq
                (call "some_peer_id" ("some_service_id" "local_fn_name") [] modules)
                (call "some_peer_id" ("some_service_id" "local_fn_name") [] modules)
            )
        "#;

    let result = call_vm!(vm, "asd", script, "", "");

    assert_eq!(result.ret_code, 1005);
    assert!(result.next_peer_pks.is_empty());
}

// Check that string literals can be used as call parameters.
#[test]
fn string_parameters() {
    let call_service: CallServiceClosure = Box::new(|args| -> Option<IValue> {
        let arg = match &args.function_args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(arg.clone())]).unwrap(),
        ))
    });

    let vm_peer_id = String::from("A");
    let mut vm = create_avm(call_service, vm_peer_id.clone());

    let set_variable_vm_peer_id = String::from("set_variable");
    let mut set_variable_vm = create_avm(
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

    let result = checked_call_vm!(set_variable_vm, "asd", &script, "", "");
    let result = checked_call_vm!(vm, "asd", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = executed_state::scalar_string_array(vec!["arg1", "arg2", "arg3_value"]);

    assert_eq!(actual_trace.len(), 2);
    assert_eq!(actual_trace[1], expected_state);
}
