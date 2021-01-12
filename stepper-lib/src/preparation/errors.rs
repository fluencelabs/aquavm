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

use super::CallResult;
use super::ExecutedState;

use serde::Error as SerdeJsonError;
use thiserror::Error as ThisError;

use std::env::VarError;
use std::error::Error;

/// Errors happened during the stepper preparation step.
#[derive(Debug)]
pub(crate) enum PreparationError {
    /// Error occurred while parsing AIR script
    AIRParseError(String),

    /// Errors occurred on call evidence deserialization.
    CallEvidenceDeError(SerdeJsonError, Vec<u8>),

    /// Indicates that environment variable with current name doesn't set.
    CurrentPeerIdEnvError(VarError, String),

    /// Errors occurred while merging previous and current data.
    StateMergingError(DataMergingError),
}

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub(crate) enum DataMergingError {
    /// Errors occurred when previous and current evidence states are incompatible.
    #[error("previous and current data have incompatible states: '{0:?}' '{1:?}'")]
    IncompatibleExecutedStates(ExecutedState, ExecutedState),

    /// Errors occurred when previous and current call results are incompatible.
    #[error("previous and current call results are incompatible: '{0:?}' '{1:?}'")]
    IncompatibleCallResults(CallResult, CallResult),

    /// Errors occurred when evidence path contains less elements then corresponding Par has.
    #[error("evidence path has {0} elements, but {1} requires by Par")]
    EvidencePathTooSmall(usize, usize),
}

impl Error for PreparationError {}
impl Error for DataMergingError {}

impl std::fmt::Display for PreparationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use PreparationError::*;

        match self {
            AIRParseError(err_msg) => write!(f, "aqua script can't be parsed:\n{}", err_msg),
            CallEvidenceDeError(serde_error, evidence_path) => {
                let print_error = move |path| {
                    write!(
                        f,
                        "an error occurred while call evidence path deserialization on '{:?}': {:?}",
                        path, serde_error
                    )
                };

                let path = match String::from_utf8(evidence_path) {
                    Ok(str) => print_error(str),
                    Err(e) => print_error(e.into_bytes()),
                };
            }
            CurrentPeerIdEnvError(err, env_name) => write!(
                f,
                "the environment variable with name '{}' can't be obtained: {:?}",
                env_name, err
            ),
            StateMergingError(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::convert::Infallible> for PreparationError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl From<std::convert::Infallible> for DataMergingError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
