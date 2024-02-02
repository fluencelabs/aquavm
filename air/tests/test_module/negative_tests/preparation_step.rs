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

use air::PreparationError;
use air_interpreter_interface::CallResultsFormat;
use air_interpreter_interface::CallResultsRepr;
use air_interpreter_interface::RunParameters;
use air_interpreter_interface::MAX_AIR_SIZE;
use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
use air_interpreter_interface::MAX_PARTICLE_SIZE;
use air_interpreter_sede::FromSerialized;
use air_test_utils::prelude::*;

use serde::Deserialize;
use serde::Serialize;

#[test]
fn invalid_data_without_versions() {
    use air_interpreter_sede::Format;
    use air_interpreter_sede::Representation;

    #[derive(Serialize, Deserialize)]
    struct InvalidDataStruct {
        pub trace: Vec<u8>,
    }

    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = r#"(null)"#;
    let invalid_data = InvalidDataStruct { trace: vec![1, 2, 3] };

    let invalid_data = InterpreterDataEnvelopeRepr.get_format().to_vec(&invalid_data).unwrap();

    let result = call_vm!(vm, <_>::default(), script, "", invalid_data.clone());

    let expected_serde_error = InterpreterDataEnvelope::try_from_slice(&invalid_data).unwrap_err();
    let expected_error = PreparationError::EnvelopeDeFailed {
        error: expected_serde_error,
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn invalid_data_with_versions() {
    use air_interpreter_sede::Format;
    use air_interpreter_sede::Representation;

    #[derive(Serialize, Deserialize)]
    struct InvalidDataStruct {
        pub trace: Vec<u8>,
        #[serde(flatten)]
        pub versions: Versions,
    }

    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = r#"(null)"#;
    let versions = Versions::new(semver::Version::new(1, 1, 1));
    let invalid_data = InvalidDataStruct {
        trace: vec![1, 2, 3],
        versions: versions.clone(),
    };
    let invalid_data = InterpreterDataEnvelopeRepr.get_format().to_vec(&invalid_data).unwrap();

    let result = call_vm!(vm, <_>::default(), script, "", invalid_data.clone());

    let expected_serde_error = InterpreterDataEnvelope::try_from_slice(&invalid_data).unwrap_err();
    let expected_error = PreparationError::EnvelopeDeFailedWithVersions {
        error: expected_serde_error,
        versions,
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn invalid_callresults() {
    use air_interpreter_sede::Format;

    let air = r#"(null)"#.to_string();
    let client_peer_id = "some_peer_id".to_string();
    let prev_data = InterpreterDataEnvelope::new(semver::Version::new(1, 1, 1));
    let prev_data: Vec<u8> = prev_data.serialize().unwrap();
    let data = Vec::<u8>::new();
    let vec = Vec::<u8>::new();
    let wrong_call_results = CallResultsFormat::default().to_vec(&vec).unwrap();
    let keypair = fluence_keypair::KeyPair::generate_ed25519();
    let air_size_limit = MAX_AIR_SIZE;
    let particle_size_limit = MAX_PARTICLE_SIZE;
    let call_result_size_limit = MAX_CALL_RESULT_SIZE;

    let run_parameters = RunParameters::new(
        client_peer_id.clone(),
        client_peer_id.clone(),
        0,
        0,
        keypair.key_format().into(),
        keypair.secret().unwrap(),
        "".to_owned(),
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
    );

    let result = air::execute_air(air, prev_data, data, run_parameters, wrong_call_results.clone().into());
    let result = RawAVMOutcome::from_interpreter_outcome(result).unwrap();

    let expected_serde_error = CallResultsRepr.deserialize(&wrong_call_results).unwrap_err();
    let expected_error = PreparationError::CallResultsDeFailed {
        error: expected_serde_error,
    };

    assert!(check_error(&result, expected_error));
}

#[test]
fn air_size_limit() {
    let script = "a".repeat((MAX_AIR_SIZE + 1) as usize);
    let mut vm = create_avm(unit_call_service(), "some_peer_id");
    let result = vm.call(script, "", "", <_>::default()).unwrap();

    let expected_error = PreparationError::AIRSizeLimitReached((MAX_AIR_SIZE + 1) as usize, MAX_AIR_SIZE);

    assert!(check_error(&result, expected_error));
}

#[test]
fn particle_size_limit() {
    let script = "(null)";
    let mut vm = create_avm(unit_call_service(), "some_peer_id");
    let cur_data = vec![0; (MAX_PARTICLE_SIZE + 1) as usize];
    let result = vm.call(script, "", cur_data, <_>::default()).unwrap();

    let expected_error =
        PreparationError::ParticleSizeLimitReached((MAX_PARTICLE_SIZE + 1) as usize, MAX_PARTICLE_SIZE);

    assert!(check_error(&result, expected_error));
}

#[test]
fn call_result_size_limit() {
    use maplit::hashmap;

    use air::ToErrorCode;
    use air_interpreter_interface::MAX_CALL_RESULT_SIZE;

    let peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), "some_peer_id");

    let script = "(null)";
    let result_1 = "a".repeat((MAX_CALL_RESULT_SIZE / 2 + 1) as usize);
    let result_2 = "b".repeat((MAX_CALL_RESULT_SIZE + 1) as usize);
    let call_results: CallResults =
        hashmap! {0 => CallServiceResult::ok(result_1.into()), 1 => CallServiceResult::ok(result_2.into())};

    let result = vm
        .call_single(script, "", "", peer_id, 0, 64, None, call_results, "particle_id")
        .unwrap();

    let expected_error = PreparationError::CallResultSizeLimitReached(MAX_CALL_RESULT_SIZE);

    assert_eq!(result.ret_code, expected_error.to_error_code());
}
