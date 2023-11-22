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

use air::{min_supported_version, PreparationError};
use air_interpreter_data::{verification::DataVerifierError, InterpreterData, InterpreterDataRepr};
use air_interpreter_sede::{Format, TypedFormat};
use air_interpreter_signatures::KeyError;
use air_test_utils::{
    assert_error_eq,
    prelude::{request_sent_by, unit_call_service},
    test_runner::{create_avm, create_avm_with_key, NativeAirRunner, TestRunParameters},
};
use fluence_keypair::KeyFormat;
use serde_json::json;

/// Checking that other peers' key algorithms are valid.
#[test]
fn test_banned_signature() {
    let air_script = r#"(call "other_peer_id" ("" "") [])"#;

    let bad_algo_keypair = fluence_keypair::KeyPair::generate_secp256k1();
    let bad_algo_pk = bad_algo_keypair.public();
    let bad_algo_signature: air_interpreter_signatures::Signature =
        air_interpreter_signatures::sign_cids(vec![], "particle_id", &bad_algo_keypair)
            .unwrap()
            .into();

    let bad_algo_pk_ser = bs58::encode(bad_algo_pk.encode()).into_string();
    let bad_signature_store = json!({
        bad_algo_pk_ser: bad_algo_signature,
    });
    let bad_peer_id = bad_algo_pk.to_peer_id().to_string();

    let trace = vec![request_sent_by("init_peer_fake_id")];

    let mut data = serde_json::to_value(InterpreterData::from_execution_result(
        trace.into(),
        <_>::default(),
        <_>::default(),
        <_>::default(),
        min_supported_version().clone(),
    ))
    .unwrap();

    data["signatures"] = bad_signature_store;

    let current_data = InterpreterDataRepr.get_format().to_vec(&data).unwrap();

    let mut avm = create_avm(unit_call_service(), "other_peer_id");
    let res = avm
        .call(
            air_script,
            "",
            current_data,
            TestRunParameters::from_init_peer_id("init_peer_fake_id"),
        )
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::DataSignatureCheckError(DataVerifierError::MalformedKey {
            error: KeyError::AlgorithmNotWhitelisted(KeyFormat::Secp256k1),
            peer_id: bad_peer_id
        })
    );
}

/// Checking that local key is valid.
#[test]
fn test_banned_signing_key() {
    let air_script = "(null)";
    let bad_algo_keypair = fluence_keypair::KeyPair::generate_secp256k1();

    let mut avm = create_avm_with_key::<NativeAirRunner>(bad_algo_keypair, unit_call_service());
    let res = avm
        .call(air_script, "", "", TestRunParameters::from_init_peer_id("init_peer_id"))
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::MalformedKeyPairData(KeyError::AlgorithmNotWhitelisted(KeyFormat::Secp256k1))
    );
}
