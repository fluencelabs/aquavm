/*
 * Copyright 2021 Fluence Labs Limited
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

use rmp_serde::decode::Error as SerdeDeError;
use rmp_serde::encode::Error as SerdeSeError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[allow(clippy::enum_variant_names)]
pub enum CallSeDeErrors {
    /// Errors encountered while trying to serialize call results.
    #[error("error occurred while call results `{call_results:?}` deserialization: {se_error}")]
    CallResultsSeFailed {
        call_results: air_interpreter_interface::CallResults,
        se_error: SerdeSeError,
    },

    /// This error is encountered when deserialization pof call requests failed for some reason.
    #[error("'{raw_call_request:?}' can't been serialized with error '{error}'")]
    CallRequestsDeError {
        raw_call_request: Vec<u8>,
        error: SerdeDeError,
    },

    /// Errors encountered while trying to deserialize arguments from call parameters returned
    /// by the interpreter. In the corresponding struct such arguments are Vec<JValue> serialized
    /// to a string.
    #[error("error occurred while deserialization of arguments from call params `{call_params:?}`: {de_error}")]
    CallParamsArgsDeFailed {
        call_params: air_interpreter_interface::CallRequestParams,
        de_error: SerdeJsonError,
    },

    /// Errors encountered while trying to deserialize tetraplets from call parameters returned
    /// by the interpreter. In the corresponding struct such tetraplets are
    /// Vec<Vec<SecurityTetraplet>> serialized to a string.
    #[error("error occurred while deserialization of tetraplets from call params `{call_params:?}`: {de_error}")]
    CallParamsTetrapletsDeFailed {
        call_params: air_interpreter_interface::CallRequestParams,
        de_error: SerdeJsonError,
    },
}
type JValue = serde_json::Value;

pub use call_request_parameters::*;
pub use call_service_result::*;
pub use outcome::*;
pub use particle_parameters::*;
