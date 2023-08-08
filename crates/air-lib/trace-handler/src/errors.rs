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

use air_interpreter_data::ExecutedState;
use air_interpreter_data::TracePos;
use thiserror::Error as ThisError;

use std::num::TryFromIntError;

/// Errors arose out of merging previous data with a new.
#[derive(ThisError, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum TraceHandlerError {
    #[error(transparent)]
    KeeperError(#[from] KeeperError),

    #[error(transparent)]
    MergeError(#[from] MergeError),

    #[error(transparent)]
    StateFSMError(#[from] StateFSMError),
}

#[derive(ThisError, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum GenerationCompactificationError {
    #[error("trying to change generation of an invalid trace position {0}")]
    TracePosPointsToNowhere(TracePos),

    #[error(
        "trying to change generation of a state {state} on {position} position, the state doesn't contain generation"
    )]
    TracePosPointsToInvalidState { position: TracePos, state: ExecutedState },
}

impl GenerationCompactificationError {
    pub fn points_to_nowhere(position: TracePos) -> Self {
        GenerationCompactificationError::TracePosPointsToNowhere(position)
    }

    pub fn points_to_invalid_state(position: TracePos, state: ExecutedState) -> Self {
        GenerationCompactificationError::TracePosPointsToInvalidState { position, state }
    }
}

#[derive(ThisError, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum IntConversionError {
    #[error("trying to cast integer types, there is an error {0:?}")]
    TryIntoTracePosError(TryFromIntError),
}
