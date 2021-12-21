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

use air_trace_handler::MergerApResult;
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
    #[error(transparent)]
    TraceError(#[from] TraceHandlerError),

    /// Fold state wasn't found for such iterator name.
    #[error("fold state not found for this iterable '{0}'")]
    FoldStateNotFound(String),

    /// Errors encountered while shadowing non-scalar values.
    #[error("variable with name '{0}' can't be shadowed, shadowing isn't supported for iterables")]
    IterableShadowing(String),

    /// Multiple fold states found for such iterator name.
    #[error("multiple iterable values found for iterable name '{0}'")]
    MultipleIterableValues(String),

    /// Errors occurred when result from data doesn't match to a instruction, f.e. an instruction
    /// could be applied to a stream, but result doesn't contain generation in a source position.
    #[error("ap result {0:?} doesn't match corresponding instruction")]
    ApResultNotCorrespondToInstr(MergerApResult),

    /// Multiple values for such name found.
    #[error("multiple variables found for name '{0}' in data")]
    MultipleVariablesFound(String),
}

impl ToErrorCode for UncatchableError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::UNCATCHABLE_ERRORS_START_ID;
        crate::generate_to_error_code!(self, UncatchableError, UNCATCHABLE_ERRORS_START_ID)
    }
}
