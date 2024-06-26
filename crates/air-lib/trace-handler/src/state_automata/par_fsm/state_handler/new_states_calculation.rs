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
use crate::data_keeper::TraceSlider;

use num_traits::CheckedAdd;

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

    assert!(std::mem::size_of_val(&par_subgraph_len) <= std::mem::size_of::<usize>());
    let new_subtrace_len = match subgraph_type {
        SubgraphType::Left => par_subgraph_len,
        SubgraphType::Right => slider
            .subtrace_len()
            .checked_sub(par_subgraph_len)
            .ok_or_else(|| StateFSMError::ParLenUnderflow(par_result, slider.subtrace_len(), MergeCtxType::Current))?,
    };

    let new_state = CtxState::new(new_position, new_subtrace_len);
    Ok(new_state)
}
