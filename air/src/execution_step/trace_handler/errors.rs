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

use super::data_keeper::KeeperError;
use super::merger::MergeError;
use super::state_automata::StateFSMError;

use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum TraceHandlerError {
    #[error("{0}")]
    KeeperError(#[from] KeeperError),

    #[error("{0}")]
    MergeError(#[from] MergeError),

    #[error("{0}")]
    StateFSMError(#[from] StateFSMError),
}
