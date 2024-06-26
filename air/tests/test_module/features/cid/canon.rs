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
use air::UncatchableError::ValueForCidNotFound;
use air_interpreter_cid::CID;
use air_interpreter_data::{CidStore, CidTracker};
use air_test_framework::AirScriptExecutor;
use air_test_utils::key_utils::at;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_canon_ok() {
    let init_peer_name = "init_peer_id";

    let script = format!(
        r#"(seq
       (seq
           (ap 42 $stream)
           (call "{init_peer_name}" ("serv" "func") [] $stream)) ; ok = "to canon"
       (canon "{init_peer_name}" $stream #canon)
    )"#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .await
        .unwrap();
    let result = executor.execute_one(init_peer_name).await.unwrap();
    let data = data_from_result(&result);

    let mut cid_state = ExecutionCidState::new();

    let init_peer_id = at(init_peer_name);

    let stream_exec_state = stream_tracked!(
        "to canon",
        1,
        cid_state,
        peer_name = init_peer_name,
        service = "serv..0",
        function = "func"
    );

    let service_result_cid = extract_service_result_cid(&stream_exec_state);

    let expected_trace = vec![
        ap(0),
        stream_exec_state,
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "lens": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                    "provenance": Provenance::literal(),
                }, {
                    "result": "to canon",
                    "tetraplet": {
                        "function_name": "func",
                        "lens": "",
                        "peer_pk": init_peer_id,
                        "service_id": "serv..0",
                    },
                    "provenance": Provenance::service_result(service_result_cid),
                }]
            }),
            &mut cid_state,
        ),
    ];

    assert_eq!(&*data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store, cid_state.value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, cid_state.tetraplet_tracker.into());
    assert_eq!(
        data.cid_info.canon_element_store,
        cid_state.canon_element_tracker.into()
    );
    assert_eq!(
        data.cid_info.service_result_store,
        cid_state.service_result_agg_tracker.into()
    );
}

#[tokio::test]
async fn test_canon_ok_multi() {
    let init_peer_name = "init_peer_id";
    let other_peer_name = "other_peer_id";

    let script = format!(
        r#"(seq
       (seq
           (call "{init_peer_name}" ("serv" "func") [] $stream) ; ok = "to canon"
           (call "{other_peer_name}" ("other_serv" "other_func") [] $stream) ; ok = "other"
       )
       (canon "{init_peer_name}" $stream #canon)
    )"#
    );

    let executor = AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &script)
        .await
        .unwrap();
    let _result1 = executor.execute_one(init_peer_name).await.unwrap();
    let _result2 = executor.execute_one(other_peer_name).await.unwrap();
    let result3 = executor.execute_one(init_peer_name).await.unwrap();
    let data = data_from_result(&result3);

    let init_peer_id = at(init_peer_name);
    let other_peer_id = at(other_peer_name);

    let mut cid_state = ExecutionCidState::new();

    let stream_state_1 = stream_tracked!(
        "to canon",
        0,
        cid_state,
        peer_name = init_peer_name,
        service = "serv..0",
        function = "func"
    );
    let service_result_cid_1 = extract_service_result_cid(&stream_state_1);

    let stream_state_2 = stream_tracked!(
        "other",
        1,
        cid_state,
        peer_name = other_peer_name,
        service = "other_serv..1",
        function = "other_func"
    );
    let service_result_cid_2 = extract_service_result_cid(&stream_state_2);

    let expected_trace = vec![
        stream_state_1,
        stream_state_2,
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": "to canon",
                    "tetraplet": {
                        "function_name": "func",
                        "lens": "",
                        "peer_pk": init_peer_id,
                        "service_id": "serv..0",
                    },
                    "provenance": Provenance::service_result(service_result_cid_1),
                }, {
                    "result": "other",
                    "tetraplet": {
                        "function_name": "other_func",
                        "lens": "",
                        "peer_pk": other_peer_id,
                        "service_id": "other_serv..1",
                    },
                    "provenance": Provenance::service_result(service_result_cid_2),
                }]
            }),
            &mut cid_state,
        ),
    ];

    assert_eq!(&*data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store.len(), 2);
    assert_eq!(data.cid_info.value_store, cid_state.value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, cid_state.tetraplet_tracker.into());
    assert_eq!(
        data.cid_info.canon_element_store,
        cid_state.canon_element_tracker.into()
    );
    assert_eq!(
        data.cid_info.service_result_store,
        cid_state.service_result_agg_tracker.into()
    );
}

