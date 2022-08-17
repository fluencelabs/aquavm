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

use avm_interface::{AVMInterfaceError, ErrorAVMOutcome};
use marine::IValue;
use marine::MarineError;

use serde_json::Error as SerdeError;
use thiserror::Error as ThisError;

use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, ThisError)]
pub enum AVMError<E> {
    /// This error contains interpreter outcome in case when execution failed on the interpreter
    /// side. A host should match on this error type explicitly to save provided data.
    #[error("interpreter failed with: {0:?}")]
    InterpreterFailed(ErrorAVMOutcome),

    /// This errors are encountered from an AVM runner.
    #[error(transparent)]
    RunnerError(RunnerError),

    /// This errors are encountered from a data store object.
    #[error(transparent)]
    DataStoreError(#[from] E),

    /// This errors are encountered from serialization of data tracked during an anomaly.
    #[error(transparent)]
    AnomalyDataSeError(SerdeError),
}

#[derive(Debug, ThisError)]
pub enum RunnerError {
    /// This errors are encountered from FaaS.
    #[error(transparent)]
    MarineError(#[from] MarineError),

    /// Specified path to AIR interpreter .wasm file was invalid
    #[error("path to AIR interpreter .wasm ({invalid_path:?}) is invalid: {reason}; IO Error: {io_error:?}")]
    InvalidAIRPath {
        invalid_path: PathBuf,
        io_error: Option<IOError>,
        reason: &'static str,
    },

    /// AIR interpreter result deserialization errors.
    #[error("{0}")]
    InterpreterResultDeError(String),

    /// Marine call returns Vec<IValue> to support multi-value in a future,
    /// but actually now it could return empty vec or a vec with one value.
    /// This error is encountered when it returns vec with not a one value.
    #[error("result `{0:?}` returned from Marine should contain only one element")]
    IncorrectInterpreterResult(Vec<IValue>),

    /// This errors are encountered from an call results/params se/de.
    #[error(transparent)]
    CallSeDeErrors(#[from] CallSeDeErrors),
}

// TODO same variant? it will make the RunnerError type little larger; but it is yet to be measured
impl From<AVMInterfaceError> for RunnerError {
    fn from(value: AVMInterfaceError) -> Self {
        match value {
            AVMInterfaceError::InterpreterResultDeError(e) => {
                RunnerError::InterpreterResultDeError(e)
            }
            AVMInterfaceError::CallSeDeErrors(e) => RunnerError::CallSeDeErrors(e),
        }
    }
}

pub use avm_interface::CallSeDeErrors;
