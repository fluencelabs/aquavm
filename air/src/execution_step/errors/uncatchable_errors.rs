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

use crate::ToErrorCode;

use air_interpreter_data::TracePos;
use air_interpreter_data::Value;
use air_trace_handler::merger::MergerApResult;
use air_trace_handler::GenerationCompatificationError;
use air_trace_handler::TraceHandlerError;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

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

    #[error(transparent)]
    GenerationCompatificationError(#[from] GenerationCompatificationError),

    /// Fold state wasn't found for such iterator name.
    #[error("fold state not found for this iterable '{0}'")]
    FoldStateNotFound(String),

    /// Errors encountered while shadowing non-scalar values.
    #[error("variable with name '{0}' can't be shadowed, shadowing isn't supported for iterables")]
    IterableShadowing(String),

    /// Multiple fold states found for such iterator name.
    #[error("multiple iterable values found for iterable name '{0}'")]
    MultipleIterableValues(String),

    /// Errors occurred when result from data doesn't match to an ap instruction, f.e. an ap
    /// could be applied to a stream, but result doesn't contain generation in a source position.
    #[error("ap result {0:?} doesn't match with corresponding instruction")]
    ApResultNotCorrespondToInstr(MergerApResult),

    /// Errors occurred when result from data doesn't match to a call instruction, f.e. a call
    /// could be applied to a stream, but result doesn't contain generation in a source position.
    #[error("call result value {0:?} doesn't match with corresponding instruction")]
    CallResultNotCorrespondToInstr(Value),

    /// Variable shadowing is not allowed, usually it's thrown when a AIR tries to assign value
    /// for a variable not in a fold block or in a global scope but not right after new.
    #[error("trying to shadow variable '{0}', but shadowing is allowed only inside fold blocks")]
    ShadowingIsNotAllowed(String),

    /// This error occurred when new tries to pop up a variable at the end, but scalar state doesn't
    /// contain an appropriate variable. It should be considered as an internal error and shouldn't
    /// be caught by a xor instruction.
    #[error("new end block tries to pop up a variable '{scalar_name}' that wasn't defined at depth {depth}")]
    ScalarsStateCorrupted { scalar_name: String, depth: usize },

    /// Variable with such a position wasn't defined during AIR script execution.
    /// Canon instruction requires this value to be present in data, otherwise it's considered
    /// as a hard error.
    #[error("variable with position '{0}' wasn't defined during script execution")]
    VariableNotFoundByPos(TracePos),
}

impl ToErrorCode for UncatchableError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::UNCATCHABLE_ERRORS_START_ID;
        crate::generate_to_error_code!(self, UncatchableError, UNCATCHABLE_ERRORS_START_ID)
    }
}
