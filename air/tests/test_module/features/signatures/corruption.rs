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

/// This test module asserts CID store verification functionality:
/// values forged in the CID stores.
use air::ExecutionCidState;
use air::PreparationError;
use air_interpreter_cid::CidVerificationError;
use air_interpreter_signatures::PeerCidTracker;
use air_interpreter_signatures::PublicKey;
use air_interpreter_signatures::SignatureStore;
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use pretty_assertions::assert_eq;
use semver::Version;

#[tokio::test]
async fn test_attack_replace_value() {
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
        if val.as_str().unwrap() == json!("alice").to_string() {
            *val = json!("evil").to_string().into();
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                // fragile: it is OK if this exact string changes on compiler upgrade
                type_name: "air_interpreter_data::raw_value::RawValue",
                cid_repr: "bagaaihrayhxgqijfajraxivb7hxwshhbsdqk4j5zyqypb54zggmn5v7mmwxq".into(),
            }
            .into()
        )
    );
}

#[tokio::test]
async fn test_attack_replace_tetraplet() {
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                type_name: "marine_call_parameters::SecurityTetraplet",
                cid_repr: "bagaaihraxnms7hna6c27qhgfzhyayz62y2q2dxc4dwriq33tdilonmq4ruoq".into(),
            }
            .into()
        )
    );
}

#[tokio::test]
async fn test_attack_replace_call_result() {
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
    let alice_trace_1_cid = extract_service_result_cid(&alice_trace_1).get_inner();

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
        if &*cid == &*alice_trace_1_cid {
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                type_name: "air_interpreter_data::executed_state::ServiceResultCidAggregate",
                cid_repr: "bagaaihradr2m7mlsvqhtnzszpuifqgiytee6zpyxyfzxbuqcmf23fgmbemqq".into(),
            }
            .into()
        )
    );
}

#[tokio::test]
async fn test_attack_replace_canon_value() {
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
            "tetraplet": {"peer_pk": &alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
            "values": [{
                "tetraplet": {"peer_pk": &alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                type_name: "air_interpreter_data::executed_state::CanonCidAggregate",
                cid_repr: "bagaaihracce5ggyu3cbxm4xh35mjlmb7qb3xltlwoqqas62e2yftii4x4msq".into(),
            }
            .into()
        )
    );
}

#[tokio::test]
async fn test_attack_replace_canon_result_values() {
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
            "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
            "values": [{
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
                "result": 1,
                "provenance": Provenance::literal(),
            }, {
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                type_name: "air_interpreter_data::executed_state::CanonResultCidAggregate",
                cid_repr: "bagaaihraxsxqmnfevwk6briizagprfikpm4x73mdf626mm5xju2f33vp7c7q".into(),
            }
            .into()
        )
    );
}

#[tokio::test]
async fn test_attack_replace_canon_result_tetraplet() {
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
            "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
            "values": [{
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
                "result": 1,
                "provenance": Provenance::literal(),
            }, {
                "tetraplet": {"peer_pk": alice_peer_id, "service_id": "", "function_name": "", "lens": ""},
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

    let mallory_data = InterpreterDataEnvelope::from_execution_result(
        mallory_trace.into(),
        serde_json::from_value(mallory_cid_info).unwrap(),
        signature_store,
        0,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id).await;
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = mallory_data.serialize().unwrap();
    let res = bob_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::CidStoreVerificationError(
            CidVerificationError::ValueMismatch {
                type_name: "air_interpreter_data::executed_state::CanonResultCidAggregate",
                cid_repr: "bagaaihraxsxqmnfevwk6briizagprfikpm4x73mdf626mm5xju2f33vp7c7q".into(),
            }
            .into()
        )
    );
}
