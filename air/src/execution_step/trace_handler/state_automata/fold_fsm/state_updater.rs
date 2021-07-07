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
use crate::execution_step::trace_handler::ResolvedFold;

/// This state updater manage to do the same thing as SubTreeStateUpdater in ParFSM,
/// for details please see its detailed comment.
#[derive(Debug, Default, Clone)]
pub(super) struct SubTreeStateUpdater {
    prev_pos: usize,
    prev_size: usize,
    current_pos: usize,
    current_size: usize,
}

impl SubTreeStateUpdater {
    pub(super) fn new(prev_fold: &ResolvedFold, current_fold: &ResolvedFold, data_keeper: &DataKeeper) -> Self {
        // TODO: check for overflow
        let prev_pos = data_keeper.prev_slider().position() + prev_fold.fold_states_count;
        let prev_size = data_keeper.prev_slider().subtrace_len() - prev_fold.fold_states_count;

        let current_pos = data_keeper.current_slider().position() + current_fold.fold_states_count;
        let current_size = data_keeper.current_slider().subtrace_len() - current_fold.fold_states_count;

        Self {
            prev_pos,
            prev_size,
            current_pos,
            current_size,
        }
    }

    pub(super) fn update(self, data_keeper: &mut DataKeeper) {
        // these calls shouldn't produce a error, because sizes become less and
        // they have been already checked in the ctor. It's important to make it
        // in a such way, because this functions is called from error_exit that
        // shouldn't fail.
        let _ = data_keeper
            .prev_slider_mut()
            .set_position_and_len(self.prev_pos, self.prev_size);
        let _ = data_keeper
            .current_slider_mut()
            .set_position_and_len(self.current_pos, self.current_size);
    }
}
