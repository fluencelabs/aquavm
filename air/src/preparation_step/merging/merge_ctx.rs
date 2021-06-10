/*
 * Copyright 2020 Fluence Labs Limited
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

use super::DataMergingError;
use super::ExecutionTrace;
use super::MergeResult;
use super::TraceSlider;

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct MergeCtx {
    pub(crate) slider: TraceSlider,
    pub(crate) old_pos_to_new: HashMap<usize, usize>,
}

impl MergeCtx {
    pub(crate) fn from_trace(trace: ExecutionTrace) -> Self {
        let slider = TraceSlider::new(trace);

        Self {
            slider,
            old_pos_to_new: HashMap::new(),
        }
    }

    pub(crate) fn add_correspondence(&mut self, old_pos: usize, new_pos: usize) {
        self.old_pos_to_new.insert(old_pos, new_pos);
    }

    pub(crate) fn try_get_new_pos(&self, old_pos: usize) -> MergeResult<usize> {
        println!("old_pos_to_new: {:?}", self.old_pos_to_new);
        self.old_pos_to_new
            .get(&old_pos)
            .map(|v| *v)
            .ok_or_else(|| DataMergingError::FoldValuesPosNotStream(old_pos))
    }
}
