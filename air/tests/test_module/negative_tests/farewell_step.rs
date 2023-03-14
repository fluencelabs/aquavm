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

use air::FarewellError;
use air_test_utils::prelude::*;

#[test]
fn unprocessed_call_result() {
    let air = r#"(null)"#;
    let client_peer_id = "some_peer_id";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id);
    let prev_data = InterpreterData::new(semver::Version::new(1, 1, 1));
    let prev_data: Vec<u8> = serde_json::to_vec(&prev_data).unwrap();
    let call_service_result = air_test_utils::CallServiceResult::ok(json!("null"));
    let call_results_4_call = maplit::hashmap!(
        70 => call_service_result,
    );

    let result = client_vm
        .runner
        .call(air, prev_data, "", client_peer_id, 0, 0, None, call_results_4_call)
        .unwrap();

    let expected_call_service_result = air_interpreter_interface::CallServiceResult::ok(&json!("null"));
    let expected_call_results = maplit::hashmap!(
        70 => expected_call_service_result,
    );
    let expected_error = FarewellError::UnprocessedCallResult(expected_call_results);
    assert!(check_error(&result, expected_error));
}
