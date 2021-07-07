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

use super::DataKeeper;
use super::FSMResult;
use super::StateFSMError;
use crate::execution_step::trace_handler::MergeCtxType;
use crate::execution_step::trace_handler::ResolvedFold;

/// This state updater manage to do the same thing as SubTreeStateUpdater in ParFSM,
/// for details please see its detailed comment.
#[derive(Debug, Default, Clone)]
pub(super) struct SubTreeStateUpdater {
    prev_pos: usize,
    prev_len: usize,
    current_pos: usize,
    current_len: usize,
}

impl SubTreeStateUpdater {
    pub(super) fn new(
        prev_fold: &ResolvedFold,
        current_fold: &ResolvedFold,
        data_keeper: &DataKeeper,
    ) -> FSMResult<Self> {
        let (prev_pos, prev_len) = compute_new_pos_and_len(prev_fold, data_keeper, MergeCtxType::Previous)?;
        let (current_pos, current_len) = compute_new_pos_and_len(current_fold, data_keeper, MergeCtxType::Current)?;

        let updater = Self {
            prev_pos,
            prev_len,
            current_pos,
            current_len,
        };

        Ok(updater)
    }

    pub(super) fn update(self, data_keeper: &mut DataKeeper) {
        // these calls shouldn't produce a error, because sizes become less and
        // they have been already checked in the ctor. It's important to make it
        // in a such way, because this functions is called from error_exit that
        // shouldn't fail.
        let _ = data_keeper
            .prev_slider_mut()
            .set_position_and_len(self.prev_pos, self.prev_len);
        let _ = data_keeper
            .current_slider_mut()
            .set_position_and_len(self.current_pos, self.current_len);
    }
}

fn compute_new_pos_and_len(
    fold: &ResolvedFold,
    data_keeper: &DataKeeper,
    ctx_type: MergeCtxType,
) -> FSMResult<(usize, usize)> {
    let slider = match ctx_type {
        MergeCtxType::Previous => data_keeper.prev_slider(),
        MergeCtxType::Current => data_keeper.current_slider(),
    };

    let current_position = slider.position();
    let current_len = slider.subtrace_len();

    let position = current_position
        .checked_add(fold.fold_states_count)
        .ok_or(StateFSMError::FoldPosOverflow(fold.clone(), current_position, ctx_type))?;

    let len = current_len
        .checked_sub(fold.fold_states_count)
        .ok_or(StateFSMError::FoldLenUnderflow(
            fold.clone(),
            current_position,
            ctx_type,
        ))?;

    Ok((position, len))
}
