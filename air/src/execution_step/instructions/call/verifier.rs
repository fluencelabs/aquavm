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

use crate::UncatchableError;

use polyplets::SecurityTetraplet;

/// Check that computed call parameters match the parameters from current data.
pub(crate) fn verify_call(
    expected_argument_hash: &str,
    expected_tetraplet: &SecurityTetraplet,
    stored_argument_hash: &str,
    stored_tetraplet: &SecurityTetraplet,
) -> Result<(), UncatchableError> {
    if expected_argument_hash != stored_argument_hash {
        return Err(UncatchableError::CallParametersMismatch {
            param: "argument_hash",
            expected_value: expected_argument_hash.to_owned(),
            stored_value: stored_argument_hash.to_owned(),
        });
    }
    if expected_tetraplet != stored_tetraplet {
        return Err(UncatchableError::CallParametersMismatch {
            param: "tetraplet",
            expected_value: format!("{expected_tetraplet:?}"),
            stored_value: format!("{stored_tetraplet:?}"),
        });
    }
    Ok(())
}
