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
        return Err(UncatchableError::InstructionParametersMismatch {
            param: "call argument_hash",
            expected_value: expected_argument_hash.to_owned(),
            stored_value: stored_argument_hash.to_owned(),
        });
    }
    if expected_tetraplet != stored_tetraplet {
        return Err(UncatchableError::InstructionParametersMismatch {
            param: "call tetraplet",
            expected_value: format!("{expected_tetraplet:?}"),
            stored_value: format!("{stored_tetraplet:?}"),
        });
    }
    Ok(())
}
