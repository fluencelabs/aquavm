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

pub(super) fn compute_new_states(
    data_keeper: &DataKeeper,
    prev_par: ParResult,
    current_par: ParResult,
    subtree_type: SubtreeType,
) -> FSMResult<(CtxStateNibble, CtxStateNibble)> {
    let (prev_len, current_len) = match subtree_type {
        SubtreeType::Left => (prev_par.left_size, current_par.left_size),
        SubtreeType::Right => (prev_par.right_size, current_par.right_size),
    };

    let prev_nibble = compute_new_state(data_keeper, prev_len as usize, MergeCtxType::Previous, prev_par)?;
    let current_nibble = compute_new_state(data_keeper, current_len as usize, MergeCtxType::Current, current_par)?;

    Ok((prev_nibble, current_nibble))
}

fn compute_new_state(
    data_keeper: &DataKeeper,
    par_subtree_len: usize,
    ctx_type: MergeCtxType,
    par: ParResult,
) -> FSMResult<CtxStateNibble> {
    let slider = match ctx_type {
        MergeCtxType::Previous => data_keeper.prev_slider(),
        MergeCtxType::Current => data_keeper.current_slider(),
    };

    let pos = slider
        .position()
        .checked_add(par_subtree_len)
        .ok_or_else(|| StateFSMError::ParPosOverflow(par, slider.position(), MergeCtxType::Previous))?;

    let subtrace_len = slider
        .subtrace_len()
        .checked_sub(par_subtree_len)
        .ok_or_else(|| StateFSMError::ParLenUnderflow(par, slider.subtrace_len(), MergeCtxType::Current))?;

    let nibble = CtxStateNibble::new(pos, subtrace_len);
    Ok(nibble)
}
