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

mod ap_merger;
mod call_merger;
mod canon_merger;
mod errors;
mod fold_merger;
mod par_merger;
mod position_mapping;

pub use ap_merger::MergerApResult;
pub use call_merger::MergerCallResult;
pub use call_merger::MetResult;
pub use call_merger::ValueSource;
pub use canon_merger::MergerCanonResult;
pub use fold_merger::MergerFoldResult;
pub use par_merger::MergerParResult;
pub use position_mapping::PreparationScheme;

pub use errors::ApResultError;
pub use errors::CallResultError;
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

use std::fmt;

impl fmt::Display for MergeCtxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeCtxType::Previous => write!(f, "previous"),
            MergeCtxType::Current => write!(f, "current"),
        }
    }
}
