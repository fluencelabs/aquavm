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

use super::KeeperError;
use super::ParResult;
use crate::merger::MergeCtxType;
use crate::ResolvedFold;
use crate::TracePos;

use air_interpreter_data::TraceLen;
use thiserror::Error as ThisError;

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

    /// Errors occurred when {1} - 1{0}.fold_states_count underflows.
    #[error("underflow is occurred while calculating the new position of a {2} slider for resolved fold {0:?} and current subtrace len {1}'")]
    FoldLenUnderflow(ResolvedFold, TracePos, MergeCtxType),

    /// Errors bubbled from DataKeeper.
    #[error(transparent)]
    KeeperError(#[from] KeeperError),
}
