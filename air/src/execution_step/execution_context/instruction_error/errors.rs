/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::JValue;

use thiserror::Error as ThisError;

/// Describes errors related to converting a scalar into error object.
#[derive(Debug, Clone, ThisError)]
pub enum LastErrorObjectError {
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
}
