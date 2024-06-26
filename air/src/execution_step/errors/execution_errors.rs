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

use super::CatchableError;
use super::ErrorAffectable;
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
                CatchableError::MatchValuesNotEqual | CatchableError::MismatchValuesEqual
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
            $crate::execution_step::ExecutionError::Uncatchable($crate::execution_step::UncatchableError::TraceError {
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
            ExecutionError::Uncatchable(_) => false,
        }
    }
}

impl ErrorAffectable for ExecutionError {
    fn affects_last_error(&self) -> bool {
        match self {
            ExecutionError::Catchable(err) => err.affects_last_error(),
            ExecutionError::Uncatchable(_) => false,
        }
    }

    fn affects_error(&self) -> bool {
        match self {
            ExecutionError::Catchable(_) => true,
            ExecutionError::Uncatchable(_) => false,
        }
    }
}
