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

use super::par_fsm::SubtreeType;
use super::KeeperError;
use super::ParResult;
use super::StateFSM;
use crate::execution_step::trace_handler::MergeCtxType;
use crate::execution_step::trace_handler::ResolvedFold;

use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub(crate) enum StateFSMError {
    /// Error occurred while trying to access or pop elements from an empty queue.
    #[error("queue is empty, while fsm of type {0} is requested")]
    QueueIsEmpty(&'static str),

    /// Errors occurred while trying to access or pop elements from queue,
    /// which contains element of different type.
    #[error("queue last top element is '{1:?}', while fsm of type {0} requested")]
    IncompatibleFSM(&'static str, StateFSM),

    /// Errors occurred when ParResult.0 + ParResult.1 overflows.
    #[error("overflow is occurred while calculating the entire len occupied by executed states corresponded to current par: '{0:?}'")]
    ParLenOverflow(ParResult),

    /// Errors occurred when ParResult.0 + ParResult.1 value is bigger than current subtree size.
    #[error("par '{0:?}' contains subtree size that is bigger than current one '{1}'")]
    ParSubtreeUnderflow(ParResult, usize),

    /// Errors occurred when {0}.fold_states_count + {1} overflows.
    #[error("overflow is occurred while calculating the new position of a {2} slider for resolved fold {0:?} and current position {1}'")]
    FoldPosOverflow(ResolvedFold, usize, MergeCtxType),

    /// Errors occurred when {1} - 1{0}.fold_states_count underflows.
    #[error("overflow is occurred while calculating the new position of a {2} slider for resolved fold {0:?} and current subtrace len {1}'")]
    FoldLenUnderflow(ResolvedFold, usize, MergeCtxType),

    /// Errors occurred when a subtree of a Par instructions was finished but remaining interval isn't empty.
    #[error(
        "par {0} subtree of '{1:?}' was completed, but its interval was not fully exhausted and contains {2} elements"
    )]
    ParSubtreeNonExhausted(SubtreeType, ParResult, usize),

    /// Errors bubbled from DataKeeper.
    #[error("{0}")]
    KeeperError(#[from] KeeperError),
}
