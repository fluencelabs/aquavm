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

use super::FoldLoreCtor;
use super::FoldSubTraceLore;
use super::ResolvedFoldSubTraceLore;

#[derive(Debug, Default, Clone)]
pub(super) struct LoreCtorQueue {
    queue: Vec<LoreCtorElement>,
    back_traversal_pos: usize,
}

impl LoreCtorQueue {
    pub(super) fn straight_ctor_mut(&mut self) -> &mut LoreCtorElement {
        &mut self.queue.last_mut().unwrap()
    }

    pub(super) fn backward_ctor_mut(&mut self) -> &mut LoreCtorElement {
        &mut self.queue[self.back_traversal_pos - 1]
    }

    pub(super) fn add_element(&mut self, element: LoreCtorElement) {
        self.queue.push(element);
        self.back_traversal_pos += 1;
    }

    pub(super) fn backward_traverse(&mut self) {
        self.back_traversal_pos -= 1;
    }

    pub(super) fn were_no_back_traversals(&self) -> bool {
        self.queue.len() == self.back_traversal_pos
    }

    pub(super) fn transform_to_subtale(&mut self) -> Vec<Vec<FoldSubTraceLore>> {
        self.queue
            .drain(..)
            .map(|l| l.lore_ctor.into_subtrace())
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
pub(super) struct LoreCtorElement {
    pub(super) prev_lore: Option<ResolvedFoldSubTraceLore>,
    pub(super) current_lore: Option<ResolvedFoldSubTraceLore>,
    pub(super) lore_ctor: FoldLoreCtor,
}
