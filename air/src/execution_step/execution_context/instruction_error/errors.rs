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

use crate::JValue;

use thiserror::Error as ThisError;

/// Describes errors related to converting a scalar into error object.
#[derive(Debug, Clone, ThisError)]
pub enum ErrorObjectError {
    #[error("scalar should have an object type to be converted into error object, but '{0}' doesn't have")]
    ScalarMustBeObject(JValue),

    #[error("scalar '{scalar}' must have field with name '{field_name}'")]
    ScalarMustContainField { scalar: JValue, field_name: &'static str },

    #[error("{field_name} of scalar '{scalar}' must have {expected_type} type")]
    ScalarFieldIsWrongType {
        scalar: JValue,
        field_name: &'static str,
        expected_type: &'static str,
    },

    #[error("error code must be non-zero, but it is zero")]
    ErrorCodeMustBeNonZero,
}
