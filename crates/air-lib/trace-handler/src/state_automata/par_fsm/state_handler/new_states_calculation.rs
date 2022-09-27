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
use crate::data_keeper::TraceSlider;

pub(super) fn compute_new_states(
    data_keeper: &DataKeeper,
    prev_par: ParResult,
    current_par: ParResult,
    subgraph_type: SubgraphType,
) -> FSMResult<CtxStatesPair> {
    let (prev_len, current_len) = match subgraph_type {
        SubgraphType::Left => (prev_par.left_size, current_par.left_size),
        SubgraphType::Right => {
            let prev_par_size = prev_par.size().ok_or(StateFSMError::ParLenOverflow(prev_par))?;
            let current_par_size = current_par.size().ok_or(StateFSMError::ParLenOverflow(current_par))?;

            (prev_par_size as u32, current_par_size as u32)
        }
    };

    let mut prev_state = compute_new_state(prev_len as usize, data_keeper.prev_slider(), prev_par)?;
    let mut current_state = compute_new_state(current_len as usize, data_keeper.current_slider(), current_par)?;

    if matches!(subgraph_type, SubgraphType::Left) {
        prev_state.subtrace_len = prev_par.right_size as usize;
        current_state.subtrace_len = current_par.right_size as usize;
    }

    let pair = CtxStatesPair::new(prev_state, current_state);
    Ok(pair)
}

fn compute_new_state(par_subgraph_len: usize, slider: &TraceSlider, par: ParResult) -> FSMResult<CtxState> {
    let pos = slider
        .position()
        .checked_add(par_subgraph_len)
        .ok_or_else(|| StateFSMError::ParPosOverflow(par, slider.position(), MergeCtxType::Previous))?;

    let subtrace_len = slider
        .subtrace_len()
        .checked_sub(par_subgraph_len)
        .ok_or_else(|| StateFSMError::ParLenUnderflow(par, slider.subtrace_len(), MergeCtxType::Current))?;

    let new_state = CtxState::new(pos, subtrace_len);
    Ok(new_state)
}
