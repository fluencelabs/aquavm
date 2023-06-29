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

use air::{ExecutionCidState, PreparationError};
use air_interpreter_signatures::{SignatureStore, PeerCidTracker, PublicKey};
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use semver::Version;

#[test]
fn test_attack_injection_current_peer_scalar() {
    // injecting a value that arrives to peer who does the next step
    let (alice_keypair, alice_peer_id) = derive_dummy_keypair("alice_peer");
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair("mallory_peer");

    let air_script = format!(
        r#"
    (seq
       (seq
          (call "{alice_peer_id}" ("" "") [] x)
          (call "{mallory_peer_id}" ("" "") [] y))
       (call "{alice_peer_id}" ("" "") [] z))
    "#
    );

    let mut alice_cid_state = ExecutionCidState::new();
    let mut alice_signature_tracker = PeerCidTracker::new(alice_peer_id.clone());
    let mut alice_signature_store = SignatureStore::new();

    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_state, peer = &alice_peer_id);
    alice_signature_tracker.register(&*alice_peer_id, &extract_service_result_cid(&alice_call_1));
    let alice_trace = vec![alice_call_1.clone()];
    let alice_signature = alice_signature_tracker.gen_signature("", &alice_keypair).unwrap();
    alice_signature_store.put(alice_keypair.public().into(), alice_signature);

    let mut mallory_cid_state = alice_cid_state.clone();
    let mut mallory_signature_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    let mut mallory_signature_store = alice_signature_store.clone();

    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = scalar_tracked!("fake result", &mut mallory_cid_state, peer = &alice_peer_id);
    mallory_signature_tracker.register(&*mallory_peer_id, &extract_service_result_cid(&mallory_call_2));
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];
    let mallory_signature = mallory_signature_tracker.gen_signature("", &mallory_keypair).unwrap();
    mallory_signature_store.put(mallory_keypair.public().into(), mallory_signature);

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_state.into(),
        alice_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_state.into(),
        mallory_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm_with_key::<NativeAirRunner>(alice_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = serde_json::to_vec(&alice_data).unwrap();
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = alice_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .unwrap();
    assert_ne!(res.ret_code, 0);
}

#[test]
fn test_attack_injection_current_peer_stream() {
    // injecting a value that arrives to peer who does the next step
    let alice_peer_id = "alice_peer";
    let mallory_peer_id = "mallory_peer";

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
       (call "{alice_peer_id}" ("" "") [] $z))
    "#
    );

    let mut alice_cid_state = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_state, peer = &alice_peer_id);

    let mut alice_signature_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_signature_tracker.register(&*alice_peer_id, &extract_service_result_cid(&alice_call_1));
    let mut alice_signature_store = SignatureStore::new();
    let alice_signature = alice_signature_tracker.gen_signature("", &alice_keypair).unwrap();
    alice_signature_store.put(alice_pk, alice_signature);

    let alice_trace = vec![alice_call_1.clone()];

    let mut mallory_cid_state = alice_cid_state.clone();
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = stream_tracked!("fake result", 0, &mut mallory_cid_state, peer = &alice_peer_id);

    let mut mallory_signature_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_signature_tracker.register(&*mallory_peer_id, &extract_service_result_cid(&mallory_call_2));
    let mut mallory_signature_store = SignatureStore::new();
    let mallory_signature = mallory_signature_tracker.gen_signature("", &mallory_keypair).unwrap();
    mallory_signature_store.put(mallory_pk, mallory_signature);

    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_state.into(),
        alice_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_state.into(),
        mallory_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm_with_key::<NativeAirRunner>(alice_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = serde_json::to_vec(&alice_data).unwrap();
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = alice_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .unwrap();
    assert_ne!(res.ret_code, 0, "{}", res.error_message);
}

#[test]
fn test_attack_injection_current_injection_unused() {
    // injecting a value that arrives to peer who does the next step
    let alice_peer_id = "alice_peer";
    let mallory_peer_id = "mallory_peer";

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
           (call "{alice_peer_id}" ("" "") []))
        "#
    );

    let mut alice_cid_state = ExecutionCidState::default();
    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_state, peer = &alice_peer_id);
    let alice_trace = vec![alice_call_1.clone()];

    let mut mallory_cid_state = alice_cid_state.clone();
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = unused!("fake result", peer = &alice_peer_id);
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mut alice_signature_store = SignatureStore::new();

    let mut alice_cid_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_cid_tracker.register(&alice_peer_id, &extract_service_result_cid(&mallory_trace[0]));
    let alice_signature = alice_cid_tracker.gen_signature("", &alice_keypair).unwrap();
    alice_signature_store.put(alice_pk, alice_signature);

    let mallory_signature_store = alice_signature_store.clone();
    let mut mallory_cid_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_cid_tracker.register(&mallory_peer_id, &extract_service_result_cid(&mallory_trace[1]));
    let mallory_signature = mallory_cid_tracker.gen_signature("", &mallory_keypair).unwrap();
    alice_signature_store.put(mallory_pk, mallory_signature);

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_state.into(),
        alice_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_state.into(),
        mallory_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm_with_key::<NativeAirRunner>(alice_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = serde_json::to_vec(&alice_data).unwrap();
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = alice_avm
        .call(&air_script, prev_data, cur_data, test_run_params)
        .unwrap();

    assert_ne!(res.ret_code, 0, "{}", res.error_message);
}

#[test]
fn test_attack_injection_other_peer_scalar() {
    // injecting a value that arrives to peer who does the next step
    let alice_peer_id = "alice_peer";
    let bob_peer_id = "bob_peer";
    let mallory_peer_id = "mallory_peer";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_peer_id);
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

    let mut mallory_cid_state = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_state, peer = &alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = scalar_tracked!("fake result", &mut mallory_cid_state, peer = &alice_peer_id);

    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

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
        mallory_cid_state.into(),
        signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();
    assert_ne!(res.ret_code, 0);
}

