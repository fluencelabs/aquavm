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

use jsonpath_lib::JsonPathError;
use serde_json::Error as SerdeJsonError;
use serde_sexpr::Error as SExprError;

use std::convert::Into;
use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub enum AquamarineError {
    /// Errors occurred while parsing aqua script in the form of S expressions.
    SExprParseError(SExprError),

    /// Errors occurred while parsing supplied data.
    DataParseError(SerdeJsonError),

    /// Indicates that environment variable with name CURRENT_PEER_ID isn't set.
    CurrentPeerIdNotSet(VarError),

    /// Semantic errors in instructions.
    InstructionError(String),

    /// Semantic errors in instructions.
    LocalServiceError(String),

    /// Value with such name isn't presence in data.
    VariableNotFound(String),

    /// Value with such path wasn't found in data with such error.
    VariableNotInJsonPath(String, JsonPathError),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::SExprParseError(err) => write!(f, "{}", err),
            AquamarineError::DataParseError(err) => write!(f, "{}", err),
            AquamarineError::CurrentPeerIdNotSet(err) => write!(f, "{}", err),
            AquamarineError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::VariableNotFound(variable_name) => write!(
                f,
                "variable with name {} isn't present in data",
                variable_name
            ),
            AquamarineError::VariableNotInJsonPath(json_path, json_path_err) => write!(
                f,
                "variable with path {} not found with error: {}",
                json_path, json_path_err
            ),
            AquamarineError::MultipleValuesInJsonPath(json_path) => write!(
                f,
                "multiple variables found for this json path {}",
                json_path
            ),
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
            AquamarineError::DataParseError(..) => 2,
            AquamarineError::CurrentPeerIdNotSet(..) => 3,
            AquamarineError::InstructionError(..) => 4,
            AquamarineError::LocalServiceError(..) => 5,
            AquamarineError::VariableNotFound(..) => 6,
            AquamarineError::VariableNotInJsonPath(..) => 7,
            AquamarineError::MultipleValuesInJsonPath(..) => 8,
        };

        StepperOutcome {
            ret_code,
            data: format!("{}", self),
            next_peer_pks: vec![],
        }
    }
}
