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
use crate::JValue;
use crate::StepperOutcome;

use jsonpath_lib::JsonPathError;
use serde_json::Error as SerdeJsonError;
use serde_sexpr::Error as SExprError;

use std::convert::Into;
use std::env::VarError;
use std::error::Error;

#[derive(Debug)]
pub(crate) enum CallEvidenceError {
    IncorrectState(String),

}

impl Error for CallEvidenceError {}

impl std::fmt::Display for CallEvidenceError {
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
            AquamarineError::CurrentPeerIdEnvError(err, env_name) => write!(
                f,
                "the environment variable \"{}\" can't be obtained: {:?}",
                env_name,
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
                "multiple variables found for name {} in data",
                variable_name
            ),
            AquamarineError::VariableNotInJsonPath(value, json_path, json_path_err) => write!(
                f,
                "variable with path {} not found in {:?} with error: {:?}",
                json_path, value, json_path_err
            ),
            AquamarineError::IncompatibleJValueType(avalue, desired_type) => write!(
                f,
                "got avalue \"{:?}\", but {} type is needed",
                avalue,
                desired_type,
            ),
            AquamarineError::MultipleValuesInJsonPath(json_path) => write!(
                f,
                "multiple variables found for this json path {}",
                json_path
            ),
            AquamarineError::FoldStateNotFound(iterator) => write!(
                f,
                "fold state not found for this iterable {}",
                iterator
            ),
            AquamarineError::MultipleFoldStates(iterator) => write!(
                f,
                "multiple fold states found for iterable {}",
                iterator
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
