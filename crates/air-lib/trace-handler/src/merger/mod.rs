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

mod ap_merger;
mod call_merger;
mod canon_merger;
mod errors;
mod fold_merger;
mod par_merger;
mod position_mapping;

pub use ap_merger::MergerApResult;
pub use ap_merger::MetApResult;
pub use call_merger::MergerCallResult;
pub use call_merger::MetCallResult;
pub use canon_merger::MergerCanonResult;
pub use fold_merger::MergerFoldResult;
pub use par_merger::MergerParResult;
pub use position_mapping::PreparationScheme;

pub use errors::ApResultError;
pub use errors::CallResultError;
pub use errors::CanonResultError;
pub use errors::DataType;
pub use errors::FoldResultError;
pub use errors::MergeError;

pub use fold_merger::ResolvedFold;
pub use fold_merger::ResolvedSubTraceDescs;

pub(super) use ap_merger::try_merge_next_state_as_ap;
pub(super) use call_merger::try_merge_next_state_as_call;
pub(super) use canon_merger::try_merge_next_state_as_canon;
pub(crate) use fold_merger::try_merge_next_state_as_fold;
pub(crate) use par_merger::try_merge_next_state_as_par;

use position_mapping::prepare_positions_mapping;

type MergeResult<T> = std::result::Result<T, MergeError>;

use super::data_keeper::KeeperError;
use super::DataKeeper;

use air_interpreter_data::*;

#[derive(Debug, Copy, Clone)]
pub enum MergeCtxType {
    Current,
    Previous,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueSource {
    PreviousData,
    CurrentData,
}

use std::fmt;

impl fmt::Display for MergeCtxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeCtxType::Previous => write!(f, "previous"),
            MergeCtxType::Current => write!(f, "current"),
        }
    }
}
