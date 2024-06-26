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

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod call_request_parameters;
mod call_service_result;
mod outcome;
mod particle_parameters;
pub mod raw_outcome;

use air_interpreter_interface::CallArgumentsDeserializeError;
use air_interpreter_interface::CallRequestsDeserializeError;
use air_interpreter_interface::CallResultsSerializeError;
use air_interpreter_interface::SerializedCallRequests;
use air_interpreter_interface::TetrapletDeserializeError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[allow(clippy::enum_variant_names)]
pub enum CallSeDeErrors {
    /// Errors encountered while trying to serialize call results.
    #[error("error occurred while call results `{call_results:?}` deserialization: {se_error}")]
    CallResultsSeFailed {
        call_results: air_interpreter_interface::CallResults,
        se_error: CallResultsSerializeError,
    },

    /// This error is encountered when deserialization pof call requests failed for some reason.
    #[error("'{raw_call_request:?}' can't been serialized with error '{error}'")]
    CallRequestsDeError {
        raw_call_request: SerializedCallRequests,
        error: CallRequestsDeserializeError,
    },

    /// Errors encountered while trying to deserialize arguments from call parameters returned
    /// by the interpreter. In the corresponding struct such arguments are Vec<JValue> serialized
    /// to a string.
    #[error("error occurred while deserialization of arguments from call params `{call_params:?}`: {de_error}")]
    CallParamsArgsDeFailed {
        call_params: air_interpreter_interface::CallRequestParams,
        de_error: CallArgumentsDeserializeError,
    },

    /// Errors encountered while trying to deserialize tetraplets from call parameters returned
    /// by the interpreter. In the corresponding struct such tetraplets are
    /// Vec<Vec<SecurityTetraplet>> serialized to a string.
    #[error("error occurred while deserialization of tetraplets from call params `{call_params:?}`: {de_error}")]
    CallParamsTetrapletsDeFailed {
        call_params: air_interpreter_interface::CallRequestParams,
        de_error: TetrapletDeserializeError,
    },
}

type JValue = serde_json::Value;

pub use air_interpreter_interface::SoftLimitsTriggering;
pub use call_request_parameters::*;
pub use call_service_result::*;
pub use outcome::*;
pub use particle_parameters::*;
