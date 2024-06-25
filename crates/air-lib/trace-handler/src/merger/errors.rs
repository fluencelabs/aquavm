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

use super::ApResult;
use super::CallResult;
use super::ExecutedState;
use super::FoldResult;
use super::KeeperError;
use super::ValueRef;

use air_interpreter_data::CanonResult;
use air_interpreter_data::TracePos;
use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub enum MergeError {
    /// Errors occurred when previous and current executed states are incompatible.
    #[error("previous and current data have incompatible states: '{0:?}' '{1:?}'")]
    IncompatibleExecutedStates(ExecutedState, ExecutedState),

    /// Merger was expected to see other state that was obtained from one of traces
    /// (the other state was absent).
    #[error("state from {1} `{0:?}` is incompatible with expected {2}")]
    DifferentExecutedStateExpected(ExecutedState, DataType, &'static str),

    #[error(transparent)]
    KeeperError(#[from] KeeperError),

    #[error(transparent)]
    IncorrectApResult(#[from] ApResultError),

    #[error(transparent)]
    IncorrectCallResult(#[from] CallResultError),

    #[error(transparent)]
    IncorrectCanonResult(#[from] CanonResultError),

    #[error(transparent)]
    IncorrectFoldResult(#[from] FoldResultError),
}

#[derive(ThisError, Debug)]
pub enum ApResultError {
    /// Error occurred when Ap results contains not 1 generation in destination.
    #[error("{0:?} ap result contains inappropriate generation count in destination")]
    InvalidDstGenerations(ApResult),
}

#[derive(ThisError, Debug)]
pub enum CallResultError {
    #[error("values in call results are not equal: {prev_value:?} != {current_value:?}")]
    ValuesNotEqual {
        prev_value: ValueRef,
        current_value: ValueRef,
    },

    /// Errors occurred when previous and current call results are incompatible.
    #[error("previous and current call results are incompatible: '{prev_call:?}' '{current_call:?}'")]
    IncompatibleCallResults {
        prev_call: CallResult,
        current_call: CallResult,
    },
}

#[derive(ThisError, Debug)]
pub enum CanonResultError {
    #[error("canon results {prev_canon_result:?} {current_canon_result:?} points to incompatible execution states")]
    IncompatibleState {
        prev_canon_result: CanonResult,
        current_canon_result: CanonResult,
    },
}

#[derive(ThisError, Debug)]
pub enum FoldResultError {
    #[error("the first {count} subtrace descriptors lens of fold {fold_result:?} overflows")]
    SubtraceLenOverflow { fold_result: FoldResult, count: usize },

    /// There are several lores with the same value_pos.
    #[error("{0:?} contains several subtraces with the same value_pos {1}")]
    SeveralRecordsWithSamePos(FoldResult, TracePos),

    /// Errors occurred when one of the fold subtrace lore doesn't contain 2 descriptors.
    #[error("fold contains {0} sublore descriptors, but 2 is expected")]
    FoldIncorrectSubtracesCount(usize),
}

impl MergeError {
    // shouldn't be called with both Nones
    pub(crate) fn incompatible_states(
        prev_state: Option<ExecutedState>,
        current_state: Option<ExecutedState>,
        expected_state: &'static str,
    ) -> Self {
        match (prev_state, current_state) {
            (Some(prev_state), Some(current_state)) => {
                MergeError::IncompatibleExecutedStates(prev_state, current_state)
            }
            (None, Some(current_state)) => {
                MergeError::DifferentExecutedStateExpected(current_state, DataType::Current, expected_state)
            }
            (Some(prev_state), None) => {
                MergeError::DifferentExecutedStateExpected(prev_state, DataType::Previous, expected_state)
            }
            (None, None) => unreachable!("shouldn't be called with both None"),
        }
    }
}

// these impl methods allow construction of MergeError and are used to make code more clean
impl CallResultError {
    pub(crate) fn not_equal_values(prev_value: ValueRef, current_value: ValueRef) -> MergeError {
        let call_result_error = CallResultError::ValuesNotEqual {
            prev_value,
            current_value,
        };

        MergeError::IncorrectCallResult(call_result_error)
    }

    pub(crate) fn incompatible_calls(prev_call: CallResult, current_call: CallResult) -> MergeError {
        let call_result_error = CallResultError::IncompatibleCallResults {
            prev_call,
            current_call,
        };

        MergeError::IncorrectCallResult(call_result_error)
    }
}

impl CanonResultError {
    pub(crate) fn incompatible_state(prev_canon_result: CanonResult, current_canon_result: CanonResult) -> Self {
        Self::IncompatibleState {
            prev_canon_result,
            current_canon_result,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Previous,
    Current,
}

use std::fmt;

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Previous => write!(f, "previous"),
            DataType::Current => write!(f, "current"),
        }
    }
}
