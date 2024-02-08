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
use air_test_utils::prelude::*;

#[tokio::test]
fn invalid_air() {
    let vm_peer_id = "some_peer_id";
    let mut vm = create_avm(unit_call_service(), vm_peer_id);

    let script = r#"(seq )"#;

    let result = call_vm!(vm, <_>::default(), script, "", "");

    let error_message = air_parser::parse(script).expect_err("air parser should fail on this script");
    let expected_error = PreparationError::AIRParseError(error_message);
    assert!(check_error(&result, expected_error));
}
