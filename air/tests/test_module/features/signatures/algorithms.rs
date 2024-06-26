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

use air::PreparationError;
use air_interpreter_signatures::KeyError;
use air_test_utils::{
    assert_error_eq,
    prelude::unit_call_service,
    test_runner::{create_avm_with_key, NativeAirRunner, TestRunParameters},
};
use fluence_keypair::KeyFormat;

/// Checking that other peers' key algorithms are valid.
#[tokio::test]
// ignored for a while until we find an easy way to create "incorrect" rkyv data;
//
// n.b.: cfg(any()) disables compilation
#[cfg(any())]
fn test_banned_signature() {
    use air::min_supported_version;
    use air_interpreter_data::verification::DataVerifierError;
    use air_interpreter_data::InterpreterDataEnvelope;
    use air_interpreter_signatures::PublicKey;
    use air_test_utils::prelude::request_sent_by;
    use air_test_utils::test_runner::create_avm;
    use air_test_utils::JValue;
    use serde_json::json;

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

    let mut data_env = InterpreterDataEnvelope::from_execution_result(
        trace.into(),
        <_>::default(),
        <_>::default(),
        <_>::default(),
        min_supported_version().clone(),
    );

    let mut data: JValue = InterpreterDataRepr
        .get_format()
        .from_slice(&data_env.inner_data)
        .unwrap();

    data["signatures"] = bad_signature_store;
    data_env.inner_data = InterpreterDataRepr.get_format().to_vec(&data).unwrap();

    let current_data = data_env.serialize().unwrap();

    let mut avm = create_avm(unit_call_service(), "other_peer_id").await;
    let res = avm
        .call(
            air_script,
            "",
            current_data,
            TestRunParameters::from_init_peer_id("init_peer_fake_id"),
        )
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::DataSignatureCheckError(DataVerifierError::MalformedKey {
            error: KeyError::AlgorithmNotWhitelisted(KeyFormat::Secp256k1),
            key: PublicKey::new(bad_algo_pk).to_string(),
        })
    );
}

/// Checking that local key is valid.
#[tokio::test]
async fn test_banned_signing_key() {
    let air_script = "(null)";
    let bad_algo_keypair = fluence_keypair::KeyPair::generate_secp256k1();

    let mut avm = create_avm_with_key::<NativeAirRunner>(bad_algo_keypair, unit_call_service(), <_>::default()).await;
    let res = avm
        .call(air_script, "", "", TestRunParameters::from_init_peer_id("init_peer_id"))
        .await
        .unwrap();

    assert_error_eq!(
        &res,
        PreparationError::MalformedKeyPairData(KeyError::AlgorithmNotWhitelisted(KeyFormat::Secp256k1))
    );
}
