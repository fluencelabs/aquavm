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

use air::min_supported_version;
use air::PreparationError;
use air_interpreter_interface::INTERPRETER_SUCCESS;
use air_test_utils::prelude::*;

#[tokio::test]
async fn minimal_version_check() {
    let mut vm = create_avm(echo_call_service(), "").await;
    let script = "(null)";

    let actual_version = semver::Version::new(0, 31, 1);
    let current_data = InterpreterDataEnvelope::new(actual_version.clone());
    let current_data = current_data.serialize().expect("default serializer shouldn't fail");
    let result = call_vm!(vm, <_>::default(), script, "", current_data);

    let expected_error = PreparationError::UnsupportedInterpreterVersion {
        actual_version,
        required_version: min_supported_version().clone(),
    };

    assert!(check_error(&result, expected_error));
}

#[tokio::test]
async fn publish_version_check() {
    let mut vm = create_avm(echo_call_service(), "").await;
    let script = "(null)";

    let actual_version =
        semver::Version::parse("1.0.1-feat-VM-173-add-interpreter-version-in-data-a2d575b-205-1.0").unwrap();
    let current_data = InterpreterDataEnvelope::new(actual_version);
    let current_data = current_data.serialize().expect("default serializer shouldn't fail");
    let result = call_vm!(vm, <_>::default(), script, "", current_data);

    assert_eq!(result.ret_code, INTERPRETER_SUCCESS, "{:?}", result.error_message);
}

#[tokio::test]
async fn publish_unsupported_version_check() {
    let mut vm = create_avm(echo_call_service(), "").await;

    let actual_version =
        semver::Version::parse("0.31.1-feat-VM-173-add-interpreter-version-in-data-a2d575b-205-1.0").unwrap();
    let current_data = InterpreterDataEnvelope::new(actual_version.clone());
    let current_data = current_data.serialize().expect("default serializer shouldn't fail");
    let result = call_vm!(vm, <_>::default(), "", "", current_data);

    let expected_error = PreparationError::UnsupportedInterpreterVersion {
        actual_version,
        required_version: min_supported_version().clone(),
    };

    assert!(check_error(&result, expected_error));
}
