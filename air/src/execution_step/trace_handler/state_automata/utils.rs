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

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct CtxState {
    pub(super) pos: usize,
    pub(super) subtrace_len: usize,
    pub(super) total_subtrace_len: usize,
}

impl CtxState {
    pub(super) fn new(pos: usize, subtrace_len: usize, total_subtrace_len: usize) -> Self {
        Self {
            pos,
            subtrace_len,
            total_subtrace_len,
        }
    }
}

pub(super) fn update_with_states(prev_state: CtxState, current_state: CtxState, data_keeper: &mut DataKeeper) {
    // these calls shouldn't produce a error, because sizes become less and
    // they have been already checked in a state updater ctor. It's important
    // to make it in a such way, because this function could be called from
    // error_exit that shouldn't fail.
    let _ = data_keeper
        .prev_slider_mut()
        .set_position_and_len(prev_state.pos, prev_state.subtrace_len);
    data_keeper
        .prev_ctx
        .set_total_subtrace_len(prev_state.total_subtrace_len);

    let _ = data_keeper
        .current_slider_mut()
        .set_position_and_len(current_state.pos, current_state.subtrace_len);
    data_keeper
        .current_ctx
        .set_total_subtrace_len(current_state.subtrace_len);
}
