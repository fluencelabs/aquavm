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

use air_interpreter_data::TraceLen;

use super::DataKeeper;
use super::FSMResult;
use super::MergeCtx;
use crate::TracePos;

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct CtxState {
    pub(super) pos: TracePos,
    pub(super) subtrace_len: TraceLen,
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct CtxStatesPair {
    pub(super) prev_state: CtxState,
    pub(super) current_state: CtxState,
}

impl CtxState {
    pub(super) fn new(pos: TracePos, subtrace_len: TraceLen) -> Self {
        Self { pos, subtrace_len }
    }

    pub(super) fn update_ctx_state(self, ctx: &mut MergeCtx) -> FSMResult<()> {
        ctx.slider
            .set_position_and_len(self.pos, self.subtrace_len)
            .map_err(Into::into)
    }
}

impl CtxStatesPair {
    pub(super) fn new(prev_state: CtxState, current_state: CtxState) -> Self {
        Self {
            prev_state,
            current_state,
        }
    }
}

pub(super) fn update_ctx_states(state_pair: CtxStatesPair, data_keeper: &mut DataKeeper) {
    // these calls shouldn't produce a error, because sizes become less and
    // they have been already checked in a state updater ctor. It's important
    // to make it in a such way, because this function could be called from
    // error_exit that shouldn't fail.
    let _ = state_pair.prev_state.update_ctx_state(&mut data_keeper.prev_ctx);
    let _ = state_pair.current_state.update_ctx_state(&mut data_keeper.current_ctx);
}
