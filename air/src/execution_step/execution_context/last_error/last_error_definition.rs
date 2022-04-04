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

use super::LastErrorObjectError;
use crate::execution_step::RcSecurityTetraplet;

use air_values::boxed_value::RcBoxedValue;

use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

pub const ERROR_CODE_FIELD_NAME: &str = "error_code";
pub const MESSAGE_FIELD_NAME: &str = "message";
pub const INSTRUCTION_FIELD_NAME: &str = "instruction";
pub const PEER_ID_FIELD_NAME: &str = "peer_id";

/// This struct is intended to track the last arisen error.
/// LastError is essentially a scalar value with support of lambda expressions.
/// The only differences from a scalar are
///  - it's accessed by %last_error% literal
///  - if it's unset before the usage, JValue::Null will be used without join behaviour
///  - it's a global scalar, meaning that fold and new scopes doesn't apply for it
#[derive(Debug, Clone)]
pub struct LastError {
    /// Error object that represents the last occurred error.
    pub error: RcBoxedValue,

    /// Tetraplet that identify host where the error occurred.
    pub tetraplet: Option<RcSecurityTetraplet>,
}

pub(crate) fn error_from_raw_fields(
    error_code: i64,
    error_message: &str,
    instruction: &str,
    peer_id: &str,
) -> RcBoxedValue {
    // TODO: abstract over it
    let jvalue = serde_json::json!({
        ERROR_CODE_FIELD_NAME: error_code,
        MESSAGE_FIELD_NAME: error_message,
        INSTRUCTION_FIELD_NAME: instruction,
        PEER_ID_FIELD_NAME: peer_id,
    });

    Rc::new(jvalue) as RcBoxedValue
}

/// Checks that a scalar is a value of an object types that contains at least two fields:
///  - error_code
///  - message
pub(crate) fn check_error_object(value: &RcBoxedValue) -> Result<(), LastErrorObjectError> {
    check_error_code(value)?;
    check_message(value)
}

fn check_error_code(value: &RcBoxedValue) -> Result<(), LastErrorObjectError> {
    let error_code = value.get_by_field_name(ERROR_CODE_FIELD_NAME).and_then(|v| v.as_i64());

    match error_code {
        Some(_) => Ok(()),
        None => Err(LastErrorObjectError::ScalarFieldIsWrongType {
            scalar: value.to_string(),
            field_name: ERROR_CODE_FIELD_NAME,
            expected_type: "integer",
        }),
    }
}

fn check_message(value: &RcBoxedValue) -> Result<(), LastErrorObjectError> {
    let error_code = value.get_by_field_name(MESSAGE_FIELD_NAME).and_then(|v| v.as_str());

    match error_code {
        Some(_) => Ok(()),
        None => Err(LastErrorObjectError::ScalarFieldIsWrongType {
            scalar: value.to_string(),
            field_name: MESSAGE_FIELD_NAME,
            expected_type: "integer",
        }),
    }
}
