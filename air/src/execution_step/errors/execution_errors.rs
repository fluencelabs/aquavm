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

use super::CatchableError;
use super::Joinable;
use super::UncatchableError;
use crate::ToErrorCode;

use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

use std::rc::Rc;

// TODO: add tests for all execution errors
/// Errors arisen while executing AIR script.
/// This enum is pub since it's used in tests.
#[derive(ThisError, EnumDiscriminants, Debug)]
#[strum_discriminants(derive(EnumIter))]
pub enum ExecutionError {
    #[error(transparent)]
    Catchable(#[from] Rc<CatchableError>),

    #[error(transparent)]
    Uncatchable(#[from] UncatchableError),
}

impl ExecutionError {
    pub fn is_catchable(&self) -> bool {
        matches!(self, ExecutionError::Catchable(_))
    }

    pub fn is_match_or_mismatch(&self) -> bool {
        match self {
            ExecutionError::Catchable(catchable) => matches!(
                catchable.as_ref(),
                CatchableError::MatchWithoutXorError | CatchableError::MismatchWithoutXorError
            ),
            _ => false,
        }
    }
}

impl From<CatchableError> for ExecutionError {
    fn from(catchable: CatchableError) -> Self {
        Self::Catchable(std::rc::Rc::new(catchable))
    }
}

#[macro_export]
macro_rules! trace_to_exec_err {
    ($trace_expr: expr, $instruction: ident) => {
        $trace_expr.map_err(|trace_error| {
            crate::execution_step::ExecutionError::Uncatchable(crate::execution_step::UncatchableError::TraceError {
                trace_error,
                instruction: $instruction.to_string(),
            })
        })
    };
}
impl ToErrorCode for ExecutionError {
    fn to_error_code(&self) -> i64 {
        match self {
            ExecutionError::Catchable(err) => err.to_error_code(),
            ExecutionError::Uncatchable(err) => err.to_error_code(),
        }
    }
}

impl Joinable for ExecutionError {
    fn is_joinable(&self) -> bool {
        match self {
            ExecutionError::Catchable(err) => err.is_joinable(),
            _ => false,
        }
    }
}
