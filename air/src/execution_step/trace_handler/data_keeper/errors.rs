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

use crate::contexts::execution_trace::*;

use thiserror::Error as ThisError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum KeeperError {
    /// Errors occurred when executed trace contains less elements then corresponding Par has.
    #[error("executed trace has {0} elements, but {1} requires by Par")]
    ExecutedTraceTooSmall(usize, usize),

    /// Errors occurred when data contains no generation for stream with the following name.
    #[error("data doesn't contain generation for stream with name '{0}'")]
    NoSuchStream(String),

    /// Errors occurred when there were no correspondence between new position and old position.
    #[error("context doesn't contain correspondence for position {0}")]
    NoSuchCorrespondence(usize),
}
