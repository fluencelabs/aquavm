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

use super::ErrorAVMOutcome;
use fluence_faas::FaaSError;
use fluence_faas::IValue;

use serde_json::Error as SerdeError;
use thiserror::Error as ThisError;

use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, ThisError)]
pub enum AVMError {
    /// FaaS errors.
    #[error(transparent)]
    FaaSError(#[from] FaaSError),

    /// AIR interpreter result deserialization errors.
    #[error("{0}")]
    InterpreterResultDeError(String),

    /// Specified path to AIR interpreter .wasm file was invalid
    #[error("path to AIR interpreter .wasm ({invalid_path:?}) is invalid: {reason}; IO Error: {io_error:?}")]
    InvalidAIRPath {
        invalid_path: PathBuf,
        io_error: Option<IOError>,
        reason: &'static str,
    },

    /// FaaS call returns Vec<IValue> to support multi-value in a future,
    /// but actually now it could return empty vec or a vec with one value.
    /// This error is encountered when it returns vec with not a one value.
    #[error("result `{0:?}` returned from FaaS should contain only one element")]
    IncorrectInterpreterResult(Vec<IValue>),

    /// This error is encountered when deserialization pof call requests failed for some reason.
    #[error("'{raw_call_request:?}' can't been serialized with error '{error}'")]
    CallRequestsDeError {
        raw_call_request: Vec<u8>,
        error: SerdeError,
    },

    /// This error contains interpreter outcome in case when execution failed on the interpreter
    /// side. A host should match on this error type explicitly to save provided data.
    #[error("interpreter failed with: {0:?}")]
    InterpreterFailed(ErrorAVMOutcome),

    /// This errors are encountered from a data store object.
    #[error(transparent)]
    DataStoreError(#[from] eyre::Error),
}

impl From<std::convert::Infallible> for AVMError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
