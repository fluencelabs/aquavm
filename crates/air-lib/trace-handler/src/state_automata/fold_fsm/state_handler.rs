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
use crate::merger::MergeCtxType;
use crate::ResolvedFold;

use num_traits::ops::checked::CheckedAdd;

/// This state updater manage to do the same thing as CtxStateHandler in ParFSM,
/// for details please see its detailed comment.
#[derive(Debug, Default, Clone)]
pub(super) struct CtxStateHandler {
    state_pair: CtxStatesPair,
}

impl CtxStateHandler {
    pub(super) fn prepare(
        prev_fold: &ResolvedFold,
        current_fold: &ResolvedFold,
        data_keeper: &DataKeeper,
    ) -> FSMResult<Self> {
        let prev_state = compute_new_state(prev_fold, data_keeper, MergeCtxType::Previous)?;
        let current_state = compute_new_state(current_fold, data_keeper, MergeCtxType::Current)?;
        let state_pair = CtxStatesPair::new(prev_state, current_state);

        let updater = Self { state_pair };
        Ok(updater)
    }

    pub(super) fn set_final_states(self, data_keeper: &mut DataKeeper) {
        update_ctx_states(self.state_pair, data_keeper)
    }
}

fn compute_new_state(fold: &ResolvedFold, data_keeper: &DataKeeper, ctx_type: MergeCtxType) -> FSMResult<CtxState> {
    let ctx = match ctx_type {
        MergeCtxType::Previous => &data_keeper.prev_ctx,
        MergeCtxType::Current => &data_keeper.current_ctx,
    };

    let current_position = ctx.slider.position();
    let pos = current_position
        .checked_add(&fold.fold_states_count.into())
        .ok_or_else(|| StateFSMError::FoldPosOverflow(fold.clone(), current_position, ctx_type))?;

    let current_len = ctx.slider.subtrace_len();
    let subtrace_len = current_len
        .checked_sub(fold.fold_states_count)
        // TODO judging by the error message, one should pass current_len instead.
        .ok_or_else(|| StateFSMError::FoldLenUnderflow(fold.clone(), current_position, ctx_type))?;

    let state = CtxState::new(pos, subtrace_len);
    Ok(state)
}
