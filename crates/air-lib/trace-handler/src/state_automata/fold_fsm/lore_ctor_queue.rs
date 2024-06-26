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

use super::DataKeeper;
use super::FoldLore;
use super::ResolvedSubTraceDescs;
use super::SubTraceLoreCtor;

/// This queue emulates behaviour of fold states traversal:
///  - at first states are traversal forward until the end of states
///  - then states are traversal backward the same times
#[derive(Debug, Default, Clone)]
pub(super) struct SubTraceLoreCtorQueue {
    queue: Vec<LoreCtorDesc>,
    back_traversal_pos: usize,
    back_traversal_started: bool,
}

impl SubTraceLoreCtorQueue {
    pub(super) fn current(&mut self) -> &mut LoreCtorDesc {
        &mut self.queue[self.back_traversal_pos - 1]
    }

    pub(super) fn add_element(
        &mut self,
        ctor: SubTraceLoreCtor,
        prev_lore: Option<ResolvedSubTraceDescs>,
        current_lore: Option<ResolvedSubTraceDescs>,
    ) {
        let new_element = LoreCtorDesc {
            ctor,
            prev_lore,
            current_lore,
        };
        self.queue.push(new_element);
        self.back_traversal_pos += 1;
    }

    pub(super) fn traverse_back(&mut self) {
        self.back_traversal_pos -= 1;
    }

    pub(super) fn start_back_traverse(&mut self) {
        self.back_traversal_started = true;
    }

    pub(super) fn end_back_traverse(&mut self) {
        self.back_traversal_started = false;
    }

    pub(super) fn back_traversal_started(&self) -> bool {
        self.back_traversal_started
    }

    pub(super) fn transform_to_lore(&mut self) -> FoldLore {
        self.queue
            .drain(..)
            .map(|l| l.ctor.into_subtrace_lore())
            .collect::<Vec<_>>()
    }

    // this function should be called in a case of early exit from fold, f.e.
    // in last error bubbling or join behaviour in the following situations:
    //    (fold iterable iterator
    //      (seq
    //        (call .. [joined_variable])
    //        (next iterator)
    //      )
    //    )
    //
    // In such example next wouldn't be called and correspondingly all pushed to
    // ctor queue states wouldn't be properly finished. This function serves such
    // situations, having called from generation_end.
    pub(super) fn finish(&mut self, data_keeper: &DataKeeper) {
        // TODO: optimize it
        for ctor in self.queue.iter_mut() {
            ctor.ctor.finish(data_keeper);
        }

        // set this to zero to correspond that all states were "observed" with back traversal
        self.back_traversal_pos = 0;
    }
}

#[derive(Debug, Clone)]
pub(super) struct LoreCtorDesc {
    pub(super) ctor: SubTraceLoreCtor,
    pub(super) prev_lore: Option<ResolvedSubTraceDescs>,
    pub(super) current_lore: Option<ResolvedSubTraceDescs>,
}
