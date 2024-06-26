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

use air_interpreter_data::GenerationIdx;

use super::ExecutionTrace;
use super::KeeperError;
use super::KeeperResult;
use super::TraceSlider;
use crate::TracePos;

/// Contains all necessary information about data.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MergeCtx {
    pub slider: TraceSlider,
}

impl MergeCtx {
    pub(crate) fn from_trace(trace: ExecutionTrace) -> Self {
        let slider = TraceSlider::new(trace);

        Self { slider }
    }

    pub(crate) fn try_get_generation(&self, position: TracePos) -> KeeperResult<GenerationIdx> {
        use air_interpreter_data::*;

        let state = self
            .slider
            .state_at_position(position)
            .ok_or_else(|| KeeperError::NoElementAtPosition {
                position,
                trace_len: self.slider.trace_len(),
            })?;

        match state {
            ExecutedState::Call(CallResult::Executed(ValueRef::Stream { generation, .. })) => Ok(*generation),
            // such Aps are always preceded by Fold where corresponding stream could be used
            // so it's been already checked that res_generation is well-formed
            // and accessing 0th element is safe here
            ExecutedState::Ap(ap_result) => Ok(ap_result.res_generations[0]),
            state => Err(KeeperError::NoStreamState { state: state.clone() }),
        }
    }
}