#[tokio::test]
async fn test_canon_value_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id).await;

    let mut cid_state = ExecutionCidState::new();

    let air_script = format!(
        r#"
       (seq
          (ap 42 $stream)
          (canon "{init_peer_id}" $stream #canon))"#
    );
    let trace = vec![
        ap(0),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "lens": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                }]
            }),
            &mut cid_state,
        ),
    ];

    let missing_cid = "bagaaihra3ijwi5gxk5odex3qfo32u5prci4giaz4ysel67m4a5hk3l432djq";
    let value_store: CidStore<_> = cid_state.value_tracker.into();
    assert!(
        value_store.get(&CID::<_>::new(missing_cid)).is_some(),
        "{:#?}",
        value_store
    );

    // Override with fake data.
    cid_state.value_tracker = CidTracker::<_>::new();
    let cur_data = raw_data_from_trace_with_canon(trace, cid_state);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);
    let expected_error = ValueForCidNotFound("value", missing_cid.into());
    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn test_canon_root_tetraplet_not_found() {
    let init_peer_id = "vm_peer_id";
    let other_peer_id = "other_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id).await;

    let mut cid_state = ExecutionCidState::new();

    let air_script = format!(
        r#"
       (seq
          (ap 42 $stream)
          (canon "{other_peer_id}" $stream #canon))"#
    );
    let trace = vec![
        ap(0),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "lens": "",
                        "peer_pk": other_peer_id,
                        "service_id": "",
                    },
                }]
            }),
            &mut cid_state,
        ),
    ];

    let missing_cid = "bagaaihrays67nve662j4pn5jdqquxlqqi5vpisgs72n4tmnrqbbnah3t5ola";
    let tetraplet_store: CidStore<_> = cid_state.tetraplet_tracker.into();
    assert!(
        tetraplet_store.get(&CID::<_>::new(missing_cid)).is_some(),
        "{:#?}",
        tetraplet_store
    );

    let mut fake_tetraplet_tracker = CidTracker::<_>::new();
    fake_tetraplet_tracker
        .track_value(SecurityTetraplet::literal_tetraplet(other_peer_id))
        .unwrap();

    cid_state.tetraplet_tracker = fake_tetraplet_tracker;

    let cur_data = raw_data_from_trace_with_canon(trace, cid_state);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    let expected_error = ValueForCidNotFound("tetraplet", missing_cid.into());
    assert_error_eq!(&result, expected_error);
}

#[tokio::test]
async fn test_canon_tetraplet_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id).await;

    let mut cid_state = ExecutionCidState::new();

    let air_script = format!(
        r#"
       (seq
          (call "peer_1" ("serv" "func") [] $stream)
          (canon "{init_peer_id}" $stream #canon))"#
    );
    let trace = vec![
        stream_tracked!(
            42,
            0,
            cid_state,
            peer = "peer_1",
            service = "serv..0",
            function = "func"
        ),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "func",
                        "lens": "",
                        "peer_pk": "peer_1",
                        "service_id": "serv..0",
                    },
                }]
            }),
            &mut cid_state,
        ),
    ];
    let missing_cid = "bagaaihramktnmwzskmyxlah5zyownsfxv4vt7wf2ypzwvrygb2x7o72vpfyq";
    let tetraplet_store: CidStore<_> = cid_state.tetraplet_tracker.into();
    assert!(
        tetraplet_store.get(&CID::<_>::new(missing_cid)).is_some(),
        "{:#?}",
        tetraplet_store
    );

    let mut fake_tetraplet_tracker = CidTracker::<_>::new();
    fake_tetraplet_tracker
        .track_value(SecurityTetraplet::literal_tetraplet(init_peer_id))
        .unwrap();

    cid_state.tetraplet_tracker = fake_tetraplet_tracker;
    let cur_data = raw_data_from_trace_with_canon(trace, cid_state);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    let expected_error = ValueForCidNotFound("tetraplet", missing_cid.into());
    assert_error_eq!(&result, expected_error);
}

#[tokio::test]
async fn test_canon_agg_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id).await;

    let mut cid_state = ExecutionCidState::new();

    let air_script = format!(
        r#"
       (seq
          (ap 42 $stream)
          (canon "other_peer_id" $stream #canon))"#
    );
    let trace = vec![
        ap(0),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "lens": "", "peer_pk": "other_peer_id", "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "lens": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                    "provenance": Provenance::literal(),
                }]
            }),
            &mut cid_state,
        ),
    ];

    let missing_cid = "bagaaihrad3w3ebwqwgzoxyvdyq7wgxeawv2i6olczg6mnivu6fnwwm4m42oq";
    let canon_element_store: CidStore<_> = cid_state.canon_element_tracker.into();
    assert!(
        canon_element_store.get(&CID::<_>::new(missing_cid)).is_some(),
        "{:#?}",
        canon_element_store
    );

    // Fake data
    cid_state.canon_element_tracker = <_>::default();
    let cur_data = raw_data_from_trace_with_canon(trace, cid_state);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    let expected_error = ValueForCidNotFound("canon aggregate", missing_cid.into());
    assert!(check_error(&result, expected_error));
}
