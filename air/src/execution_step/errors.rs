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

mod catchable
pub(crate) use catchable::Catchable;
pub(crate) use joinable::Joinable;

use super::Stream;
use crate::execution_step::lambda_applier::LambdaError;
use crate::JValue;

use air_interpreter_interface::CallResults;
use air_trace_handler::MergerApResult;
use air_trace_handler::TraceHandlerError;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

use std::rc::Rc;

/// Errors arisen while executing AIR script.
#[derive(ThisError, EnumDiscriminants, Debug)]
#[strum_discriminants(derive(EnumIter))]
pub(crate) enum ExecutionError {
    /// An error is occurred while calling local service via call_service.
    #[error("Local service error, ret_code is {0}, error message is '{1}'")]
    LocalServiceError(i32, Rc<String>),

    /// Variable with such a name wasn't defined during AIR script execution.
    #[error("variable with name '{0}' wasn't defined during script execution")]
    VariableNotFound(String),

    /// Multiple values for such name found.
    #[error("multiple variables found for name '{0}' in data")]
    MultipleVariablesFound(String),

    /// An error occurred while trying to apply lambda to a value.
    #[error(transparent)]
    LambdaApplierError(#[from] LambdaError),

    /// An error occurred while trying to apply lambda to an empty stream.
    #[error("lambda is applied to an empty stream")]
    EmptyStreamLambdaError,

    /// Provided JValue has incompatible type with a requested one.
    #[error(
        "expected JValue type '{expected_value_type}' for the variable `{variable_name}`, but got '{actual_value}'"
    )]
    IncompatibleJValueType {
        variable_name: String,
        actual_value: JValue,
        expected_value_type: &'static str,
    },

    /// Fold state wasn't found for such iterator name.
    #[error("fold state not found for this iterable '{0}'")]
    FoldStateNotFound(String),

    /// Multiple fold states found for such iterator name.
    #[error("multiple iterable values found for iterable name '{0}'")]
    MultipleIterableValues(String),

    /// A fold instruction must iterate over array value.
    #[error("lambda '{1}' returned non-array value '{0}' for fold instruction")]
    FoldIteratesOverNonArray(JValue, String),

    /// Errors encountered while shadowing non-scalar values.
    #[error("variable with name '{0}' can't be shadowed, shadowing isn't supported for iterables")]
    IterableShadowing(String),

    /// This error type is produced by a match to notify xor that compared values aren't equal.
    #[error("match is used without corresponding xor")]
    MatchWithoutXorError,

    /// This error type is produced by a mismatch to notify xor that compared values aren't equal.
    #[error("mismatch is used without corresponding xor")]
    MismatchWithoutXorError,

    /// This error type is produced by a match to notify xor that compared values aren't equal.
    #[error("fail with ret_code '{ret_code}' and error_message '{error_message}' is used without corresponding xor")]
    FailWithoutXorError { ret_code: i64, error_message: String },

    /// Errors bubbled from a trace handler.
    #[error(transparent)]
    TraceError(#[from] TraceHandlerError),

    /// Errors occurred while insertion of a value inside stream that doesn't have corresponding generation.
    #[error("stream {0:?} doesn't have generation with number {1}, probably a supplied to the interpreter data is corrupted")]
    StreamDontHaveSuchGeneration(Stream, usize),

    /// Errors occurred when result from data doesn't match to a instruction, f.e. an instruction
    /// could be applied to a stream, but result doesn't contain generation in a source position.
    #[error("ap result {0:?} doesn't match corresponding instruction")]
    ApResultNotCorrespondToInstr(MergerApResult),

    /// Call results should be empty at the end of execution thanks to a execution invariant.
    #[error(
        "after finishing execution of supplied AIR, call results aren't empty: `{0:?}`, probably wrong call_id used"
    )]
    CallResultsNotEmpty(CallResults),
}

impl From<LambdaError> for Rc<ExecutionError> {
    fn from(e: LambdaError) -> Self {
        Rc::new(ExecutionError::LambdaApplierError(e))
    }
}

/// This macro is needed because it's impossible to implement
/// From<TraceHandlerError> for Rc<ExecutionError> due to the orphan rule.
#[macro_export]
macro_rules! trace_to_exec_err {
    ($trace_expr: expr) => {
        $trace_expr.map_err(|e| std::rc::Rc::new(crate::execution_step::ExecutionError::TraceError(e)))
    };
}

impl ExecutionError {
    pub(crate) fn to_error_code(&self) -> u32 {
        const EXECUTION_ERRORS_START_ID: u32 = 1000;

        let mut errors = ExecutionErrorDiscriminants::iter();
        let actual_error_type = ExecutionErrorDiscriminants::from(self);

        // unwrap is safe here because errors are guaranteed to contain all errors variants
        let enum_variant_position = errors.position(|et| et == actual_error_type).unwrap() as u32;
        EXECUTION_ERRORS_START_ID + enum_variant_position
    }
}

macro_rules! log_join {
    ($($args:tt)*) => {
        log::info!(target: air_log_targets::JOIN_BEHAVIOUR, $($args)*)
    }
}

#[rustfmt::skip::macros(log_join)]
impl Joinable for ExecutionError {
    /// Returns true, if supplied error is related to variable not found errors type.
    /// Print log if this is joinable error type.
    fn is_joinable(&self) -> bool {
        use ExecutionError::*;

        match self {
            VariableNotFound(var_name) => {
                log_join!("  waiting for an argument with name '{}'", var_name);
                true
            }
            LambdaApplierError(LambdaError::StreamNotHaveEnoughValues { stream_size, idx }) => {
                log_join!("  waiting for an argument with idx '{}' on stream with size '{}'", idx, stream_size);
                true
            }
            EmptyStreamLambdaError => {
                log_join!("  waiting on empty stream for path ");
                true
            }

            _ => false,
        }
    }
}

impl Catchable for ExecutionError {
    fn is_catchable(&self) -> bool {
        // this kind is related to an invalid data and should treat as a non-catchable error
        !matches!(self, ExecutionError::TraceError(_))
    }
}

impl From<std::convert::Infallible> for ExecutionError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
