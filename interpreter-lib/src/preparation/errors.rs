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

use serde_json::Error as SerdeJsonError;
use thiserror::Error as ThisError;

use std::env::VarError;
use std::error::Error;

/// Errors happened during the interpreter preparation step.
#[derive(Debug)]
pub enum PreparationError {
    /// Error occurred while parsing AIR script
    AIRParseError(String),

    /// Errors occurred on executed trace deserialization.
    ExecutedTraceDeError(SerdeJsonError, Vec<u8>),

    /// Point out that error is occured while getting current peer id.
    CurrentPeerIdEnvError(VarError),

    /// Errors occurred while merging previous and current data.
    StateMergingError(DataMergingError),
}

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub enum DataMergingError {
    /// Errors occurred when previous and current executed states are incompatible.
    #[error("previous and current data have incompatible states: '{0:?}' '{1:?}'")]
    IncompatibleExecutedStates(ExecutedState, ExecutedState),

    /// Errors occurred when previous and current call results are incompatible.
    #[error("previous and current call results are incompatible: '{0:?}' '{1:?}'")]
    IncompatibleCallResults(CallResult, CallResult),

    /// Errors occurred when executed trace contains less elements then corresponding Par has.
    #[error("executed trace has {0} elements, but {1} requires by Par")]
    ExecutedTraceTooSmall(usize, usize),

    /// Errors occurred when corresponding fold have different iterable names.
    #[error("saved folds have different iterable names: {0}, {1}")]
    IncompatibleFoldIterableNames(String, String),
}

impl Error for PreparationError {}

impl PreparationError {
    pub(crate) fn to_error_code(&self) -> u32 {
        use DataMergingError::*;
        use PreparationError::*;

        match self {
            AIRParseError(_) => 1,
            ExecutedTraceDeError(..) => 2,
            CurrentPeerIdEnvError(_) => 3,
            StateMergingError(IncompatibleExecutedStates(..)) => 4,
            StateMergingError(IncompatibleCallResults(..)) => 5,
            StateMergingError(ExecutedTraceTooSmall(..)) => 6,
            StateMergingError(IncompatibleFoldIterableNames(..)) => 7,
        }
    }
}

use std::fmt;

impl fmt::Display for PreparationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use PreparationError::*;

        match self {
            AIRParseError(err_msg) => write!(f, "aqua script can't be parsed:\n{}", err_msg),
            ExecutedTraceDeError(serde_error, executed_trace) => {
                fn print_error(
                    f: &mut fmt::Formatter<'_>,
                    trace: impl std::fmt::Debug,
                    serde_error: &SerdeJsonError,
                ) -> Result<(), fmt::Error> {
                    write!(
                        f,
                        "an error occurred while executed trace deserialization on '{:?}': {:?}",
                        trace, serde_error
                    )
                }

                match String::from_utf8(executed_trace.to_vec()) {
                    Ok(str) => print_error(f, str, serde_error),
                    Err(e) => print_error(f, e.into_bytes(), serde_error),
                }
            }
            CurrentPeerIdEnvError(err) => write!(f, "current peer id can't be obtained: {:?}", err),
            StateMergingError(err) => write!(f, "{}", err),
        }
    }
}

impl From<DataMergingError> for PreparationError {
    fn from(err: DataMergingError) -> Self {
        Self::StateMergingError(err)
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
