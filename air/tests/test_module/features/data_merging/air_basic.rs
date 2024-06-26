/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use air::ExecutionCidState;
use air_interpreter_data::ExecutionTrace;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
async fn seq_par_call() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

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
async fn par_par_call() {
    let vm_peer_id = "some_peer_id";
    let remote_peer_id = "remote_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

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
