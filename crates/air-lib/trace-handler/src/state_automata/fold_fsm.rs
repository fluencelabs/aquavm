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
mod state_handler;

use std::thread::current;
use super::*;
use lore_applier::*;
use lore_ctor::*;
use lore_ctor_queue::*;
use state_handler::CtxStateHandler;

use air_interpreter_data::FoldLore;

/// This FSM manages fold and keeps internally queue of lore ctors.
/// State transitioning functions must work in the following way:
///     meet_fold_start.1 ->
///         meet_generation_start.N ->
///             meet_next.M ->
///             meet_prev.M ->
///         meet_generation_end.N ->
///     meet_fold_end.1
/// where .T means that this function should be called exactly T times.
#[derive(Debug, Default, Clone)]
pub(crate) struct FoldFSM {
    prev_fold: ResolvedFold,
    current_fold: ResolvedFold,
    state_inserter: StateInserter,
    ctor_queue: SubTraceLoreCtorQueue,
    result_lore: FoldLore,
    state_handler: CtxStateHandler,
}

impl FoldFSM {
    pub(crate) fn from_fold_start(fold_result: MergerFoldResult, data_keeper: &mut DataKeeper) -> FSMResult<Self> {
        let state_inserter = StateInserter::from_keeper(data_keeper);
        let state_handler =
            CtxStateHandler::prepare(&fold_result.prev_fold_lore, &fold_result.current_fold_lore, data_keeper)?;

        let fold_fsm = Self {
            prev_fold: fold_result.prev_fold_lore,
            current_fold: fold_result.current_fold_lore,
            state_inserter,
            state_handler,
            ..<_>::default()
        };

        Ok(fold_fsm)
    }

    pub(crate) fn meet_iteration_start(&mut self, value_pos: usize, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        println!("  meet_iteration_start {} {:?}", value_pos, data_keeper.new_to_old_pos);
        let (prev_pos, current_pos) = match data_keeper.new_to_old_pos.get(&value_pos) {
            Some(DataPositions { prev_pos, current_pos }) => (prev_pos, current_pos),
            None => return self.prepare(None, None, value_pos, data_keeper),
        };

        println!("  meet_iteration_start: {:?} {:?}", prev_pos, current_pos);

        let prev_lore = prev_pos.map(|pos| self.prev_fold.lore.remove(&pos)).flatten();
        let current_lore = current_pos.map(|pos| self.current_fold.lore.remove(&pos)).flatten();

        self.prepare(prev_lore, current_lore, value_pos, data_keeper)
    }

    fn prepare(
        &mut self,
        prev_lore: Option<ResolvedSubTraceDescs>,
        current_lore: Option<ResolvedSubTraceDescs>,
        value_pos: usize,
        data_keeper: &mut DataKeeper,
    ) -> FSMResult<()> {
        apply_fold_lore_before(data_keeper, &prev_lore, &current_lore)?;

        let ctor = SubTraceLoreCtor::from_before_start(value_pos, data_keeper);
        self.ctor_queue.add_element(ctor, prev_lore, current_lore);

        Ok(())
    }

    pub(crate) fn meet_iteration_end(&mut self, data_keeper: &mut DataKeeper) {
        self.ctor_queue.current().ctor.before_end(data_keeper);
    }

    pub(crate) fn meet_back_iterator(&mut self, data_keeper: &mut DataKeeper) -> FSMResult<()> {
        let back_traversal_started = self.ctor_queue.back_traversal_started();

        let LoreCtorDesc {
            ctor,
            prev_lore,
            current_lore,
        } = self.ctor_queue.current();

        if !back_traversal_started {
            ctor.maybe_before_end(data_keeper);
            ctor.after_start(data_keeper);
            apply_fold_lore_after(data_keeper, prev_lore, current_lore)?;
            self.ctor_queue.start_back_traverse();
        } else {
            ctor.after_end(data_keeper);
            self.ctor_queue.traverse_back();

            let LoreCtorDesc {
                ctor,
                prev_lore,
                current_lore,
            } = self.ctor_queue.current();

            ctor.after_start(data_keeper);
            apply_fold_lore_after(data_keeper, prev_lore, current_lore)?;
        }

        Ok(())
    }

    pub(crate) fn meet_generation_end(&mut self, data_keeper: &mut DataKeeper) {
        self.ctor_queue.finish(data_keeper);
        self.ctor_queue.end_back_traverse();

        let fold_lore = self.ctor_queue.transform_to_lore();
        self.result_lore.extend(fold_lore);
    }

    pub(crate) fn meet_fold_end(self, data_keeper: &mut DataKeeper) {
        // TODO: check for prev and current lore emptiness
        let fold_result = FoldResult { lore: self.result_lore };
        let state = ExecutedState::Fold(fold_result);
        self.state_inserter.insert(data_keeper, state);
        self.state_handler.set_final_states(data_keeper);
    }
}

#[derive(Clone, Copy)]
pub(self) enum ByNextPosition {
    /// Represents executed states before next.
    Before,

    /// Represents executed states after next.
    After,
}
