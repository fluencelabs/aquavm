/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::StepperOutcome;

use serde_json::Error as SerdeJsonError;
use serde_sexpr::Error as SExprError;

use std::convert::Into;
use std::error::Error;

#[derive(Debug)]
pub enum AquamarineError {
    /// Errors occurred while parsing aqua script in the form of S expressions.
    SExprParseError(SExprError),

    /// Errors occurred while parsing supplied data.
    DataParseError(SerdeJsonError),

    /// Aquamarine result deserialization errors.
    ExecutionError(String),
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::SExprParseError(err) => write!(f, "{}", err),
            AquamarineError::DataParseError(err) => write!(f, "{}", err),
            AquamarineError::ExecutionError(err_msg) => write!(f, "{}", err_msg),
        }
    }
}

impl From<SExprError> for AquamarineError {
    fn from(err: SExprError) -> Self {
        AquamarineError::SExprParseError(err)
    }
}

impl From<SerdeJsonError> for AquamarineError {
    fn from(err: SerdeJsonError) -> Self {
        AquamarineError::DataParseError(err)
    }
}

impl From<std::convert::Infallible> for AquamarineError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl Into<StepperOutcome> for AquamarineError {
    fn into(self) -> StepperOutcome {
        let ret_code = match self {
            AquamarineError::SExprParseError(_) => 1,
            AquamarineError::DataParseError(_) => 2,
            AquamarineError::ExecutionError(_) => 3,
        };

        StepperOutcome {
            ret_code,
            data: format!("{}", self),
            next_peer_pks: vec![],
        }
    }
}