#[test]
fn test_attack_injection_other_peer_stream() {
    // injecting a value that arrives to peer who does the next step
    let alice_peer_id = "alice_peer";
    let bob_peer_id = "bob_peer";
    let mallory_peer_id = "mallory_peer";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
    (seq
       (seq
          (call "{alice_peer_id}" ("" "") [] x)
          (call "{mallory_peer_id}" ("" "") [] y))
       (call "{bob_peer_id}" ("" "") [] $z))
    "#
    );

    let mut mallory_cid_state = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_state, peer = &alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = stream_tracked!("fake result", 0, &mut mallory_cid_state, peer = &alice_peer_id);

    let mut signature_store = SignatureStore::new();
    let mut alice_signature_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_signature_tracker.register(&*alice_peer_id, &extract_service_result_cid(&alice_call_1));
    let alice_signature = alice_signature_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_signature_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_signature_tracker.register(&*mallory_peer_id, &extract_service_result_cid(&mallory_call_2));
    let mallory_signature = mallory_signature_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_state.into(),
        signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();
    assert_ne!(res.ret_code, 0, "{}", res.error_message);
}

#[test]
fn test_attack_injection_other_peer_unused() {
    // injecting a value that arrives to peer who does the next step
    let alice_peer_id = "alice_peer";
    let bob_peer_id = "bob_peer";
    let mallory_peer_id = "mallory_peer";

    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_peer_id);
    let (bob_keypair, bob_peer_id) = derive_dummy_keypair(bob_peer_id);
    let (mallory_keypair, mallory_peer_id) = derive_dummy_keypair(mallory_peer_id);
    let alice_pk: PublicKey = alice_keypair.public().into();
    let mallory_pk: PublicKey = mallory_keypair.public().into();

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{bob_peer_id}" ("" "") []))
        "#
    );

    let mut mallory_cid_state = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_state, peer = &alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_state, peer = &mallory_peer_id);
    let fake_call_3 = unused!("fake result", peer = &alice_peer_id);

    let mut signature_store = SignatureStore::new();
    let mut alice_signature_tracker = PeerCidTracker::new(alice_peer_id.clone());
    alice_signature_tracker.register(&*alice_peer_id, &extract_service_result_cid(&alice_call_1));
    let alice_signature = alice_signature_tracker.gen_signature("", &alice_keypair).unwrap();
    signature_store.put(alice_pk, alice_signature);

    let mut mallory_signature_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    mallory_signature_tracker.register(&*mallory_peer_id, &extract_service_result_cid(&mallory_call_2));
    let mallory_signature = mallory_signature_tracker.gen_signature("", &mallory_keypair).unwrap();
    signature_store.put(mallory_pk, mallory_signature);

    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_state.into(),
        signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair, unit_call_service());
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    // please not that such injection is not caught
    assert_eq!(res.ret_code, 0, "{}", res.error_message);
}

#[test]
fn test_attack_replay() {
    let alice_name = "alice_peer_id";
    let bob_name = "bob_peer_id";
    let (alice_keypair, alice_peer_id) = derive_dummy_keypair(alice_name);
    let (bob_keypair, _) = derive_dummy_keypair(bob_name);

    let air_script = format!(
        r#"(seq
             (call "{alice_peer_id}" ("" "") [] y)
             (call "bob" ("" "") [] z))"#
    );

    let mut alice_avm = create_avm_with_key::<NativeAirRunner>(alice_keypair.clone(), unit_call_service());
    let mut bob_avm = create_avm_with_key::<NativeAirRunner>(bob_keypair.clone(), unit_call_service());

    let run_params1 = TestRunParameters::from_init_peer_id(&alice_peer_id).with_particle_id("first_particle");
    let run_params2 = run_params1.clone();

    let res1 = alice_avm.call(&air_script, "", "", run_params1.clone()).unwrap();
    let res2 = alice_avm.call(&air_script, "", "", run_params2).unwrap();

    assert_eq!(res1.ret_code, 0, "test validity check failed: {}", res1.error_message);
    assert_eq!(res1, res2, "test validity check failed");

    let res_bob = bob_avm.call(&air_script, "", res1.data.clone(), run_params1).unwrap();
    assert_eq!(
        res_bob.ret_code, 0,
        "test validity check failed: {}",
        res_bob.error_message
    );

    let mallory_run_params = TestRunParameters::from_init_peer_id(&alice_peer_id).with_particle_id("second_particle");

    let res_replay = bob_avm.call(&air_script, "", res1.data, mallory_run_params).unwrap();

    let dalek_error = ed25519_dalek::ed25519::Error::from_source("Verification equation was not satisfied");
    let nested_error = fluence_keypair::error::VerificationError::Ed25519(
        dalek_error,
        "2XNyeQMxiZnW6NGJdn1eP1RDGTgMA8DXKoh7VrWyn3tpLi9nC6X1AcyGeHUkH3m1gDNtHeRpcBfFLMe2wYgCNJCM".to_owned(),
        "6m3zmtymxDL56KBpNgKqc7QiGRuWuxr82bG2q7dF5xCD".to_owned(),
    );
    let cids: Vec<Box<str>> = vec!["bagaaieraazcwm4lxybe4pwlisvcgpv4mii63nxouogvf4ihkmz762mnhea7a".into()];
    let expected = PreparationError::DataSignatureCheckError(verification::DataVerifierError::SignatureMismatch {
        error: nested_error,
        cids,
    });
    assert_error_eq!(&res_replay, expected);
}
