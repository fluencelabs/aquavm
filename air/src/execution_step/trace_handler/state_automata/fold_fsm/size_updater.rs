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

#[derive(Debug, Default, Clone)]
pub(super) struct SubTreeSizeUpdater {
    pub(super) prev_states_seen: usize,
    pub(super) current_states_seen: usize,
    pub(super) prev_subtrace_len: usize,
    pub(super) current_subtrace_len: usize,
}

impl SubTreeSizeUpdater {
    pub(super) fn new(data_keeper: &DataKeeper) -> Self {
        let prev_subtrace_len = data_keeper.prev_ctx.slider.subtrace_len();
        let current_subtrace_len = data_keeper.current_ctx.slider.subtrace_len();

        Self {
            prev_subtrace_len,
            current_subtrace_len,
            ..<_>::default()
        }
    }

    pub(super) fn track() {}

    pub(super) fn update(self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        let new_prev_interval = self.prev_subtrace_len - self.prev_states_seen;
        data_keeper.prev_ctx.slider.set_subtrace_len(new_prev_interval)?;

        let new_current_interval = self.current_subtrace_len - self.current_states_seen;
        data_keeper.prev_ctx.slider.set_subtrace_len(new_current_interval)?;
        Ok(())
    }
}
