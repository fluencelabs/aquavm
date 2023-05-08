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

use air_interpreter_signatures::SignatureTracker;
use air_test_framework::{ephemeral::PeerId, AirScriptExecutor};
use air_test_utils::prelude::extract_service_result_cid;
use air_test_utils::test_runner::TestRunParameters;
use air_test_utils::*;

pub fn stub_keypair(_peer_id: &str) -> fluence_keypair::KeyPair {
    fluence_keypair::KeyPair::from_secret_key([1; 32].into(), fluence_keypair::KeyFormat::Ed25519).unwrap()
}

#[test]
fn test_signature_empty() {
    let script = "(null)";
    let init_peer_id = "init_peer_id";
    let exec = AirScriptExecutor::new(
        TestRunParameters::from_init_peer_id(init_peer_id),
        vec![],
        vec![PeerId::from(init_peer_id)].into_iter(),
        script,
    )
    .unwrap();
    let res = exec.execute_one(init_peer_id).unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);

    let keypair = stub_keypair(init_peer_id);
    let expected_signature: air_interpreter_signatures::Signature = keypair.sign(b"[]").unwrap().into();

    let data = data_from_result(&res);
    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[test]
fn test_signature_call_var() {
    let init_peer_id = "init_peer_id";
    let air_script = format!(
        r#"
        (call "{init_peer_id}" ("" "") [] var) ; ok = "ok"
        "#
    );
    let exec = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &air_script).unwrap();

    let res = exec.execution_iter(init_peer_id).unwrap().last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let expected_call_state = scalar!("ok", peer = init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let keypair = stub_keypair(init_peer_id);

    let mut expected_tracker = SignatureTracker::new();
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid).clone());
    let expected_signature = expected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[test]
fn test_signature_call_stream() {
    let init_peer_id = "init_peer_id";
    let air_script = format!(
        r#"
        (call "{init_peer_id}" ("" "") [] $var) ; ok = "ok"
        "#
    );
    let exec = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &air_script).unwrap();

    let res = exec.execution_iter(init_peer_id).unwrap().last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let expected_call_state = stream!("ok", 0, peer = init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let keypair = stub_keypair(init_peer_id);

    let mut expected_tracker = SignatureTracker::new();
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid).clone());
    let expected_signature = expected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[test]
fn test_signature_call_ununsed() {
    let init_peer_id = "init_peer_id";
    let air_script = format!(
        r#"
        (call "{init_peer_id}" ("" "") []) ; ok = "ok"
        "#
    );
    let exec = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &air_script).unwrap();

    let res = exec.execution_iter(init_peer_id).unwrap().last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let keypair = stub_keypair(init_peer_id);

    let mut expected_tracker = SignatureTracker::new();
    let expected_signature = expected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[test]
fn test_signature_call_merged() {
    let init_peer_id = "init_peer_id";
    let other_peer_id = "other_peer_id";

    let air_script = format!(
        r#"
    (seq
       (call "{init_peer_id}" ("" "") [] x) ; ok = "res0"
       (seq
          (call "{other_peer_id}" ("" "") [] y) ; ok = "res1"
          (call "{init_peer_id}" ("" "") [] z) ; ok = "res2"
       ))
    "#
    );

    let exec = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &air_script).unwrap();
    let _ = exec.execute_one(init_peer_id).unwrap();
    let _ = exec.execute_one(other_peer_id).unwrap();
    let res2 = exec.execute_one(init_peer_id).unwrap();
    let data2 = data_from_result(&res2);

    let expected_call_state0 = scalar!("res0", peer = init_peer_id, service = "..0");
    let expected_cid0 = extract_service_result_cid(&expected_call_state0);
    let expected_call_state2 = scalar!("res2", peer = init_peer_id, service = "..2");
    let expected_cid2 = extract_service_result_cid(&expected_call_state2);

    let keypair = stub_keypair(init_peer_id);

    let mut expected_tracker = SignatureTracker::new();
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid0).clone());
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid2).clone());
    let expected_signature = expected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    let signature = data2.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data2.signatures);
}

#[test]
fn test_signature_call_double() {
    // Test that if some CID appears twice in the call result, it is accounted twice.
    let init_peer_id = "init_peer_id";
    let air_script = format!(
        r#"
        (seq
            (seq (ap 1 $s) (ap 2 $s))
            (fold $s i
                (seq
                    (call "{init_peer_id}" ("" "") [] var) ; ok = "ok"
                    (next i))))
        "#
    );
    let exec = AirScriptExecutor::simple(TestRunParameters::from_init_peer_id(init_peer_id), &air_script).unwrap();

    let res = exec.execution_iter(init_peer_id).unwrap().last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let expected_call_state = scalar!("ok", peer = init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let keypair = stub_keypair(init_peer_id);

    let mut unexpected_tracker = SignatureTracker::new();
    unexpected_tracker.register(init_peer_id.to_owned(), (*expected_cid).clone());
    let unexpected_signature = unexpected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    let mut expected_tracker = SignatureTracker::new();
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid).clone());
    expected_tracker.register(init_peer_id.to_owned(), (*expected_cid).clone());
    let expected_signature = expected_tracker.into_signature(init_peer_id, &keypair).unwrap();

    assert_ne!(expected_signature, unexpected_signature, "test is incorrect");

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}
