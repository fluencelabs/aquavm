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

use super::*;
use ExecutedState::Par;

#[derive(Default, Debug, Copy, Clone)]
pub struct MergerParResult {
    pub prev_par: Option<ParResult>,
    pub current_par: Option<ParResult>,
}

pub(crate) fn try_merge_next_state_as_par(data_keeper: &mut DataKeeper) -> MergeResult<MergerParResult> {
    let prev_state = data_keeper.prev_slider_mut().next_state();
    let current_state = data_keeper.current_slider_mut().next_state();

    let result = match (prev_state, current_state) {
        (Some(Par(prev_par)), Some(Par(current_par))) => MergerParResult::from_pars(prev_par, current_par),
        (None, Some(Par(current_par))) => MergerParResult::from_current_par(current_par),
        (Some(Par(prev_par)), None) => MergerParResult::from_prev_par(prev_par),
        (None, None) => MergerParResult::default(),
        (prev_state, current_state) => return Err(MergeError::incompatible_states(prev_state, current_state, "par")),
    };

    Ok(result)
}

impl MergerParResult {
    fn from_pars(prev_par: ParResult, current_par: ParResult) -> Self {
        Self {
            prev_par: Some(prev_par),
            current_par: Some(current_par),
        }
    }

    fn from_prev_par(prev_par: ParResult) -> Self {
        Self {
            prev_par: Some(prev_par),
            current_par: None,
        }
    }

    fn from_current_par(current_par: ParResult) -> Self {
        Self {
            prev_par: None,
            current_par: Some(current_par),
        }
    }
}
