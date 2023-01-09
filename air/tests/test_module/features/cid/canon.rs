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

use air_interpreter_cid::CID;
use air_interpreter_data::{CidStore, CidTracker};
use air_test_framework::AirScriptExecutor;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn test_canon_ok() {
    let init_peer_id = "init_peer_id";

    let script = format!(
        r#"(seq
       (seq
           (ap 42 $stream)
           (call "{init_peer_id}" ("serv" "func") [] $stream)) ; ok = "to canon"
       (canon "{init_peer_id}" $stream #canon)
    )"#
    );

    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &script).unwrap();
    let result = executor.execute_one(init_peer_id).unwrap();
    let data = data_from_result(&result);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

    let expected_trace = vec![
        ap(0),
        stream_tracked("to canon", 1, &mut value_tracker),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "json_path": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                }, {
                    "result": "to canon",
                    "tetraplet": {
                        "function_name": "func",
                        "json_path": "",
                        "peer_pk": init_peer_id,
                        "service_id": "serv..0",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    assert_eq!(&*data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store, value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, tetraplet_tracker.into());
    assert_eq!(data.cid_info.canon_store, canon_tracker.into());
}

#[test]
fn test_canon_ok_multi() {
    let init_peer_id = "init_peer_id";
    let other_peer_id = "other_peer_id";

    let script = format!(
        r#"(seq
       (seq
           (call "{init_peer_id}" ("serv" "func") [] $stream) ; ok = "to canon"
           (call "{other_peer_id}" ("other_serv" "other_func") [] $stream) ; ok = "other"
       )
       (canon "{init_peer_id}" $stream #canon)
    )"#
    );

    let executor = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &script).unwrap();
    let _result1 = executor.execute_one(init_peer_id).unwrap();
    let _result2 = executor.execute_one(other_peer_id).unwrap();
    let result3 = executor.execute_one(init_peer_id).unwrap();
    let data = data_from_result(&result3);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

    let expected_trace = vec![
        stream_tracked("to canon", 0, &mut value_tracker),
        stream_tracked("other", 1, &mut value_tracker),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": "to canon",
                    "tetraplet": {
                        "function_name": "func",
                        "json_path": "",
                        "peer_pk": init_peer_id,
                        "service_id": "serv..0",
                    },
                }, {
                    "result": "other",
                    "tetraplet": {
                        "function_name": "other_func",
                        "json_path": "",
                        "peer_pk": other_peer_id,
                        "service_id": "other_serv..1",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    assert_eq!(&*data.trace, expected_trace);
    assert_eq!(data.cid_info.value_store.len(), 2);
    assert_eq!(data.cid_info.value_store, value_tracker.into());
    assert_eq!(data.cid_info.tetraplet_store, tetraplet_tracker.into());
    assert_eq!(data.cid_info.canon_store, canon_tracker.into());
}

#[test]
fn test_canon_value_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

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
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "json_path": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    let missing_cid = "bagaaieraondvznakk2hi3kfaixhnceatpykz7cikytniqo3lc7ogkgz2qbeq";
    let value_store: CidStore<_> = value_tracker.into();
    assert!(value_store.get(&CID::<_>::new(missing_cid)).is_some());

    let cur_data = raw_data_from_trace_with_canon(trace, CidTracker::<_>::new(), tetraplet_tracker, canon_tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    assert_eq!(result.ret_code, 20012);
    assert_eq!(
        result.error_message,
        format!("value for CID \"{missing_cid}\" not found")
    );
}

#[test]
fn test_canon_root_tetraplet_not_found() {
    let init_peer_id = "vm_peer_id";
    let other_peer_id = "other_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

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
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "json_path": "",
                        "peer_pk": other_peer_id,
                        "service_id": "",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    let missing_cid = "bagaaiera2bwoxisr5k7qlbzhxi2jmdqlgqybqgxcfwt3v652nqdo5fyc665q";
    let tetraplet_store: CidStore<_> = tetraplet_tracker.into();
    assert!(tetraplet_store.get(&CID::<_>::new(missing_cid)).is_some());

    let mut fake_tetraplet_tracker = CidTracker::<_>::new();
    fake_tetraplet_tracker
        .record_value(SecurityTetraplet::literal_tetraplet(other_peer_id))
        .unwrap();

    let cur_data = raw_data_from_trace_with_canon(trace, value_tracker, fake_tetraplet_tracker, canon_tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    assert_eq!(result.ret_code, 20012);
    assert_eq!(
        result.error_message,
        format!("tetraplet for CID \"{missing_cid}\" not found")
    );
}

#[test]
fn test_canon_tetraplet_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

    let air_script = format!(
        r#"
       (seq
          (call "peer_1" ("serv" "func") [] $stream)
          (canon "{init_peer_id}" $stream #canon))"#
    );
    let trace = vec![
        stream_tracked(42, 0, &mut value_tracker),
        canon_tracked(
            json!({
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": init_peer_id, "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "func",
                        "json_path": "",
                        "peer_pk": "peer_1",
                        "service_id": "serv",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    let missing_cid = "bagaaieracu6twiik6az3cosyzlplrscon3ek6rnu3lkjnflibphqkw6kcdiq";
    let tetraplet_store: CidStore<_> = tetraplet_tracker.into();
    assert!(tetraplet_store.get(&CID::<_>::new(missing_cid)).is_some());

    let mut fake_tetraplet_tracker = CidTracker::<_>::new();
    fake_tetraplet_tracker
        .record_value(SecurityTetraplet::literal_tetraplet(init_peer_id))
        .unwrap();

    let cur_data = raw_data_from_trace_with_canon(trace, value_tracker, fake_tetraplet_tracker, canon_tracker);
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    assert_eq!(result.ret_code, 20012);
    assert_eq!(
        result.error_message,
        format!("tetraplet for CID \"{missing_cid}\" not found"),
    );
}

#[test]
fn test_canon_agg_not_found() {
    let init_peer_id = "vm_peer_id";
    let mut vm = create_avm(echo_call_service(), init_peer_id);

    let mut value_tracker = CidTracker::<JValue>::new();
    let mut tetraplet_tracker = CidTracker::<SecurityTetraplet>::new();
    let mut canon_tracker = CidTracker::<CanonCidAggregate>::new();

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
                "tetraplet": {"function_name": "", "json_path": "", "peer_pk": "other_peer_id", "service_id": ""},
                "values": [{
                    "result": 42,
                    "tetraplet": {
                        "function_name": "",
                        "json_path": "",
                        "peer_pk": init_peer_id,
                        "service_id": "",
                    },
                }]
            }),
            &mut value_tracker,
            &mut tetraplet_tracker,
            &mut canon_tracker,
        ),
    ];

    let missing_cid = "bagaaierapp2oi35ib4iveexfswax6jcf2zhj3e2ergzjyavm6m7stlzh23ta";
    let canon_store: CidStore<_> = canon_tracker.into();
    assert!(canon_store.get(&CID::<_>::new(missing_cid)).is_some());

    let cur_data = raw_data_from_trace_with_canon(trace, value_tracker, tetraplet_tracker, <_>::default());
    let result = call_vm!(vm, <_>::default(), air_script, vec![], cur_data);

    assert_eq!(result.ret_code, 20012);
    assert_eq!(
        result.error_message,
        format!("canon aggregate for CID \"{missing_cid}\" not found")
    );
}
