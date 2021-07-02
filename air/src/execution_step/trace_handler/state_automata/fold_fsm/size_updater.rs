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

use super::ByNextPosition;
use super::DataKeeper;
use super::FSMResult;
use super::ResolvedFoldSubTraceLore;

/// At the end of a Fold execution it's needed to reduce subtrace_len of both sliders on a count
/// of seen states. This count is calculated during execution of a fold instruction. This struct
/// manage to save the updated lens and update subtrace_len of sliders.
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

    pub(super) fn track_before(
        &mut self,
        prev_lore: &Option<ResolvedFoldSubTraceLore>,
        current_lore: &Option<ResolvedFoldSubTraceLore>,
    ) {
        self.track(prev_lore, current_lore, ByNextPosition::Before);
    }

    pub(super) fn track_after(
        &mut self,
        prev_lore: &Option<ResolvedFoldSubTraceLore>,
        current_lore: &Option<ResolvedFoldSubTraceLore>,
    ) {
        self.track(prev_lore, current_lore, ByNextPosition::After);
    }

    fn track(
        &mut self,
        prev_lore: &Option<ResolvedFoldSubTraceLore>,
        current_lore: &Option<ResolvedFoldSubTraceLore>,
        next_position: ByNextPosition,
    ) {
        let prev_len = subtrace_len(prev_lore, next_position) as usize;
        self.prev_states_seen += prev_len;

        let current_len = subtrace_len(current_lore, next_position) as usize;
        self.current_states_seen += current_len;
    }

    pub(super) fn update(self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        let new_prev_interval = self.prev_subtrace_len - self.prev_states_seen;
        data_keeper.prev_ctx.slider.set_subtrace_len(new_prev_interval)?;

        let new_current_interval = self.current_subtrace_len - self.current_states_seen;
        data_keeper.prev_ctx.slider.set_subtrace_len(new_current_interval)?;
        Ok(())
    }
}

fn subtrace_len(subtrace_lore: &Option<ResolvedFoldSubTraceLore>, next_position: ByNextPosition) -> u32 {
    match next_position {
        ByNextPosition::Before => subtrace_lore
            .as_ref()
            .map(|l| l.before_subtrace.subtrace_len)
            .unwrap_or_default(),
        ByNextPosition::After => subtrace_lore
            .as_ref()
            .map(|l| l.after_subtrace.subtrace_len)
            .unwrap_or_default(),
    }
}
