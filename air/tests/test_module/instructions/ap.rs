/*
 * Copyright 2021 Fluence Labs Limited
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

use air_test_utils::prelude::*;

use fstrings::f;
use fstrings::format_args_f;
use std::collections::HashSet;

#[test]
fn ap_with_scalars() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1.$.field! scalar_2)
            )
            (call "{}" ("" "") [scalar_2])
        )
        "#,
        vm_1_peer_id, vm_2_peer_id
    );

    let result = checked_call_vm!(vm_1, "", &script, "", "");
    let result = checked_call_vm!(vm_2, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::scalar(json!({ "field": test_value })),
        executed_state::scalar_string(test_value),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_string_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap "some_string" $stream)
            (call "{}" ("" "") [$stream])
        )
        "#,
        vm_1_peer_id
    );

    let result = checked_call_vm!(vm_1, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::ap(Some(0)),
        executed_state::scalar(json!(["some_string"])),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_bool_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap true $stream)
            (call "{}" ("" "") [$stream])
        )
        "#,
        vm_1_peer_id
    );

    let result = checked_call_vm!(vm_1, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(Some(0)), executed_state::scalar(json!([true]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_number_literal() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap 100 $stream)
            (call "{}" ("" "") [$stream])
        )
        "#,
        vm_1_peer_id
    );

    let result = checked_call_vm!(vm_1, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(Some(0)), executed_state::scalar(json!([100]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_last_error() {
    let vm_1_peer_id = "vm_1_peer_id";
    let mut vm_1 = create_avm(echo_call_service(), vm_1_peer_id);

    let script = format!(
        r#"
        (seq
            (ap %last_error%.$.msg  $stream)
            (call "{}" ("" "") [$stream])
        )
        "#,
        vm_1_peer_id
    );

    let result = checked_call_vm!(vm_1, "", script, "", "");

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![executed_state::ap(Some(0)), executed_state::scalar(json!([""]))];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn ap_with_dst_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(set_variable_call_service(json!({ "field": test_value })), vm_1_peer_id);

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = f!(r#"
        (seq
            (seq
                (call "{vm_1_peer_id}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1 $stream)
            )
            (call "{vm_2_peer_id}" ("" "") [$stream])
        )
        "#);

    let result = checked_call_vm!(vm_1, "", &script, "", "");
    let result = checked_call_vm!(vm_2, "", script, "", result.data);

    let actual_trace = trace_from_result(&result);
    let expected_state = vec![
        executed_state::scalar(json!({ "field": test_value })),
        executed_state::ap(Some(0)),
        executed_state::scalar(json!([{ "field": test_value }])),
    ];

    assert_eq!(actual_trace, expected_state);
    assert!(result.next_peer_pks.is_empty());
}

#[test]
fn par_ap_behaviour() {
    let client_id = "client_id";
    let relay_id = "relay_id";
    let variable_setter_id = "variable_setter_id";
    let mut client = create_avm(unit_call_service(), client_id);
    let mut relay = create_avm(unit_call_service(), relay_id);
    let mut variable_setter = create_avm(unit_call_service(), variable_setter_id);

    let script = f!(r#"
        (par
            (call "{variable_setter_id}" ("peer" "timeout") [] join_it)
            (seq
                (par
                    (call "{relay_id}" ("peer" "timeout") [join_it] $result)
                    (ap "fast_result" $result) ;; ap doesn't affect the subtree_complete flag
                )
                (call "{client_id}" ("op" "return") [$result.$[0]])
            )
        )
        "#);

    let mut client_result_1 = checked_call_vm!(client, "", &script, "", "");
    let actual_next_peers: HashSet<_> = client_result_1.next_peer_pks.drain(..).collect();
    let expected_next_peers: HashSet<_> = maplit::hashset!(relay_id.to_string(), variable_setter_id.to_string());
    assert_eq!(actual_next_peers, expected_next_peers);

    let setter_result = checked_call_vm!(variable_setter, "", &script, "", client_result_1.data.clone());
    assert!(setter_result.next_peer_pks.is_empty());

    let relay_result = checked_call_vm!(relay, "", script, "", client_result_1.data);
    assert!(relay_result.next_peer_pks.is_empty());
}
