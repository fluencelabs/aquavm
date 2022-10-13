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

#[test]
fn minimal_version_check() {
    let mut vm = create_avm(echo_call_service(), "");

    let actual_version = semver::Version::new(0, 31, 1);
    let current_data = InterpreterData::new(actual_version.clone());
    let current_data = serde_json::to_vec(&current_data).expect("default serializer shouldn't fail");
    let result = call_vm!(vm, <_>::default(), "", "", current_data);

    let expected_error = PreparationError::UnsupportedInterpreterVersion {
        actual_version,
        required_version: semver::Version::new(0, 31, 2),
    };

    assert!(check_error(&result, expected_error));
}
