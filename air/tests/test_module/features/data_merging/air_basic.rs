/*
 * Copyright 2022 Fluence Labs Limited
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
use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
fn seq_par_call() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = format!(
        r#"
        (seq 
            (par 
                (call "{vm_peer_id}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "{vm_peer_id}" ("local_service_id" "local_fn_name") [] result_2)
        )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);

    let unit_call_service_result = "result from unit_call_service";
    let expected_trace = vec![
        executed_state::par(1, 1),
        scalar!(
            unit_call_service_result,
            peer = vm_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
        executed_state::request_sent_by(vm_peer_id),
        scalar!(
            unit_call_service_result,
            peer = vm_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ];

    assert_eq!(actual_trace, ExecutionTrace::from(expected_trace));
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}

#[tokio::test]
fn par_par_call() {
    let vm_peer_id = "some_peer_id";
    let remote_peer_id = "remote_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = format!(
        r#"
        (par
            (par
                (call "{vm_peer_id}" ("local_service_id" "local_fn_name") [] result_1)
                (call "{remote_peer_id}" ("service_id" "fn_name") [] g)
            )
            (call "{vm_peer_id}" ("local_service_id" "local_fn_name") [] result_2)
        )"#
    );

    let result = checked_call_vm!(vm, <_>::default(), script, "", "");
    let actual_trace = trace_from_result(&result);

    let unit_call_service_result = "result from unit_call_service";
    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        executed_state::par(3, 1),
        executed_state::par(1, 1),
        scalar_tracked!(
            unit_call_service_result,
            cid_state,
            peer = vm_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
        executed_state::request_sent_by(vm_peer_id),
        scalar_tracked!(
            unit_call_service_result,
            cid_state,
            peer = vm_peer_id,
            service = "local_service_id",
            function = "local_fn_name"
        ),
    ];

    assert_eq!(actual_trace, ExecutionTrace::from(expected_trace));
    assert_eq!(result.next_peer_pks, vec![String::from("remote_peer_id")]);
}
