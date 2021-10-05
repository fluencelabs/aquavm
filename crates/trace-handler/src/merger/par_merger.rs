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
    pub(self) fn from_pars(prev_par: ParResult, current_par: ParResult) -> Self {
        Self {
            prev_par: Some(prev_par),
            current_par: Some(current_par),
        }
    }

    pub(self) fn from_prev_par(prev_par: ParResult) -> Self {
        Self {
            prev_par: Some(prev_par),
            current_par: None,
        }
    }

    pub(self) fn from_current_par(current_par: ParResult) -> Self {
        Self {
            prev_par: None,
            current_par: Some(current_par),
        }
    }
}
