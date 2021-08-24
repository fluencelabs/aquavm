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

use fluence_faas::FaaSError;
use fluence_faas::IValue;

use thiserror::Error as ThisError;

use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, ThisError)]
pub enum AVMError {
    /// FaaS errors.
    #[error("{0}")]
    FaaSError(#[from] FaaSError),

    /// AIR interpreter result deserialization errors.
    #[error("{0}")]
    InterpreterResultDeError(String),

    /// I/O errors while persisting resulted data.
    #[error("an error occurred while saving prev data {0:?} by {1:?} path")]
    PersistDataError(#[source] IOError, PathBuf),

    /// Errors related to particle_data_store path from supplied config.
    #[error("an error occurred while creating data storage {0:?} by {1:?} path")]
    InvalidDataStorePath(#[source] IOError, PathBuf),

    /// Failed to create Particle File Vault directory (thrown inside Effect)
    #[error("error creating Particle File Vault {1:?}: {0:?}")]
    CreateVaultDirError(#[source] IOError, PathBuf),

    /// Failed to remove particle directories (called by node after particle's ttl is expired)
    #[error("error cleaning up particle directory {1:?}: {0:?}")]
    CleanupParticleError(#[source] IOError, PathBuf),

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
}

impl From<std::convert::Infallible> for AVMError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
