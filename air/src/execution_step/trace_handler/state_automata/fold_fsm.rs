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

mod lore_applier;
mod lore_ctor;
mod lore_ctor_queue;
mod size_updater;

use super::*;
use crate::JValue;
use lore_applier::*;
use lore_ctor::*;
use lore_ctor_queue::*;
use size_updater::SubTreeSizeUpdater;

use air_interpreter_data::FoldLore;

use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub(crate) struct FoldFSM {
    prev_fold_lore: ResolvedFoldLore,
    current_fold_lore: ResolvedFoldLore,
    state_inserter: StateInserter,
    ctor_queue: LoreCtorQueue,
    result_tale: FoldLore,
    len_updater: SubTreeSizeUpdater,
}

pub(crate) struct ValueAndPos {
    pub(crate) value: Rc<JValue>,
    pub(crate) pos: usize,
}

impl FoldFSM {
    pub(crate) fn from_fold_start(fold_result: MergerFoldResult, data_keeper: &mut DataKeeper) -> FSMResult<Self> {
        let state_inserter = StateInserter::from_keeper(data_keeper);
        let len_updater = SubTreeSizeUpdater::new(data_keeper);

        let fold_fsm = Self {
            prev_fold_lore: fold_result.prev_fold_lore,
            current_fold_lore: fold_result.current_fold_lore,
            state_inserter,
            len_updater,
            ..<_>::default()
        };

        Ok(fold_fsm)
    }

    pub(crate) fn meet_generation_start(&mut self, value: ValueAndPos, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.meet_before_state(value, data_keeper)
    }

    pub(crate) fn meet_next(&mut self, value: ValueAndPos, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        self.ctor_queue.forward_ctor_mut().lore_ctor.before_end(data_keeper);
        self.meet_before_state(value, data_keeper)
    }

    pub(crate) fn meet_prev(&mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        let lore_elem = if self.ctor_queue.were_no_back_traversals() {
            let lore_elem = self.ctor_queue.forward_ctor_mut();
            lore_elem.lore_ctor.before_end(data_keeper);
            lore_elem.lore_ctor.after_start(data_keeper);
            lore_elem
        } else {
            self.ctor_queue.backward_ctor_mut().lore_ctor.after_end(data_keeper);

            self.ctor_queue.backward_traverse();
            let lore_elem = self.ctor_queue.backward_ctor_mut();
            lore_elem.lore_ctor.after_start(data_keeper);

            lore_elem
        };

        apply_fold_lore_after(data_keeper, &lore_elem.prev_lore, &lore_elem.current_lore)
    }

    pub(crate) fn meet_generation_end(&mut self, data_keeper: &mut DataKeeper) {
        self.ctor_queue.backward_ctor_mut().lore_ctor.after_end(data_keeper);

        let subtale = self.ctor_queue.transform_to_lore();
        self.result_tale.extend(subtale);
    }

    pub(crate) fn meet_fold_end(self, data_keeper: &mut DataKeeper) {
        // TODO: check for prev and current tale emptiness
        let fold_result = FoldResult(self.result_tale);
        let state = ExecutedState::Fold(fold_result);
        self.state_inserter.insert(data_keeper, state);
    }

    fn meet_before_state(&mut self, value: ValueAndPos, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        let prev_lore = remove_first(&mut self.prev_fold_lore, &value.value);
        let current_lore = remove_first(&mut self.current_fold_lore, &value.value);
        apply_fold_lore_before(data_keeper, &prev_lore, &current_lore)?;

        let lore_ctor = FoldLoreCtor::from_before_start(value.pos, data_keeper);
        let queue_elem = LoreCtorElement {
            prev_lore,
            current_lore,
            lore_ctor,
        };
        self.ctor_queue.add_element(queue_elem);

        Ok(())
    }
}

fn remove_first(elems: &mut Vec<ResolvedFoldSubTraceLore>, elem: &Rc<JValue>) -> Option<ResolvedFoldSubTraceLore> {
    let elem_pos = elems.iter().position(|e| &e.value == elem)?;
    let result = elems.swap_remove(elem_pos);

    Some(result)
}
