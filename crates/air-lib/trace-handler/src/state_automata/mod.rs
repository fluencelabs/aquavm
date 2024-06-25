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

mod errors;
mod fold_fsm;
mod fsm_queue;
mod par_fsm;
mod state_inserter;
mod utils;

pub use errors::StateFSMError;
pub use par_fsm::SubgraphType;

pub(crate) type FSMResult<T> = std::result::Result<T, StateFSMError>;

pub(super) use fold_fsm::FoldFSM;
pub(super) use fsm_queue::FSMKeeper;
pub(super) use par_fsm::ParFSM;

use super::data_keeper::KeeperError;
use super::merger::MergeCtxType;
use super::merger::MergerParResult;
use super::DataKeeper;
use super::ExecutedState;
use super::FoldResult;
use super::FoldSubTraceLore;
use super::MergeCtx;
use super::MergerFoldResult;
use super::ParResult;
use super::ResolvedFold;
use super::ResolvedSubTraceDescs;
use state_inserter::StateInserter;
use utils::*;
