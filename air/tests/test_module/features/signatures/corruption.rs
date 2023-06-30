/*
 * Copyright 2023 Fluence Labs Limited
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

/// This test module asserts CID store verification functionality:
/// values forged in the CID stores.
use air::ExecutionCidState;
use air::PreparationError;
use air_interpreter_signatures::PeerCidTracker;
use air_interpreter_signatures::PublicKey;
use air_interpreter_signatures::SignatureStore;
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;
use semver::Version;

#[test]
fn test_attack_replace_value() {
    // Bob gets a trace where call result value is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{bob_peer_id}" ("" "") [] z))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let mallory_trace = vec![
        scalar_tracked!("alice", &mut mallory_cid_state, peer = &alice_peer_id),
        scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id),
    ];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;
    for (_cid, val) in mallory_cid_info["value_store"].as_object_mut().unwrap().iter_mut() {
        if *val == "alice" {
            *val = "evil".into();
            cnt += 1;
        }
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_service_result_cid(&mallory_trace[0]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[1]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "serde_json::value::Value",
            cid_repr: "bagaaierabjifaczkgq2745dsq57lelki2r5cfduunmfzsgvxiavi2ahwwmwq".to_owned(),
        })
    );
}

#[test]
fn test_attack_replace_tetraplet() {
    // Bob gets a trace where call result tetraplet is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{bob_peer_id}" ("" "") [] z))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let mallory_trace = vec![
        scalar_tracked!("alice", &mut mallory_cid_state, peer = &alice_peer_id),
        scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id),
    ];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;
    for (_cid, tetraplet_val) in mallory_cid_info["tetraplet_store"].as_object_mut().unwrap().iter_mut() {
        if tetraplet_val["peer_pk"] == alice_peer_id {
            tetraplet_val["service_id"] = json!("evil");
            cnt += 1;
        }
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_service_result_cid(&mallory_trace[0]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[1]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "polyplets::tetraplet::SecurityTetraplet",
            cid_repr: "bagaaierapisclqfeq36psuo6uxiazvcash32pndayqlwxrqchii2ykxerfba".to_owned(),
        })
    );
}

#[test]
fn test_attack_replace_call_result() {
    // Bob gets a trace where call result is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{bob_peer_id}" ("" "") [] z))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let alice_trace_1 = scalar_tracked!("alice", &mut mallory_cid_state, peer = &alice_peer_id);
    let alice_trace_1_cid = (*extract_service_result_cid(&alice_trace_1)).clone().into_inner();

    let mallory_trace = vec![
        alice_trace_1,
        scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id),
    ];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;
    for (cid, service_cid_val) in mallory_cid_info["service_result_store"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
    {
        if *cid == alice_trace_1_cid {
            service_cid_val["argument_hash"] = "42".into();
            cnt += 1;
        }
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_service_result_cid(&mallory_trace[0]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[1]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "air_interpreter_data::executed_state::ServiceResultCidAggregate",
            cid_repr: "bagaaierarbji6ebokx3pantdp6xg2l57bhdj7pmlydwe2wnbd6fdkatg7xka".to_owned(),
        })
    );
}

#[test]
fn test_attack_replace_canon_value() {
    // Bob gets a trace where canon value is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
    (seq
       (seq
          (ap 1 $s)
          (canon "{alice_peer_id}" $s #c))
       (seq
          (call "{mallory_peer_id}" ("" "") [] x)
          (call "{bob_peer_id}" ("" "") [])))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let alice_canon_cid = canon_tracked(
        json!({
            "tetraplet": {"peer_pk": &alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
            "values": [{
                "tetraplet": {"peer_pk": &alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
                "result": 1,
                "provenance": Provenance::literal(),
            }]
        }),
        &mut mallory_cid_state,
    );
    let mallory_call_result_state = scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id);
    let mallory_call_result_cid = extract_service_result_cid(&mallory_call_result_state);
    let mallory_trace = vec![ap(0), ap(0), alice_canon_cid, mallory_call_result_state];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;
    for (_cid, canon_element) in mallory_cid_info["canon_element_store"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
    {
        canon_element["provenance"] = json!(Provenance::service_result(mallory_call_result_cid.clone()));
        cnt += 1;
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_canon_result_cid(&mallory_trace[2]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[3]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "air_interpreter_data::executed_state::CanonCidAggregate",
            cid_repr: "bagaaierayrb7yu6tvdofr3d7tvuzx7fb3uve27rqty4ckzy7ox66oicuhjjq".to_owned(),
        })
    );
}

#[test]
fn test_attack_replace_canon_result_values() {
    // Bob gets a trace where canon result is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
    (seq
       (seq
          (seq
             (ap 1 $s)
             (ap 2 $s))
          (canon "{alice_peer_id}" $s #c))
       (seq
          (call "{mallory_peer_id}" ("" "") [] x)
          (call "{bob_peer_id}" ("" "") [])))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let alice_canon_cid = canon_tracked(
        json!({
            "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
            "values": [{
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
                "result": 1,
                "provenance": Provenance::literal(),
            }, {
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
                "result": 2,
                "provenance": Provenance::literal(),
            }]
        }),
        &mut mallory_cid_state,
    );
    let mallory_trace = vec![
        ap(0),
        ap(0),
        alice_canon_cid,
        scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id),
    ];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;
    for (_cid, canon_result) in mallory_cid_info["canon_result_store"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
    {
        canon_result["values"].as_array_mut().unwrap().pop();
        cnt += 1;
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_canon_result_cid(&mallory_trace[2]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[3]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "air_interpreter_data::executed_state::CanonResultCidAggregate",
            cid_repr: "bagaaieratezrhuyz2eprlmiidxywv6ir2tmswlxycad37noykg3p5oxhs5tq".to_owned(),
        })
    );
}

#[test]
fn test_attack_replace_canon_result_tetraplet() {
    // Bob gets a trace where canon result is edited by Mallory.
    let alice_peer_id = "alice";
    let bob_peer_id = "bob";
    let mallory_peer_id = "mallory";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
    (seq
       (seq
          (seq
             (ap 1 $s)
             (ap 2 $s))
          (canon "{alice_peer_id}" $s #c))
       (seq
          (call "{mallory_peer_id}" ("" "") [] x)
          (call "{bob_peer_id}" ("" "") [])))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::new();
    let alice_canon_cid = canon_tracked(
        json!({
            "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
            "values": [{
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
                "result": 1,
                "provenance": Provenance::literal(),
            }, {
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "json_path": ""},
                "result": 2,
                "provenance": Provenance::literal(),
            }]
        }),
        &mut mallory_cid_state,
    );
    let mallory_trace = vec![
        ap(0),
        ap(0),
        alice_canon_cid,
        scalar_tracked!("mallory", &mut mallory_cid_state, peer = &mallory_peer_id),
    ];

    let mut mallory_cid_info = serde_json::to_value::<CidInfo>(mallory_cid_state.into()).unwrap();
    let mut cnt = 0;

    let mut fake_cid = None;
    for (tetraplet_cid, tetraplet) in mallory_cid_info["tetraplet_store"].as_object().unwrap() {
        if tetraplet["peer_pk"] == mallory_peer_id {
            fake_cid = Some(tetraplet_cid.clone());
        }
    }
    assert!(fake_cid.is_some(), "test is invalid");
    for (_cid, canon_result) in mallory_cid_info["canon_result_store"].as_object_mut().unwrap() {
        canon_result["tetraplet"] = json!(fake_cid.clone().unwrap());
        cnt += 1;
    }
    assert_eq!(cnt, 1, "test validity failed");

    let mut signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_canon_result_cid(&mallory_trace[2]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[3]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(CidStoreVerificationError::MismatchError {
            type_name: "air_interpreter_data::executed_state::CanonResultCidAggregate",
            cid_repr: "bagaaieratezrhuyz2eprlmiidxywv6ir2tmswlxycad37noykg3p5oxhs5tq".to_owned(),
        })
    );
}