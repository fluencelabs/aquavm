/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub use avm_interface::CallSeDeErrors;
use avm_interface::ErrorAVMOutcome;
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

    /// Invalid secret key.
    #[error(transparent)]
    KeyError(eyre::Error),

    /// Errors from auxiliary calls.
    #[error("{0}")]
    Aux(String),
}
