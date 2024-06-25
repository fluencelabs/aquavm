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
