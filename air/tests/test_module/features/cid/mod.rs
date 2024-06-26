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

mod canon;

use air::ExecutionCidState;
use air::UncatchableError::ValueForCidNotFound;
use air_interpreter_data::ExecutionTrace;
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;

use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_missing_cid() {
    let peer_id = "peer_id";
    let mut vm = create_avm(echo_call_service(), peer_id).await;

    let air_script = r#"
       (seq
          (call "peer_id" ("service" "call1") [] x)
          (call "peer_id" ("service" "call2") []))"#;
    let mut cid_state = ExecutionCidState::new();
    let trace = vec![
        scalar_tracked!(42, cid_state, peer = peer_id, service = "service", function = "call1"),
        unused!(43, peer = peer_id, service = "service", function = "call2"),
    ];
    cid_state.service_result_agg_tracker = <_>::default();

    let missing_cid = extract_service_result_cid(&trace[0]);

    let cur_data = raw_data_from_trace(trace, cid_state);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);
    let expected_error = ValueForCidNotFound("service result aggregate", missing_cid.get_inner());
    assert!(check_error(&result, expected_error), "{:?}", result);
}

#[tokio::test]
async fn test_correct_cid() {
    let peer_id = "peer_id";
    let mut vm = create_avm(echo_call_service(), peer_id).await;

    let air_script = r#"
       (seq
          (call "peer_id" ("service" "call1") [] x)
          (call "peer_id" ("service" "call2") [] y))"#;
    let mut tracker = ExecutionCidState::new();
    let trace = vec![
        scalar_tracked!(42, tracker, peer = peer_id, service = "service", function = "call1"),
        scalar_tracked!(43, tracker, peer = peer_id, service = "service", function = "call2"),
    ];

    let cur_data = raw_data_from_trace(trace, tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);
    assert_eq!(result.ret_code, 0, "{:?}", result);
}

#[tokio::test]
async fn test_scalar_cid() {
    let vm_peer_name = "vm_peer_id";

    let annotated_air_script = format!(
        r#"
       (seq
          (call "{vm_peer_name}" ("service" "call1") [] x) ; ok="hi"
          (call "{vm_peer_name}" ("service" "call2") [] y) ; ok="ipld"
       )"#
    );
    let executor = AirScriptExecutor::from_annotated(
        TestRunParameters::from_init_peer_id(vm_peer_name),
        &annotated_air_script,
    )
    .await
    .unwrap();

    let result = executor.execute_one(vm_peer_name).await.unwrap();
    let data = data_from_result(&result);
    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        scalar_tracked!(
            "hi",
            cid_state,
            peer_name = vm_peer_name,
            service = "service..0",
            function = "call1"
        ),
        scalar_tracked!(
            "ipld",
            cid_state,
            peer_name = vm_peer_name,
            service = "service..1",
            function = "call2"
        ),
    ];

    assert_eq!(result.ret_code, 0);
    assert_eq!(data.trace, ExecutionTrace::from(expected_trace));
    assert_eq!(data.cid_info.value_store, cid_state.value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, cid_state.tetraplet_tracker.into());
    assert_eq!(
        data.cid_info.service_result_store,
        cid_state.service_result_agg_tracker.into(),
    );
}

#[tokio::test]
async fn test_stream_cid() {
    let vm_peer_name = "vm_peer_id";

    let annotated_air_script = format!(
        r#"
       (seq
          (call "{vm_peer_name}" ("service" "call1") [] $x) ; ok="hi"
          (call "{vm_peer_name}" ("service" "call2") [] $x) ; ok="ipld"
       )"#
    );
    let executor = AirScriptExecutor::from_annotated(
        TestRunParameters::from_init_peer_id(vm_peer_name),
        &annotated_air_script,
    )
    .await
    .unwrap();

    let result = executor.execute_one(vm_peer_name).await.unwrap();
    let data = data_from_result(&result);
    let mut cid_state = ExecutionCidState::new();
    let expected_trace = vec![
        stream_tracked!(
            "hi",
            0,
            cid_state,
            peer_name = vm_peer_name,
            service = "service..0",
            function = "call1"
        ),
        stream_tracked!(
            "ipld",
            1,
            cid_state,
            peer_name = vm_peer_name,
            service = "service..1",
            function = "call2"
        ),
    ];

    assert_eq!(result.ret_code, 0);
    assert_eq!(data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store, cid_state.value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, cid_state.tetraplet_tracker.into());
    assert_eq!(
        data.cid_info.service_result_store,
        cid_state.service_result_agg_tracker.into()
    );
}

#[tokio::test]
async fn test_unused_cid() {
    let vm_peer_id = "vm_peer_id";

    let annotated_air_script = format!(
        r#"
       (seq
          (call "{vm_peer_id}" ("service" "call1") []) ; ok="hi"
          (call "{vm_peer_id}" ("service" "call2") []) ; ok="ipld"
       )"#
    );
    let executor =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(vm_peer_id), &annotated_air_script)
            .await
            .unwrap();

    let result = executor.execute_one(vm_peer_id).await.unwrap();
    let data = data_from_result(&result);

    let expected_trace = vec![
        unused!("hi", peer = vm_peer_id, service = "service..0", function = "call1"),
        unused!("ipld", peer = vm_peer_id, service = "service..1", function = "call2"),
    ];

    assert_eq!(result.ret_code, 0);
    assert_eq!(data.trace, expected_trace);
    assert!(data.cid_info.value_store.is_empty());
    assert!(data.cid_info.tetraplet_store.is_empty());
    assert!(data.cid_info.service_result_store.is_empty());
}
