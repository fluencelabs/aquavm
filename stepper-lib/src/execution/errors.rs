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
use crate::ResolvedCallResult;

use jsonpath_lib::JsonPathError;
use serde_json::Error as SerdeJsonError;

use std::env::VarError;
use std::error::Error;

/// Errors arised while executing AIR script.
#[derive(Debug)]
pub enum ExecutionError {
    /// Errors occurred while parsing function arguments of an expression.
    FuncArgsSerializationError(JValue, SerdeJsonError),

    /// Errors occurred while parsing returned by call_service value.
    CallServiceResultDeError(CallServiceResult, SerdeJsonError),

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

    /// An error occurred while trying to apply json path to this JValue.
    JValueJsonPathError(JValue, String, JsonPathError),

    /// An error occurred while trying to apply json path to this accumulator with JValue's.
    JValueAccJsonPathError(Vec<ResolvedCallResult>, String, JsonPathError),

    /// Provided JValue has incompatible with target type.
    IncompatibleJValueType(JValue, &'static str),

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
    CallEvidenceDeserializationError(SerdeJsonError, Vec<u8>),

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

impl ExecutionError {
    pub(crate) fn to_error_code(&self) -> i32 {
        match self {
            ExecutionError::AIRParseError(_) => 1,
            ExecutionError::FuncArgsSerializationError(..) => 2,
            ExecutionError::CallServiceResultDeError(..) => 3,
            ExecutionError::CurrentPeerIdEnvError(..) => 4,
            ExecutionError::InstructionError(..) => 5,
            ExecutionError::LocalServiceError(..) => 6,
            ExecutionError::VariableNotFound(..) => 7,
            ExecutionError::MultipleVariablesFound(..) => 8,
            ExecutionError::JValueJsonPathError(..) => 9,
            ExecutionError::JValueAccJsonPathError(..) => 10,
            ExecutionError::IncompatibleJValueType(..) => 11,
            ExecutionError::IncompatibleAValueType(..) => 12,
            ExecutionError::MultipleValuesInJsonPath(..) => 13,
            ExecutionError::FoldStateNotFound(..) => 14,
            ExecutionError::MultipleFoldStates(..) => 15,
            ExecutionError::InvalidEvidenceState(..) => 16,
            ExecutionError::CallEvidenceDeserializationError(..) => 17,
            ExecutionError::CallEvidenceSerializationError(..) => 18,
            ExecutionError::IncompatibleEvidenceStates(..) => 19,
            ExecutionError::IncompatibleCallResults(..) => 20,
            ExecutionError::EvidencePathTooSmall(..) => 21,
            ExecutionError::ShadowingError(_) => 22,
        }
    }
}

impl Error for ExecutionError {}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ExecutionError::AIRParseError(err) => write!(f, "aqua script can't be parsed:\n{}", err),
            ExecutionError::FuncArgsSerializationError(args, err) => write!(
                f,
                "function arguments {} can't be serialized or deserialized with an error: {:?}",
                args, err
            ),
            ExecutionError::CallServiceResultDeError(result, err) => write!(
                f,
                "call_service result \"{:?}\" can't be serialized or deserialized with an error: {:?}",
                result, err
            ),
            ExecutionError::CurrentPeerIdEnvError(err, env_name) => write!(
                f,
                "the environment variable \"{}\" can't be obtained: {:?}",
                env_name, err
            ),
            ExecutionError::InstructionError(err_msg) => write!(f, "{}", err_msg),
            ExecutionError::LocalServiceError(err_msg) => write!(f, "{}", err_msg),
            ExecutionError::VariableNotFound(variable_name) => {
                write!(f, "variable with name {} isn't present in data", variable_name)
            }
            ExecutionError::MultipleVariablesFound(variable_name) => {
                write!(f, "multiple variables found for name {} in data", variable_name)
            }
            ExecutionError::JValueJsonPathError(value, json_path, json_path_err) => write!(
                f,
                "variable with path {} not found in {:?} with error: {:?}",
                json_path, value, json_path_err
            ),
            ExecutionError::JValueAccJsonPathError(value, json_path, json_path_err) => write!(
                f,
                "variable with path {} not found in {:?} with error: {:?}",
                json_path, value, json_path_err
            ),
            ExecutionError::IncompatibleJValueType(jvalue, desired_type) => {
                write!(f, "got jvalue \"{:?}\", but {} type is needed", jvalue, desired_type,)
            }
            ExecutionError::IncompatibleAValueType(avalue, desired_type) => {
                write!(f, "got avalue {}, but {} type is needed", avalue, desired_type,)
            }
            ExecutionError::MultipleValuesInJsonPath(json_path) => {
                write!(f, "multiple variables found for this json path {}", json_path)
            }
            ExecutionError::FoldStateNotFound(iterator) => {
                write!(f, "fold state not found for this iterable {}", iterator)
            }
            ExecutionError::MultipleFoldStates(iterator) => {
                write!(f, "multiple fold states found for iterable {}", iterator)
            }
            ExecutionError::InvalidEvidenceState(found, expected) => write!(
                f,
                "invalid evidence state: expected {}, but found {:?}",
                expected, found
            ),
            ExecutionError::CallEvidenceDeserializationError(err, path) => write!(
                f,
                "an error occurred while call evidence path deserialization on {:?}: {:?}",
                path, err
            ),
            ExecutionError::CallEvidenceSerializationError(err) => {
                write!(f, "an error occurred while data serialization: {:?}", err)
            }
            ExecutionError::IncompatibleEvidenceStates(prev_state, current_state) => write!(
                f,
                "previous and current data have incompatible states: {:?} {:?}",
                prev_state, current_state
            ),
            ExecutionError::IncompatibleCallResults(prev_call_result, current_call_result) => write!(
                f,
                "previous and current call results are incompatible: {:?} {:?}",
                prev_call_result, current_call_result
            ),
            ExecutionError::EvidencePathTooSmall(actual_count, desired_count) => write!(
                f,
                "evidence path remains {} elements, but {} requires by Par",
                actual_count, desired_count
            ),
            ExecutionError::ShadowingError(variable_name) => write!(
                f,
                "vairable with name = '{}' can't be shadowed, shadowing is supported only for scalar values",
                variable_name
            ),
        }
    }
}

impl From<std::convert::Infallible> for ExecutionError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
