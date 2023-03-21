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

use num_traits::CheckedAdd;

use super::*;
use crate::data_keeper::TraceSlider;

pub(super) fn compute_new_states(
    data_keeper: &DataKeeper,
    prev_par: ParResult,
    current_par: ParResult,
    subgraph_type: SubgraphType,
) -> FSMResult<CtxStatesPair> {
    let prev_state = compute_new_state(prev_par, subgraph_type, data_keeper.prev_slider())?;
    let current_state = compute_new_state(current_par, subgraph_type, data_keeper.current_slider())?;

    let pair = CtxStatesPair::new(prev_state, current_state);
    Ok(pair)
}

fn compute_new_state(par_result: ParResult, subgraph_type: SubgraphType, slider: &TraceSlider) -> FSMResult<CtxState> {
    let par_subgraph_len = match subgraph_type {
        SubgraphType::Left => par_result.left_size,
        SubgraphType::Right => par_result.size().ok_or(StateFSMError::ParLenOverflow(par_result))?,
    };

    let new_position = slider
        .position()
        .checked_add(&air_interpreter_data::TracePos::from(par_subgraph_len))
        .ok_or_else(|| StateFSMError::ParPosOverflow(par_result, slider.position(), MergeCtxType::Previous))?;

    let new_subtrace_len = match subgraph_type {
        SubgraphType::Left => par_subgraph_len as usize,
        SubgraphType::Right => slider
            .subtrace_len()
            .checked_sub(par_subgraph_len as usize)
            .ok_or_else(|| StateFSMError::ParLenUnderflow(par_result, slider.subtrace_len(), MergeCtxType::Current))?,
    };

    let new_state = CtxState::new(new_position, new_subtrace_len);
    Ok(new_state)
}
