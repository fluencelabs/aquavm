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

use air::FarewellError;
use air_test_utils::prelude::*;

#[tokio::test]
async fn unprocessed_call_result() {
    let air = r#"(null)"#;
    let client_peer_id = "some_peer_id";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id).await;
    let prev_data = InterpreterDataEnvelope::new(semver::Version::new(1, 1, 1));
    let prev_data: Vec<u8> = prev_data.serialize().unwrap();
    let call_service_result = air_test_utils::CallServiceResult::ok(json!("null"));
    let call_results_4_call = maplit::hashmap!(
        70 => call_service_result,
    );

    let result = client_vm
        .call_single(air, prev_data, "", client_peer_id, 0, 0, None, call_results_4_call, "")
        .await
        .unwrap();

    let expected_call_service_result = air_interpreter_interface::CallServiceResult::ok(&json!("null"));
    let expected_call_results = maplit::hashmap!(
        "70".to_owned() => expected_call_service_result,
    );
    let expected_error = FarewellError::UnprocessedCallResult(expected_call_results);
    assert!(check_error(&result, expected_error));
}
