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

mod errors;
mod fold_fsm;
mod fsm_queue;
mod par_fsm;
mod state_inserter;
mod utils;

pub(crate) use errors::StateFSMError;
pub(crate) use fold_fsm::ValueAndPos;
pub(crate) use par_fsm::SubtreeType;
pub(crate) type FSMResult<T> = std::result::Result<T, StateFSMError>;

pub(super) use fold_fsm::FoldFSM;
pub(super) use fsm_queue::FSMKeeper;
pub(super) use par_fsm::ParFSM;

use super::data_keeper::KeeperError;
use super::merger::MergerParResult;
use super::DataKeeper;
use super::ExecutedState;
use super::FoldResult;
use super::FoldSubTraceLore;
use super::MergeCtxType;
use super::MergerFoldResult;
use super::ParResult;
use super::ResolvedFold;
use super::ResolvedFoldSubTraceLore;
use state_inserter::StateInserter;
use utils::*;
