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

use air::ExecutionCidState;
use air_interpreter_signatures::{derive_dummy_keypair, PeerCidTracker, SignatureStore};
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

    let mut alice_cid_tracker = ExecutionCidState::new();
    let mut alice_signature_tracker = PeerCidTracker::new(alice_peer_id.clone());
    let mut alice_signature_store = SignatureStore::new();

    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_tracker, peer = &alice_peer_id);
    alice_signature_tracker.register(&*alice_peer_id, &extract_service_result_cid(&alice_call_1));
    let alice_trace = vec![alice_call_1.clone()];
    let alice_signature = alice_signature_tracker.gen_signature(&alice_keypair).unwrap();
    alice_signature_store.put(alice_keypair.public().into(), alice_signature);

    let mut mallory_cid_tracker = alice_cid_tracker.clone();
    let mut mallory_signature_tracker = PeerCidTracker::new(mallory_peer_id.clone());
    let mut mallory_signature_store = alice_signature_store.clone();

    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = &mallory_peer_id);
    let fake_call_3 = scalar_tracked!("fake result", &mut mallory_cid_tracker, peer = &alice_peer_id);
    mallory_signature_tracker.register(&*mallory_peer_id, &extract_service_result_cid(&mallory_call_2));
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];
    let mallory_signature = mallory_signature_tracker.gen_signature(&mallory_keypair).unwrap();
    mallory_signature_store.put(mallory_keypair.public().into(), mallory_signature);

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_tracker.into(),
        alice_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        mallory_signature_store,
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm(unit_call_service(), &alice_peer_id);
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

    let air_script = format!(
        r#"
    (seq
       (seq
          (call "{alice_peer_id}" ("" "") [] x)
          (call "{mallory_peer_id}" ("" "") [] y))
       (call "{alice_peer_id}" ("" "") [] $z))
    "#
    );

    let mut alice_cid_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_tracker, peer = alice_peer_id);
    let alice_trace = vec![alice_call_1.clone()];

    let mut mallory_cid_tracker = alice_cid_tracker.clone();
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = mallory_peer_id);
    let fake_call_3 = stream_tracked!("fake result", 0, &mut mallory_cid_tracker, peer = alice_peer_id);
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm(unit_call_service(), alice_peer_id);
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

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{alice_peer_id}" ("" "") []))
        "#
    );

    let mut alice_cid_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut alice_cid_tracker, peer = alice_peer_id);
    let alice_trace = vec![alice_call_1.clone()];

    let mut mallory_cid_tracker = alice_cid_tracker.clone();
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = mallory_peer_id);
    let fake_call_3 = unused!("fake result", peer = alice_peer_id);
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let alice_data = InterpreterData::from_execution_result(
        alice_trace.into(),
        <_>::default(),
        <_>::default(),
        alice_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mut alice_avm = create_avm(unit_call_service(), alice_peer_id);
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

    let air_script = format!(
        r#"
    (seq
       (seq
          (call "{alice_peer_id}" ("" "") [] x)
          (call "{mallory_peer_id}" ("" "") [] y))
       (call "{bob_peer_id}" ("" "") [] z))
    "#
    );

    let mut mallory_cid_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_tracker, peer = alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = mallory_peer_id);
    let fake_call_3 = scalar_tracked!("fake result", &mut mallory_cid_tracker, peer = alice_peer_id);

    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
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

    let air_script = format!(
        r#"
    (seq
       (seq
          (call "{alice_peer_id}" ("" "") [] x)
          (call "{mallory_peer_id}" ("" "") [] y))
       (call "{bob_peer_id}" ("" "") [] $z))
    "#
    );

    let mut mallory_cid_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_tracker, peer = alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = mallory_peer_id);
    let fake_call_3 = stream_tracked!("fake result", 0, &mut mallory_cid_tracker, peer = alice_peer_id);
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
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

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{alice_peer_id}" ("" "") [] x)
              (call "{mallory_peer_id}" ("" "") [] y))
           (call "{bob_peer_id}" ("" "") []))
        "#
    );

    let mut mallory_cid_tracker = ExecutionCidState::default();

    let alice_call_1 = scalar_tracked!("good result", &mut mallory_cid_tracker, peer = alice_peer_id);
    let mallory_call_2 = scalar_tracked!("valid result", &mut mallory_cid_tracker, peer = mallory_peer_id);
    let fake_call_3 = unused!("fake result", peer = alice_peer_id);
    let mallory_trace = vec![alice_call_1, mallory_call_2, fake_call_3];

    let mallory_data = InterpreterData::from_execution_result(
        mallory_trace.into(),
        <_>::default(),
        <_>::default(),
        mallory_cid_tracker.into(),
        todo!(),
        2,
        Version::new(1, 1, 1),
    );

    let mut bob_avm = create_avm(unit_call_service(), bob_peer_id);
    let test_run_params = TestRunParameters::from_init_peer_id(alice_peer_id);
    let prev_data = "";
    let cur_data = serde_json::to_vec(&mallory_data).unwrap();
    let res = bob_avm.call(&air_script, prev_data, cur_data, test_run_params).unwrap();

    assert_ne!(res.ret_code, 0, "{}", res.error_message);
}
