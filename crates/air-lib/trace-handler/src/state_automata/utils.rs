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
use super::MergeCtx;
use crate::TracePos;

pub type TraceLen = u32;

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
