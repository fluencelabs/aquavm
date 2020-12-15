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

use crate::build_targets::CallServiceResult;
use crate::call_evidence::CallResult;
use crate::call_evidence::EvidenceState;
use crate::JValue;

use jsonpath_lib::JsonPathError;
use serde_json::Error as SerdeJsonError;

use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub enum AquamarineError {
    /// Error occurred while parsing AIR script
    AIRParseError(String),

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

    /// Provided JValue has incompatible with target type.
    IncompatibleJValueType(JValue, String),

    /// Provided AValue has incompatible with target type.
    IncompatibleAValueType(String, String),

    /// Multiple values found for such json path.
    MultipleValuesInJsonPath(String),

    /// Fold state wasn't found for such iterator name.
    FoldStateNotFound(String),

    /// Multiple fold states found for such iterator name.
    MultipleFoldStates(String),

    /// Expected evidence state of different type.
    InvalidEvidenceState(EvidenceState, String),

    /// Errors occurred on call evidence deserialization.
    CallEvidenceDeserializationError(SerdeJsonError, String),

    /// Errors occurred on call evidence serialization.
    CallEvidenceSerializationError(SerdeJsonError),

    /// Errors occurred when previous and current evidence states are incompatible.
    IncompatibleEvidenceStates(EvidenceState, EvidenceState),

    /// Errors occurred when previous and current call results are incompatible.
    IncompatibleCallResults(CallResult, CallResult),

    /// Errors occurred when evidence path contains less elements then corresponding Par has.
    EvidencePathTooSmall(usize, usize),

    /// Errors occurred when evidence path contains less elements then corresponding Par has.
    ShadowingError(String),
}

impl AquamarineError {
    pub(crate) fn error_code(&self) -> i32 {
        match self {
            AquamarineError::AIRParseError(_) => 1,
            AquamarineError::FuncArgsSerializationError(..) => 2,
            AquamarineError::CallServiceResultDeserializationError(..) => 3,
            AquamarineError::CurrentPeerIdEnvError(..) => 4,
            AquamarineError::InstructionError(..) => 5,
            AquamarineError::LocalServiceError(..) => 6,
            AquamarineError::VariableNotFound(..) => 7,
            AquamarineError::MultipleVariablesFound(..) => 8,
            AquamarineError::VariableNotInJsonPath(..) => 9,
            AquamarineError::IncompatibleJValueType(..) => 10,
            AquamarineError::IncompatibleAValueType(..) => 11,
            AquamarineError::MultipleValuesInJsonPath(..) => 12,
            AquamarineError::FoldStateNotFound(..) => 13,
            AquamarineError::MultipleFoldStates(..) => 14,
            AquamarineError::InvalidEvidenceState(..) => 15,
            AquamarineError::CallEvidenceDeserializationError(..) => 16,
            AquamarineError::CallEvidenceSerializationError(..) => 17,
            AquamarineError::IncompatibleEvidenceStates(..) => 18,
            AquamarineError::IncompatibleCallResults(..) => 19,
            AquamarineError::EvidencePathTooSmall(..) => 20,
            AquamarineError::ShadowingError(_) => 21,
        }
    }
}

impl Error for AquamarineError {}

impl std::fmt::Display for AquamarineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AquamarineError::AIRParseError(err) => write!(f, "aqua script can't be parsed:\n{}", err),
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
                write!(f, "got jvalue \"{:?}\", but {} type is needed", jvalue, desired_type,)
            }
            AquamarineError::IncompatibleAValueType(avalue, desired_type) => {
                write!(f, "got avalue {}, but {} type is needed", avalue, desired_type,)
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
            AquamarineError::InvalidEvidenceState(found, expected) => write!(
                f,
                "invalid evidence state: expected {}, but found {:?}",
                expected, found
            ),
            AquamarineError::CallEvidenceDeserializationError(err, path) => write!(
                f,
                "an error occurred while call evidence path deserialization on {:?}: {:?}",
                path, err
            ),
            AquamarineError::CallEvidenceSerializationError(err) => {
                write!(f, "an error occurred while data serialization: {:?}", err)
            }
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
            AquamarineError::EvidencePathTooSmall(actual_count, desired_count) => write!(
                f,
                "evidence path remains {} elements, but {} requires by Par",
                actual_count, desired_count
            ),
            AquamarineError::ShadowingError(variable_name) => write!(
                f,
                "vairable with name = '{}' can't be shadowed, shadowing is supported only for scalar values",
                variable_name
            ),
        }
    }
}

impl From<std::convert::Infallible> for AquamarineError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
