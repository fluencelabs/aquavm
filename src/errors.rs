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

use crate::CallServiceResult;
use crate::SerdeValue;
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

    /// Errors occurred while parsing aqua data.
    DataSerdeError(SerdeJsonError),

    /// Errors occurred while parsing function arguments of an expression.
    FuncArgsSerdeError(SerdeValue, SerdeJsonError),

    /// Errors occurred while parsing returned by call_service value.
    CallServiceSerdeError(CallServiceResult, SerdeJsonError),

    /// Indicates that environment variable with name CURRENT_PEER_ID isn't set.
    CurrentPeerIdEnvError(VarError),

    /// Semantic errors in instructions.
    InstructionError(String),

    /// An error is occurred while calling local service via call_service.
    LocalServiceError(String),

    /// Value for such name isn't presence in data.
    VariableNotFound(String),

    /// Value for such name presences both in data and fold states.
    MultipleVariablesFound(String),

    /// Value with such path wasn't found in data with such error.
    VariableNotInJsonPath(String, JsonPathError),

    /// Value with such name isn't presence in data.
    VariableIsNotArray(SerdeValue, String),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),

    /// Fold state for such iterable variable name isn't found.
    FoldStateNotFound(String),
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::SExprParseError(err) => {
                write!(f, "aqua script can't be parsed: {:?}", err)
            }
            AquamarineError::DataSerdeError(err) => write!(
                f,
                "an error occurred while serializing/deserializing data: {:?}",
                err
            ),
            AquamarineError::FuncArgsSerdeError(args, err) => write!(
                f,
                "function arguments {} can't be serialized or deserialized with an error: {:?}",
                args, err
            ),
            AquamarineError::CallServiceSerdeError(result, err) => write!(
                f,
                "call_service result \"{:?}\" can't be serialized or deserialized with an error: {:?}",
                result, err
            ),
            AquamarineError::CurrentPeerIdEnvError(err) => write!(
                f,
                "the environment variable with current peer id can't be obtained: {:?}",
                err
            ),
            AquamarineError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::VariableNotFound(variable_name) => write!(
                f,
                "variable with name {} isn't present in data",
                variable_name
            ),
            AquamarineError::MultipleVariablesFound(variable_name) => write!(
                f,
                "variable with name {} defined twice: in  call and fold",
                variable_name
            ),
            AquamarineError::VariableNotInJsonPath(json_path, json_path_err) => write!(
                f,
                "variable with path {} not found with error: {:?}",
                json_path, json_path_err
            ),
            AquamarineError::VariableIsNotArray(value, variable_name) => write!(
                f,
                "serde value {} addressed by name {} isn't an array and couldn't be used in fold",
                value, variable_name
            ),
            AquamarineError::MultipleValuesInJsonPath(json_path) => write!(
                f,
                "multiple variables found for this json path {}",
                json_path
            ),
            AquamarineError::FoldStateNotFound(iterable_variable_name) => write!(
                f,
                "fold state for variable with name {} not found",
                iterable_variable_name
            ),
        }
    }
}

impl From<SExprError> for AquamarineError {
    fn from(err: SExprError) -> Self {
        AquamarineError::SExprParseError(err)
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
            AquamarineError::DataSerdeError(..) => 2,
            AquamarineError::FuncArgsSerdeError(..) => 3,
            AquamarineError::CallServiceSerdeError(..) => 4,
            AquamarineError::CurrentPeerIdEnvError(..) => 5,
            AquamarineError::InstructionError(..) => 6,
            AquamarineError::LocalServiceError(..) => 7,
            AquamarineError::VariableNotFound(..) => 8,
            AquamarineError::MultipleVariablesFound(..) => 9,
            AquamarineError::VariableNotInJsonPath(..) => 10,
            AquamarineError::VariableIsNotArray(..) => 11,
            AquamarineError::MultipleValuesInJsonPath(..) => 12,
            AquamarineError::FoldStateNotFound(_) => 13,
        };

        StepperOutcome {
            ret_code,
            data: format!("{}", self),
            next_peer_pks: vec![],
        }
    }
}
