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

use super::ErrorAffectable;
use super::Joinable;
use crate::execution_step::execution_context::errors::StreamMapError;
use crate::execution_step::execution_context::ErrorObjectError;
use crate::execution_step::lambda_applier::LambdaError;
use crate::JValue;
use crate::ToErrorCode;

use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

use std::rc::Rc;

/// Catchable errors arisen during AIR script execution. Catchable here means that these errors
/// could be handled by a xor instruction and their error_code could be used in a match
/// instruction.
#[derive(ThisError, EnumDiscriminants, Debug, Clone)]
#[strum_discriminants(derive(EnumIter))]
pub enum CatchableError {
    /// An error is occurred while calling local service via call_service.
    #[error("Local service error, ret_code is {0}, error message is '{1}'")]
    LocalServiceError(i32, Rc<String>),

    /// This error type is produced by a match to notify xor that compared values aren't equal.
    #[error("compared values do not match")]
    MatchValuesNotEqual,

    /// This error type is produced by a mismatch to notify xor that compared values aren't equal.
    #[error("compared values do not mismatch")]
    MismatchValuesEqual,

    /// Variable with such a name wasn't defined during AIR script execution.
    /// This error type is used in order to support the join behaviour and
    /// it's ok if some variable hasn't been defined yet, due to the par nature of AIR.
    #[error("variable with name '{0}' wasn't defined during script execution")]
    VariableNotFound(String),

    /// Provided JValue has incompatible type with a requested one.
    #[error(
        "expected JValue type '{expected_value_type}' for the variable `{variable_name}`, but got '{actual_value}'"
    )]
    IncompatibleJValueType {
        variable_name: String,
        actual_value: JValue,
        expected_value_type: &'static str,
    },

    /// A fold instruction must iterate over array value.
    #[error("expression '{1}' returned non-array value '{0}' for fold iterable")]
    FoldIteratesOverNonArray(JValue, String),

    /// This error type is produced by a fail instruction.
    #[error("fail with '{error}' is used without corresponding xor")]
    UserError { error: JValue },

    /// An error occurred while trying to apply lambda to a value.
    #[error(transparent)]
    LambdaApplierError(#[from] LambdaError),

    /// This error type is produced by a fail instruction that tries to throw a scalar that have inappropriate type.
    #[error(transparent)]
    InvalidErrorObjectError(#[from] ErrorObjectError),

    /// A new with this variable name was met and right after that it was accessed
    /// that is prohibited.
    #[error("variable with name '{0}' was cleared by new and then wasn't set")]
    VariableWasNotInitializedAfterNew(String),

    /// This error type is occurred when the length functor applied to a value of non-array type.
    #[error("the length functor could applied only to an array-like value, but it's applied to '{0}'")]
    LengthFunctorAppliedToNotArray(JValue),

    /// Call gets non-string JValue resolving triplet parts.
    #[error("call cannot resolve non-String triplet variable part `{variable_name}` with value '{actual_value}'")]
    NonStringValueInTripletResolution {
        variable_name: String,
        actual_value: JValue,
    },

    /// Stream map related errors.
    #[error(transparent)]
    StreamMapError(#[from] StreamMapError),
}

impl From<LambdaError> for Rc<CatchableError> {
    fn from(e: LambdaError) -> Self {
        Rc::new(CatchableError::LambdaApplierError(e))
    }
}

impl ToErrorCode for Rc<CatchableError> {
    fn to_error_code(&self) -> i64 {
        self.as_ref().to_error_code()
    }
}

impl ToErrorCode for CatchableError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::CATCHABLE_ERRORS_START_ID;
        crate::generate_to_error_code!(self, CatchableError, CATCHABLE_ERRORS_START_ID)
    }
}

impl ErrorAffectable for CatchableError {
    fn affects_last_error(&self) -> bool {
        !matches!(
            self,
            CatchableError::MatchValuesNotEqual | CatchableError::MismatchValuesEqual
        )
    }

    fn affects_error(&self) -> bool {
        true
    }
}

macro_rules! log_join {
    ($($args:tt)*) => {
        log::trace!(target: air_log_targets::JOIN_BEHAVIOUR, $($args)*)
    }
}

#[rustfmt::skip::macros(log_join)]
impl Joinable for CatchableError {
    /// Returns true, if supplied error is related to variable not found errors type.
    /// Print log if this is joinable error type.
    fn is_joinable(&self) -> bool {
        use CatchableError::*;

        match self {
            VariableNotFound(var_name) => {
                log_join!("  waiting for an argument with name '{}'", var_name);
                true
            }
            _ => false,
        }
    }
}
