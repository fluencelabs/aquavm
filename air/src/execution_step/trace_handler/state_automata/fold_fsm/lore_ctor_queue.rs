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

use super::FoldLore;
use super::ResolvedFoldSubTraceLore;
use super::SubTraceLoreCtor;

/// This queue emulates behaviour of fold states traversal:
///  - at first states are traversal forward until the end of states
///  - then states are traversal backward the same times
#[derive(Debug, Default, Clone)]
pub(super) struct SubTraceLoreCtorQueue {
    queue: Vec<LoreCtorDesc>,
    back_traversal_pos: usize,
}

impl SubTraceLoreCtorQueue {
    pub(super) fn forward_ctor_mut(&mut self) -> Option<&mut LoreCtorDesc> {
        self.queue.last_mut()
    }

    pub(super) fn backward_ctor_mut(&mut self) -> &mut LoreCtorDesc {
        &mut self.queue[self.back_traversal_pos - 1]
    }

    pub(super) fn add_element(&mut self, element: LoreCtorDesc) {
        self.queue.push(element);
        self.back_traversal_pos += 1;
    }

    pub(super) fn backward_traverse(&mut self) {
        self.back_traversal_pos -= 1;
    }

    pub(super) fn were_no_back_traversals(&self) -> bool {
        self.queue.len() == self.back_traversal_pos
    }

    pub(super) fn transform_to_lore(&mut self) -> FoldLore {
        self.queue
            .drain(..)
            .map(|l| l.ctor.into_subtrace_lore())
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
pub(super) struct LoreCtorDesc {
    pub(super) prev_lore: Option<ResolvedFoldSubTraceLore>,
    pub(super) current_lore: Option<ResolvedFoldSubTraceLore>,
    pub(super) ctor: SubTraceLoreCtor,
}
