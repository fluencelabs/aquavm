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
use super::ParResult;
use super::StateFSM;

use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum StateFSMError {
    /// Error occurred while trying to access or pop elements from an empty queue.
    #[error("queue is empty, while fsm of type {0} requested")]
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

    /// Errors occurred when a subtree of a Par instructions was finished but remaining interval isn't empty.
    #[error(
        "par {0} subtree of '{1:?}' was completed, but interval does not fully exhausted and contains {2} elements"
    )]
    ParSubtreeNonExhausted(SubtreeType, ParResult, usize),

    /// Errors occurred when previous and current executed states are incompatible.
    #[error("previous and current data have incompatible states: '{0:?}' '{1:?}'")]
    IncompatibleExecutedStates(ExecutedState, ExecutedState),

    /// Errors occurred when previous and current call results are incompatible.
    #[error("previous and current call results are incompatible: '{0:?}' '{1:?}'")]
    IncompatibleCallResults(CallResult, CallResult),

    /// Errors occurred when executed trace contains less elements then corresponding Par has.
    #[error("executed trace has {0} elements, but {1} requires by Par")]
    ExecutedTraceTooSmall(usize, usize),

    /// Errors occurred when executed trace contains less elements then corresponding Par has.
    #[error("tried to find CallResult::Resolved, but the actual type is different")]
    IncompatibleState,

    /// Errors occurred when sum_i { FoldResult_i.interval_len } overflows.
    #[error("overflow is occurred while calculating the entire len occupied by executed states corresponded to current fold: '{0:?}'")]
    FoldLenOverflow(FoldResult),

    /// Errors occurred when sum_i { FoldResult_i.interval_len } value is bigger than current subtree size.
    #[error("fold '{0:?}' contains subtree size that is bigger than current one '{1}'")]
    FoldSubtreeUnderflow(FoldResult, usize),

    /// Errors occurred when one of the fold lores contains more then two sublores.
    #[error("fold contains {0} sublores, but 2 is expected")]
    FoldIncorrectSubtracesCount(usize),

    /// Errors occurred when one of the fold lores contains more then two sublores.
    #[error("fold sublores have different value_pos: {0}, {1}")]
    FoldIncorrectValuePos(usize, usize),

    #[error("value_pos of a FoldResult points to '{0}', but this element isn't an element of a stream")]
    FoldValuesPosNotStream(usize),
}
