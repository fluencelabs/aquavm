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

use super::KeeperError;
use super::ParResult;
use super::TraceLen;
use crate::merger::MergeCtxType;
use crate::ResolvedFold;
use crate::TracePos;

use thiserror::Error as ThisError;

use std::num::TryFromIntError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
pub enum StateFSMError {
    /// Error occurred while trying to access or pop elements from an empty par queue.
    #[error("par queue is empty, while par FSM is requested")]
    ParQueueIsEmpty,

    /// Errors occurred while trying to access or pop elements from queue,
    /// which contains element of different type.
    #[error("fold FSM for fold id {0} wasn't found")]
    FoldFSMNotFound(u32),

    /// Errors occurred when ParResult.0 + ParResult.1 overflows.
    #[error("overflow is occurred while calculating the entire len occupied by executed states corresponded to current par: '{0:?}'")]
    ParLenOverflow(ParResult),

    /// Errors occurred when slider.position() + ParResult.0 + ParResult.1 overflows.
    #[error("overflow is occurred while calculating the new position of a {2} slider for resolved par {0:?} and current position {1}'")]
    ParPosOverflow(ParResult, TracePos, MergeCtxType),

    /// Errors occurred when ParResult.0 + ParResult.1 value is bigger than current subgraph size.
    #[error("underflow is occurred while calculating the new position of a {2} slider for resolved par {0:?} and current subtrace len {1}'")]
    ParLenUnderflow(ParResult, TraceLen, MergeCtxType),

    /// Errors occurred when {0}.fold_states_count + {1} overflows.
    #[error("overflow is occurred while calculating the new position of a {2} slider for resolved fold {0:?} and current position {1}'")]
    FoldPosOverflow(ResolvedFold, TracePos, MergeCtxType),

    /// Errors occurred when {0}.fold_states_count + {1} overflows.
    #[error("error: {0:?} converting fold states count into suitable representation for resolved fold {1:?} and current position {2}'")]
    FoldStatesCountOverflow(TryFromIntError, ResolvedFold, MergeCtxType),

    /// Errors occurred when {1} - 1{0}.fold_states_count underflows.
    #[error("underflow is occurred while calculating the new position of a {2} slider for resolved fold {0:?} and current subtrace len {1}'")]
    FoldLenUnderflow(ResolvedFold, TracePos, MergeCtxType),

    /// Errors bubbled from DataKeeper.
    #[error(transparent)]
    KeeperError(#[from] KeeperError),
}
