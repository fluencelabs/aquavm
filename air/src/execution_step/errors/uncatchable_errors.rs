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

use super::Stream;
use crate::execution_step::Generation;
use crate::execution_step::STREAM_MAX_SIZE;
use crate::CanonStreamMapError;
use crate::StreamMapError;
use crate::StreamMapKeyError;
use crate::ToErrorCode;

use air_interpreter_cid::CidCalculationError;
use air_interpreter_cid::CidRef;
use air_interpreter_data::ValueRef;
use air_interpreter_interface::CallArgumentsRepr;
use air_interpreter_interface::TetrapletsRepr;
use air_interpreter_sede::Representation;
use air_trace_handler::GenerationCompactificationError;
use air_trace_handler::IntConversionError;
use air_trace_handler::TraceHandlerError;

use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

use std::rc::Rc;

/// Uncatchable errors arisen during AIR script execution. Uncatchable here means that these errors
/// couldn't be handled by a xor instruction and their error_code couldn't be used in a match
/// instruction. They are similar to JVM runtime errors and some of them could be caught only
/// while execution of AIR script, others (FoldStateNotFound and MultipleVariablesFound) are
/// checked additionally on the validation step, and presence here for convenience.
#[derive(ThisError, EnumDiscriminants, Debug)]
#[strum_discriminants(derive(EnumIter))]
pub enum UncatchableError {
    /// Errors bubbled from a trace handler.
    #[error("on instruction '{instruction}' trace handler encountered an error: {trace_error}")]
    TraceError {
        trace_error: TraceHandlerError,
        instruction: String,
    },

    /// These errors are related to internal bug in the interpreter when result trace is corrupted.
    #[error(transparent)]
    GenerationCompactificationError(#[from] GenerationCompactificationError),

    /// Integer casts, e.g. usize(=u64) to u32, might trigger such errors.
    #[error(transparent)]
    IntConversionError(#[from] IntConversionError),

    /// Fold state wasn't found for such iterator name.
    #[error("fold state not found for this iterable '{0}'")]
    FoldStateNotFound(String),

    /// Errors encountered while shadowing non-scalar values.
    #[error("variable with name '{0}' can't be shadowed, shadowing isn't supported for iterables")]
    IterableShadowing(String),

    /// Multiple fold states found for such iterator name.
    #[error("multiple iterable values found for iterable name '{0}'")]
    MultipleIterableValues(String),

    /// Errors occurred when result from data doesn't match to a call instruction, f.e. a call
    /// could be applied to a stream, but result doesn't contain generation in a source position.
    #[error("call result value {0:?} doesn't match with corresponding instruction")]
    CallResultNotCorrespondToInstr(ValueRef),

    /// Variable shadowing is not allowed, usually it's thrown when a AIR tries to assign value
    /// for a variable not in a fold block or in a global scope but not right after new.
    #[error("trying to shadow variable '{0}', but shadowing is allowed only inside fold blocks")]
    ShadowingIsNotAllowed(String),

    /// This error occurred when new tries to pop up a variable at the end, but scalar state doesn't
    /// contain an appropriate variable. It should be considered as an internal error and shouldn't
    /// be caught by a xor instruction.
    #[error("new end block tries to pop up a variable '{scalar_name}' that wasn't defined at depth {depth}")]
    ScalarsStateCorrupted { scalar_name: String, depth: usize },

    #[error("failed to calculate value's CID")]
    CidError(#[from] CidCalculationError),

    /// We consider now that every CID should present in the data;
    /// and not having any CID is considered a non-catching error.
    #[error("{0} for CID {1:?} not found")]
    ValueForCidNotFound(&'static str, Rc<CidRef>),

    /// Errors occurred while insertion of a value inside stream that doesn't have corresponding generation.
    #[error(
        "stream doesn't have generation with number {generation}, supplied to the interpreter data is corrupted,\n\
             stream is {stream:?}"
    )]
    StreamDontHaveSuchGeneration { stream: Stream, generation: Generation },

    #[error("failed to deserialize to CallServiceFailed: {0}")]
    MalformedCallServiceFailed(serde_json::Error),

    /// Stream size estimate goes over a hardcoded limit.
    #[error("stream size goes over the allowed limit of {STREAM_MAX_SIZE}")]
    StreamSizeLimitExceeded,

    /// CanonStreamMapKey related errors.
    #[error(transparent)]
    StreamMapKeyError(#[from] StreamMapKeyError),

    /// Stream map related errors.
    #[error(transparent)]
    StreamMapError(#[from] StreamMapError),

    /// CanonStreamMap related errors.
    #[error(transparent)]
    CanonStreamMapError(#[from] CanonStreamMapError),

    /// Argument hash or tetraplet mismatch in a call/canon merged from current_data with an evaluated value.
    #[error("{param} doesn't match expected parameters: expected {expected_value}, got {stored_value} ")]
    InstructionParametersMismatch {
        param: &'static str,
        expected_value: String,
        stored_value: String,
    },

    #[error("failed to sign data: {0}")]
    SigningError(#[from] fluence_keypair::error::SigningError),

    #[error("failed to serialize tetraplets {0}")]
    TetrapletSerializationFailed(<TetrapletsRepr as Representation>::SerializeError),

    #[error("failed to serialize call arguments {0}")]
    CallArgumentsSerializationFailed(<CallArgumentsRepr as Representation>::SerializeError),

    #[error("Starlark error: {0}")]
    StarlarkError(air_interpreter_starlark::ExecutionError),
}

impl ToErrorCode for UncatchableError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::UNCATCHABLE_ERRORS_START_ID;
        crate::generate_to_error_code!(self, UncatchableError, UNCATCHABLE_ERRORS_START_ID)
    }
}
