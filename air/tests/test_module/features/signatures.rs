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

use air_test_framework::{ephemeral::PeerId, AirScriptExecutor};
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
fn test_signature_call_var() {}

#[test]
fn test_signature_call_stream() {}

#[test]
fn test_signature_call_ununsed() {}

#[test]
fn test_signature_call_merged() {}

#[test]
fn test_signature_call_double() {
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

    let res = exec.execute_one(init_peer_id).unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);

    let call_state = scalar!("ok", peer = init_peer_id);
    todo!();
}
