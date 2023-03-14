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
use air_interpreter_interface::{CallResults, RunParameters};
use air_test_utils::prelude::*;

use serde::Deserialize;
use serde::Serialize;

#[test]
fn invalid_data_without_versions() {
    #[derive(Serialize, Deserialize)]
    struct InvalidDataStruct {
        pub trace: Vec<u8>,
    }

    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = r#"(null)"#;
    let invalid_data = InvalidDataStruct { trace: vec![1, 2, 3] };
    let invalid_data = serde_json::to_vec(&invalid_data).unwrap();

    let result = call_vm!(vm, <_>::default(), script, "", invalid_data.clone());

    let expected_serde_error = serde_json::from_slice::<InterpreterData>(&invalid_data).err().unwrap();
    let expected_error = PreparationError::DataDeFailed {
        data: invalid_data,
        error: expected_serde_error,
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn invalid_data_with_versions() {
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
    let invalid_data = serde_json::to_vec(&invalid_data).unwrap();

    let result = call_vm!(vm, <_>::default(), script, "", invalid_data.clone());

    let expected_serde_error = serde_json::from_slice::<InterpreterData>(&invalid_data).err().unwrap();
    let expected_error = PreparationError::DataDeFailedWithVersions {
        data: invalid_data,
        error: expected_serde_error,
        versions,
    };
    assert!(check_error(&result, expected_error));
}

#[test]
fn invalid_callresults() {
    let air = r#"(null)"#.to_string();
    let client_peer_id = "some_peer_id".to_string();
    let prev_data = InterpreterData::new(semver::Version::new(1, 1, 1));
    let prev_data: Vec<u8> = serde_json::to_vec(&prev_data).unwrap();
    let data = Vec::<u8>::new();
    let wrong_call_results = Vec::<u32>::new();
    let wrong_call_results = serde_json::to_vec(&wrong_call_results).unwrap();
    let run_parameters = RunParameters::new(client_peer_id.clone(), client_peer_id.clone(), 0, 0);
    let result = air::execute_air(air, prev_data, data, run_parameters, wrong_call_results.clone());
    let result = RawAVMOutcome::from_interpreter_outcome(result).unwrap();

    let expected_serde_error = serde_json::from_slice::<CallResults>(&wrong_call_results)
        .err()
        .unwrap();
    let expected_error = PreparationError::CallResultsDeFailed {
        error: expected_serde_error,
        call_results: wrong_call_results,
    };

    assert!(check_error(&result, expected_error));
}
