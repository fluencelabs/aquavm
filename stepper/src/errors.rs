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

use crate::call_evidence::{CallResult, EvidenceState};
use crate::CallServiceResult;
use crate::JValue;
use crate::StepperOutcome;

use jsonpath_lib::JsonPathError;
use serde_json::Error as SerdeJsonError;
use serde_sexpr::Error as SExprError;

use std::convert::Into;
use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub(crate) enum AquamarineError {
    /// Errors occurred while parsing aqua script in the form of S expressions.
    SExprParseError(SExprError),

    /// Errors occurred on aqua data deserialization.
    DataDeserializationError(SerdeJsonError),

    /// Errors occurred on aqua data serialization.
    DataSerializationError(SerdeJsonError),

    /// Errors occurred while parsing function arguments of an expression.
    FuncArgsSerializationError(JValue, SerdeJsonError),

    /// Errors occurred while parsing returned by call_service value.
    CallServiceResultDeserializationError(CallServiceResult, SerdeJsonError),

    /// Indicates that environment variable with name CURRENT_PEER_ID isn't set.
    CurrentPeerIdEnvError(VarError, String),

    /// Semantic errors in instructions.
    InstructionError(String),

    /// An error is occurred while calling local service via call_service.
    LocalServiceError(String),

    /// Value for such name isn't presence in data.
    VariableNotFound(String),

    /// Multiple values for such name found.
    MultipleVariablesFound(String),

    /// Value with such path wasn't found in data with such error.
    VariableNotInJsonPath(JValue, String, JsonPathError),

    /// Value for such name isn't presence in data.
    IncompatibleJValueType(JValue, String),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),

    /// Fold state wasn't found for such iterator name.
    FoldStateNotFound(String),

    /// Multiple fold states found for such iterator name.
    MultipleFoldStates(String),

    /// Expected evidence state of different type.
    InvalidEvidenceState(EvidenceState, String),

    /// Errors occurred on call evidence deserialization.
    CallEvidenceDeserializationError(SerdeJsonError),

    /// Errors occurred on call evidence serialization.
    CallEvidenceSerializationError(SerdeJsonError),

    /// Errors occurred when reserved keyword is used for variable name.
    ReservedKeywordError(String),

    /// Errors occurred when previous and current evidence states are incompatible.
    IncompatibleEvidenceStates(EvidenceState, EvidenceState),

    /// Errors occurred when previous and current call results are incompatible.
    IncompatibleCallResults(CallResult, CallResult),
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::SExprParseError(err) => write!(f, "aqua script can't be parsed: {:?}", err),
            AquamarineError::DataDeserializationError(err) => {
                write!(f, "an error occurred while data deserialization: {:?}", err)
            }
            AquamarineError::DataSerializationError(err) => {
                write!(f, "an error occurred while data serialization: {:?}", err)
            }
            AquamarineError::FuncArgsSerializationError(args, err) => write!(
                f,
                "function arguments {} can't be serialized or deserialized with an error: {:?}",
                args, err
            ),
            AquamarineError::CallServiceResultDeserializationError(result, err) => write!(
                f,
                "call_service result \"{:?}\" can't be serialized or deserialized with an error: {:?}",
                result, err
            ),
            AquamarineError::CurrentPeerIdEnvError(err, env_name) => write!(
                f,
                "the environment variable \"{}\" can't be obtained: {:?}",
                env_name, err
            ),
            AquamarineError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            AquamarineError::VariableNotFound(variable_name) => {
                write!(f, "variable with name {} isn't present in data", variable_name)
            }
            AquamarineError::MultipleVariablesFound(variable_name) => {
                write!(f, "multiple variables found for name {} in data", variable_name)
            }
            AquamarineError::VariableNotInJsonPath(value, json_path, json_path_err) => write!(
                f,
                "variable with path {} not found in {:?} with error: {:?}",
                json_path, value, json_path_err
            ),
            AquamarineError::IncompatibleJValueType(jvalue, desired_type) => {
                write!(f, "got avalue \"{:?}\", but {} type is needed", jvalue, desired_type,)
            }
            AquamarineError::MultipleValuesInJsonPath(json_path) => {
                write!(f, "multiple variables found for this json path {}", json_path)
            }
            AquamarineError::FoldStateNotFound(iterator) => {
                write!(f, "fold state not found for this iterable {}", iterator)
            }
            AquamarineError::MultipleFoldStates(iterator) => {
                write!(f, "multiple fold states found for iterable {}", iterator)
            }
            AquamarineError::InvalidEvidenceState(found_state, expected) => write!(
                f,
                "invalid evidence state: expected {}, but found {:?}",
                expected, found_state
            ),
            AquamarineError::CallEvidenceDeserializationError(err) => {
                write!(f, "an error occurred while data deserialization: {:?}", err)
            }
            AquamarineError::CallEvidenceSerializationError(err) => {
                write!(f, "an error occurred while data serialization: {:?}", err)
            }
            AquamarineError::ReservedKeywordError(variable_name) => write!(
                f,
                "a variable can't be named as {} because this name is reserved",
                variable_name
            ),
            AquamarineError::IncompatibleEvidenceStates(prev_state, current_state) => write!(
                f,
                "previous and current data have incompatible states: {:?} {:?}",
                prev_state, current_state
            ),
            AquamarineError::IncompatibleCallResults(prev_call_result, current_call_result) => write!(
                f,
                "previous and current call results are incompatible: {:?} {:?}",
                prev_call_result, current_call_result
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
            AquamarineError::DataDeserializationError(..) => 2,
            AquamarineError::DataSerializationError(..) => 3,
            AquamarineError::FuncArgsSerializationError(..) => 4,
            AquamarineError::CallServiceResultDeserializationError(..) => 5,
            AquamarineError::CurrentPeerIdEnvError(..) => 6,
            AquamarineError::InstructionError(..) => 7,
            AquamarineError::LocalServiceError(..) => 8,
            AquamarineError::VariableNotFound(..) => 9,
            AquamarineError::MultipleVariablesFound(..) => 10,
            AquamarineError::VariableNotInJsonPath(..) => 11,
            AquamarineError::IncompatibleJValueType(..) => 12,
            AquamarineError::MultipleValuesInJsonPath(..) => 13,
            AquamarineError::FoldStateNotFound(..) => 14,
            AquamarineError::MultipleFoldStates(..) => 15,
            AquamarineError::InvalidEvidenceState(..) => 16,
            AquamarineError::CallEvidenceDeserializationError(..) => 17,
            AquamarineError::CallEvidenceSerializationError(..) => 18,
            AquamarineError::ReservedKeywordError(..) => 19,
            AquamarineError::IncompatibleEvidenceStates(..) => 20,
            AquamarineError::IncompatibleCallResults(..) => 21,
        };

        StepperOutcome {
            ret_code,
            data: format!("{}", self),
            next_peer_pks: vec![],
        }
    }
}
