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

#[tokio::test]
async fn invalid_data_without_versions() {
    use air_interpreter_sede::Format;
    use air_interpreter_sede::Representation;

    #[derive(Serialize, Deserialize)]
    struct InvalidDataStruct {
        pub trace: Vec<u8>,
    }

    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

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

#[tokio::test]
async fn invalid_data_with_versions() {
    use air_interpreter_sede::Format;
    use air_interpreter_sede::Representation;

    #[derive(Serialize, Deserialize)]
    struct InvalidDataStruct {
        pub trace: Vec<u8>,
        #[serde(flatten)]
        pub versions: Versions,
    }

    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id).await;

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

#[tokio::test]
async fn invalid_callresults() {
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
    let hard_limit_enable = false;

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
        hard_limit_enable,
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
fn air_size_hard_limit() {
    let script = "a".repeat((MAX_AIR_SIZE + 1) as usize);

    let peer_id = "some_peer_id".to_owned();
    let air_size_limit = MAX_AIR_SIZE;
    let particle_size_limit = MAX_PARTICLE_SIZE;
    let call_result_size_limit = MAX_CALL_RESULT_SIZE;
    let hard_limit_enable = true;

    let run_parameters = RunParameters::new(
        peer_id.clone(),
        peer_id,
        0,
        0,
        <_>::default(),
        <_>::default(),
        "".to_owned(),
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
        hard_limit_enable,
    );

    let result = air::execute_air(script, vec![], vec![], run_parameters, <_>::default());
    let result = RawAVMOutcome::from_interpreter_outcome(result).unwrap();

    let expected_error = PreparationError::air_size_limit((MAX_AIR_SIZE + 1) as usize, MAX_AIR_SIZE);

    assert!(check_error(&result, expected_error));
}

#[test]
fn particle_size_hard_limit() {
    let script = "(null)".to_owned();
    let cur_data = vec![0; (MAX_PARTICLE_SIZE + 1) as usize];

    let peer_id = "some_peer_id".to_owned();
    let air_size_limit = MAX_AIR_SIZE;
    let particle_size_limit = MAX_PARTICLE_SIZE;
    let call_result_size_limit = MAX_CALL_RESULT_SIZE;
    let hard_limit_enable = true;

    let run_parameters = RunParameters::new(
        peer_id.clone(),
        peer_id,
        0,
        0,
        <_>::default(),
        <_>::default(),
        "".to_owned(),
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
        hard_limit_enable,
    );

    let result = air::execute_air(script, vec![], cur_data, run_parameters, <_>::default());
    let result = RawAVMOutcome::from_interpreter_outcome(result).unwrap();

    let expected_error = PreparationError::particle_size_limit((MAX_PARTICLE_SIZE + 1) as usize, MAX_PARTICLE_SIZE);

    assert!(check_error(&result, expected_error));
}

#[test]
fn call_result_size_hard_limit() {
    use maplit::hashmap;

    use air::ToErrorCode;
    use air_interpreter_interface::MAX_CALL_RESULT_SIZE;
    use air_interpreter_sede::ToSerialized;

    let script = "(null)".to_owned();
    let result_1 = "a".repeat((MAX_CALL_RESULT_SIZE / 2 + 1) as usize);
    let result_2 = "b".repeat((MAX_CALL_RESULT_SIZE + 1) as usize);
    let call_results: CallResults =
        hashmap! {0 => CallServiceResult::ok(result_1.into()), 1 => CallServiceResult::ok(result_2.into())};

    let raw_call_results = into_raw_result(call_results);
    let raw_call_results = CallResultsRepr.serialize(&raw_call_results).unwrap();

    let peer_id = "some_peer_id".to_owned();
    let air_size_limit = MAX_AIR_SIZE;
    let particle_size_limit = MAX_PARTICLE_SIZE;
    let call_result_size_limit = MAX_CALL_RESULT_SIZE;
    let hard_limit_enable = true;

    let run_parameters = RunParameters::new(
        peer_id.clone(),
        peer_id,
        0,
        0,
        <_>::default(),
        <_>::default(),
        "".to_owned(),
        air_size_limit,
        particle_size_limit,
        call_result_size_limit,
        hard_limit_enable,
    );

    let result = air::execute_air(script, vec![], vec![], run_parameters, raw_call_results);
    let result = RawAVMOutcome::from_interpreter_outcome(result).unwrap();

    let expected_error = PreparationError::call_result_size_limit(MAX_CALL_RESULT_SIZE);

    assert_eq!(result.ret_code, expected_error.to_error_code());
}
