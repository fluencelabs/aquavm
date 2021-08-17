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
use serde_json::json;

#[test]
fn ap_with_scalars() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(
        set_variable_call_service(json!({ "field": test_value }).to_string()),
        vm_1_peer_id,
    );

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_string_call_service(), vm_2_peer_id);

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
fn ap_with_dst_stream() {
    let vm_1_peer_id = "vm_1_peer_id";
    let test_value = "scalar_2";
    let mut vm_1 = create_avm(
        set_variable_call_service(json!({ "field": test_value }).to_string()),
        vm_1_peer_id,
    );

    let vm_2_peer_id = "vm_2_peer_id";
    let mut vm_2 = create_avm(echo_call_service(), vm_2_peer_id);

    let script = format!(
        r#"
        (seq
            (seq
                (call "{}" ("" "") ["scalar_1_result"] scalar_1)
                (ap scalar_1 $stream)
            )
            (call "{}" ("" "") [$stream])
        )
        "#,
        vm_1_peer_id, vm_2_peer_id
    );

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
