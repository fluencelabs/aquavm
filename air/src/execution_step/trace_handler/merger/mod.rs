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

mod call_merger;
mod errors;
mod fold_merger;
mod par_merger;

pub(super) use call_merger::try_merge_next_state_as_call;
pub(crate) use call_merger::MergerCallResult;
pub(crate) use errors::MergeError;
pub(crate) use fold_merger::try_merge_next_state_as_fold;
pub(crate) use fold_merger::MergerFoldResult;
pub(crate) use fold_merger::ResolvedFold;
pub(crate) use fold_merger::ResolvedSubTraceDescs;
pub(crate) use par_merger::try_merge_next_state_as_par;
pub(crate) use par_merger::MergerParResult;

pub(self) type MergeResult<T> = std::result::Result<T, MergeError>;

pub(self) use super::data_keeper::KeeperError;
pub(self) use super::DataKeeper;
pub(self) use super::TraceSlider;

use air_interpreter_data::*;

#[derive(Debug, Copy, Clone)]
pub(crate) enum MergeCtxType {
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
