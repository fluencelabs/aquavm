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

use super::CallResult;
use super::ExecutedState;
use super::FoldResult;
use super::KeeperError;

use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub(crate) enum MergeError {
    /// Errors occurred when previous and current executed states are incompatible.
    #[error("previous and current data have incompatible states: '{0:?}' '{1:?}'")]
    IncompatibleExecutedStates(ExecutedState, ExecutedState),

    /// Merger was expected to see other state that was obtained from one of traces
    /// (the other state was absent).
    #[error("state from {1} `{0:?}` is incompatible with expected {2}")]
    DifferentExecutedStateExpected(ExecutedState, DataType, &'static str),

    #[error("{0:?} contains several subtraces with the same value_pos {1}")]
    ManyRecordsWithSamePos(FoldResult, usize),

    /// Errors occurred when previous and current call results are incompatible.
    #[error("previous and current call results are incompatible: '{0:?}' '{1:?}'")]
    IncompatibleCallResults(CallResult, CallResult),

    /// Errors occurred when one of the fold subtrace lore doesn't contain 2 descriptors.
    #[error("fold contains {0} sublore descriptors, but 2 is expected")]
    FoldIncorrectSubtracesCount(usize),

    /// Errors bubbled from DataKeeper.
    #[error("{0}")]
    KeeperError(#[from] KeeperError),
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
