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

use super::*;
use crate::contexts::execution_trace::FoldSubTraceLore;
use crate::JValue;

use MergeError::IncompatibleExecutedStates;

use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub(crate) struct MergerFoldResult {
    pub(crate) prev_fold_tale: FoldTale,
    pub(crate) current_fold_tale: FoldTale,
}

pub(crate) fn try_merge_next_state_as_fold(data_keeper: &mut DataKeeper) -> MergeResult<MergerFoldResult> {
    use ExecutedState::Fold;

    let prev_state = data_keeper.prev_ctx.slider.next_state();
    let current_state = data_keeper.current_ctx.slider.next_state();

    let fold_result = match (prev_state, current_state) {
        (Some(Fold(prev_fold)), Some(Fold(current_fold))) => {
            MergerFoldResult::from_fold_results(&prev_fold, &current_fold, data_keeper)
        }
        (None, Some(Fold(current_fold @ _))) => {
            MergerFoldResult::from_fold_result(&current_fold, data_keeper, MergeCtxType::Current)
        }
        (Some(Fold(prev_fold @ _)), None) => {
            MergerFoldResult::from_fold_result(&prev_fold, data_keeper, MergeCtxType::Previous)
        }
        (None, None) => MergerFoldResult::default(),
        (Some(prev_state), Some(current_state)) => return Err(IncompatibleExecutedStates(prev_state, current_state)),
    }?;

    Ok(fold_result)
}

impl MergerFoldResult {
    pub(self) fn from_fold_result(
        fold: &FoldResult,
        data_keeper: &DataKeeper,
        ctx_type: MergeCtxType,
    ) -> MergeResult<Self> {
        let (prev_fold_tale, current_fold_tale) = match ctx_type {
            MergeCtxType::Previous => {
                let fold_tale = read_fold_tale(&data_keeper.prev_ctx.slider, fold)?;
                (fold_tale, vec![])
            }
            MergeCtxType::Current => {
                let fold_tale = read_fold_tale(data_keeper.current_ctx.slide, fold)?;
                (vec![], fold_tale)
            }
        };

        let merge_result = Self {
            prev_fold_tale,
            current_fold_tale,
        };

        Ok(merge_result)
    }

    pub(self) fn from_fold_results(
        prev_fold: &FoldResult,
        current_fold: &FoldResult,
        data_keeper: &DataKeeper,
    ) -> MergeResult<Self> {
        let prev_fold_tale = read_fold_tale(&data_keeper.prev_ctx.slider, prev_fold)?;
        let current_fold_tale = read_fold_tale(&data_keeper.current_ctx.slider, current_fold)?;

        let merge_result = Self {
            prev_fold_tale,
            current_fold_tale,
        };

        Ok(merge_result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) type FoldTale = Vec<ResolvedFoldSubTraceLore>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFoldSubTraceLore {
    pub(crate) value: Rc<JValue>,
    pub(crate) begin_pos: FoldSubtraceInfo,
    pub(crate) interval_len: FoldSubtraceInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FoldSubtraceInfo {
    pub(crate) before: usize,
    pub(crate) after: usize,
}

impl FoldSubtraceInfo {
    pub(crate) fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }
}

fn read_fold_tale(slider: &TraceSlider, fold: &FoldResult) -> MergeResult<FoldTale> {
    let mut fold_lore = Vec::with_capacity(fold.0.len());

    for subtrace_lore in fold.0.iter() {
        check_subtrace_lore(subtrace_lore)?;

        let value = call_value_by_pos(slider, subtrace_lore[0].value_pos)?;
        let begin_pos = FoldSubtraceInfo::new(subtrace_lore[0].begin_pos, subtrace_lore[1].begin_pos);
        let interval_len = FoldSubtraceInfo::new(subtrace_lore[0].interval_len, subtrace_lore[1].interval_len);
        let fold_value = ResolvedFoldSubTraceLore {
            value,
            begin_pos,
            interval_len,
        };

        fold_lore.push(fold_value);
    }

    Ok(fold_tale)
}

fn check_subtrace_lore(subtrace_lores: &[FoldSubTraceLore]) -> MergeResult<()> {
    // this limitation is due to current constraint on count of next inside one fold
    const SUBTRACE_LORE_SIZE: usize = 2;

    if subtrace_lores.len() != SUBTRACE_LORE_SIZE {
        return Err(DataMergingError::FoldIncorrectSubtracesCount(subtrace_lores.len()));
    }

    // before and after here in terms of handling next by the interpreter
    let before_value_pos = subtrace_lores[0].value_pos;
    let after_value_pos = subtrace_lores[1].value_pos;
    if before_value_pos != after_value_pos {
        return Err(MergeError::FoldIncorrectValuePos(before_value_pos, after_value_pos));
    }

    Ok(())
}

fn call_value_by_pos(slider: &TraceSlider, pos: usize) -> MergeResult<Rc<JValue>> {
    let state = slider.state_by_pos(pos)?;
    match state {
        ExecutedState::Call(CallResult::Executed(value)) => Ok(value.clone()),
        _ => Err(MergeError::FoldPointsToNonCallResult(state.clone())),
    }
}

/*
impl TraceMerger {
    pub(super) fn merge_folds(&mut self, prev_fold: &FoldResult, current_fold: &FoldResult) -> MergeResult<()> {
        // read the whole states from current fold result
        let FoldTale {
            fold_lore,
            states_count: current_states_count,
        } = read_fold_tale(&self.current_ctx.slider, current_fold)?;
        let subtree_size_updater = FoldSliderUpdater::new(self);
        let fold_state_adder = FoldStateAdder::new(self);

        let (new_fold_result, prev_states_count) = merge_folds(self, prev_fold, fold_lore)?;

        subtree_size_updater.update(self, prev_fold, current_fold, prev_states_count, current_states_count)?;
        fold_state_adder.add(self, new_fold_result);

        Ok(())
    }
}

struct FoldSliderUpdater {
    prev_pos: usize,
    prev_len: usize,
    current_pos: usize,
    current_len: usize,
}

impl FoldSliderUpdater {
    pub(crate) fn new(trace_merger: &TraceMerger) -> Self {
        let prev_pos = trace_merger.prev_ctx.slider.position();
        let prev_len = trace_merger.prev_ctx.slider.interval_len();
        let current_pos = trace_merger.current_ctx.slider.position();
        let current_len = trace_merger.current_ctx.slider.interval_len();

        Self {
            prev_pos,
            prev_len,
            current_pos,
            current_len,
        }
    }

    pub(crate) fn update(
        &self,
        trace_merger: &mut TraceMerger,
        prev_fold: &FoldResult,
        current_fold: &FoldResult,
        prev_seen_states: usize,
        current_seen_states: usize,
    ) -> MergeResult<()> {
        let new_prev_pos = self.prev_pos + prev_seen_states;
        let new_prev_len = self
            .prev_len
            .checked_sub(prev_seen_states)
            .ok_or_else(|| DataMergingError::FoldSubtreeUnderflow(prev_fold.clone(), self.prev_len))?;

        let new_current_pos = self.current_pos + current_seen_states;
        let new_current_len = self
            .current_len
            .checked_sub(current_seen_states)
            .ok_or_else(|| DataMergingError::FoldSubtreeUnderflow(current_fold.clone(), self.prev_len))?;

        trace_merger.prev_ctx.slider.set_position(new_prev_pos);
        trace_merger.prev_ctx.slider.set_interval_len(new_prev_len);
        trace_merger.current_ctx.slider.set_position(new_current_pos);
        trace_merger.current_ctx.slider.set_interval_len(new_current_len);

        Ok(())
    }
}

fn merge_folds(
    trace_merger: &mut TraceMerger,
    prev_fold: &FoldResult,
    mut current_fold_lore: Vec<ResolvedFoldSubTraceLore>,
) -> MergeResult<(FoldResult, usize)> {
    let mut second_traversal = Vec::with_capacity(current_fold_lore.len());
    let mut prev_fold = prev_fold.clone();

    let mut prev_fold_states = 0;

    for subtrace_lore_level in prev_fold.0.iter_mut() {
        for subtrace_lore in subtrace_lore_level.iter_mut() {
            check_subtrace_lore(subtrace_lore)?;

            let prev_lore = &mut subtrace_lore[0];
            let value = trace_merger.prev_ctx.slider.call_result_by_pos(prev_lore.value_pos)?;

            match remove_first(&mut current_fold_lore, value) {
                Some(current_lore) => {
                    prev_fold_states += prev_lore.interval_len;
                    trace_merger.prev_ctx.slider.set_position(prev_lore.begin_pos);
                    trace_merger.prev_ctx.slider.set_interval_len(prev_lore.interval_len);

                    trace_merger.current_ctx.slider.set_position(current_lore.begin_pos[0]);
                    trace_merger
                        .current_ctx
                        .slider
                        .set_interval_len(current_lore.interval_len[0]);
                    let begin_pos = trace_merger.result_trace.len();

                    trace_merger.merge_subtree()?;

                    let interval_len = trace_merger.result_trace.len() - begin_pos;

                    prev_lore.value_pos = trace_merger.prev_ctx.try_get_new_pos(prev_lore.value_pos)?;
                    prev_lore.begin_pos = begin_pos;
                    prev_lore.interval_len = interval_len;

                    second_traversal.push(Some(current_lore));
                }
                None => {
                    prev_fold_states += prev_lore.interval_len;

                    trace_merger.prev_ctx.slider.set_position(prev_lore.begin_pos);
                    trace_merger.prev_ctx.slider.set_interval_len(prev_lore.interval_len);
                    let begin_pos = trace_merger.result_trace.len();

                    trace_merger.current_ctx.slider.set_interval_len(0);

                    trace_merger.merge_subtree()?;

                    let interval_len = trace_merger.result_trace.len() - begin_pos;
                    prev_lore.value_pos = trace_merger.prev_ctx.try_get_new_pos(prev_lore.value_pos)?;
                    prev_lore.begin_pos = begin_pos;
                    prev_lore.interval_len = interval_len;

                    second_traversal.push(None);
                }
            }
        }

        println!("prev: {:?}, current: {:?}", subtrace_lore_level, second_traversal);
        for (subtrace_lore, current_state) in subtrace_lore_level.iter_mut().zip(&second_traversal).rev() {
            println!("prev {:?}, current {:?}", subtrace_lore, current_state);
            let prev_lore = &mut subtrace_lore[1];

            match current_state {
                Some(current_lore) => {
                    prev_fold_states += prev_lore.interval_len;

                    trace_merger.prev_ctx.slider.set_position(prev_lore.begin_pos);
                    trace_merger.prev_ctx.slider.set_interval_len(prev_lore.interval_len);

                    trace_merger.current_ctx.slider.set_position(current_lore.begin_pos[1]);
                    trace_merger
                        .current_ctx
                        .slider
                        .set_interval_len(current_lore.interval_len[1]);
                    let begin_pos = trace_merger.result_trace.len();

                    trace_merger.merge_subtree()?;

                    let interval_len = trace_merger.result_trace.len() - begin_pos;
                    prev_lore.value_pos = trace_merger.prev_ctx.try_get_new_pos(prev_lore.value_pos)?;
                    prev_lore.begin_pos = begin_pos;
                    prev_lore.interval_len = interval_len;
                }
                None => {
                    prev_fold_states += prev_lore.interval_len;

                    trace_merger.prev_ctx.slider.set_position(prev_lore.begin_pos);
                    trace_merger.prev_ctx.slider.set_interval_len(prev_lore.interval_len);

                    trace_merger.current_ctx.slider.set_interval_len(0);

                    let begin_pos = trace_merger.result_trace.len();

                    trace_merger.merge_subtree()?;

                    let interval_len = trace_merger.result_trace.len() - begin_pos;
                    prev_lore.value_pos = trace_merger.prev_ctx.try_get_new_pos(prev_lore.value_pos)?;
                    prev_lore.begin_pos = begin_pos;
                    prev_lore.interval_len = interval_len;
                }
            }
        }
    }

    if current_fold_lore.is_empty() {
        return Ok((prev_fold.clone(), prev_fold_states));
    }

    let mut lores = Vec::with_capacity(current_fold_lore.len());
    // merge those values that aren't presence in prev_ctx
    for current_lore in current_fold_lore.iter() {
        trace_merger.current_ctx.slider.set_position(current_lore.begin_pos[0]);
        trace_merger
            .current_ctx
            .slider
            .set_interval_len(current_lore.interval_len[0]);

        trace_merger.prev_ctx.slider.set_interval_len(0);

        let begin_pos = trace_merger.result_trace.len();
        trace_merger.merge_subtree()?;
        let interval_len = trace_merger.result_trace.len() - begin_pos;

        let value_pos = trace_merger.current_ctx.try_get_new_pos(current_lore.value_pos)?;
        let new_lore = FoldSubTraceLore {
            value_pos,
            begin_pos,
            interval_len,
        };

        lores.push(vec![new_lore]);
    }

    for (lore_id, current_lore) in current_fold_lore.into_iter().enumerate() {
        trace_merger.current_ctx.slider.set_position(current_lore.begin_pos[1]);
        trace_merger
            .current_ctx
            .slider
            .set_interval_len(current_lore.interval_len[1]);

        trace_merger.prev_ctx.slider.set_interval_len(0);

        let begin_pos = trace_merger.result_trace.len();
        trace_merger.merge_subtree()?;
        let interval_len = trace_merger.result_trace.len() - begin_pos;

        let value_pos = trace_merger.current_ctx.try_get_new_pos(current_lore.value_pos)?;
        let new_lore = FoldSubTraceLore {
            value_pos,
            begin_pos,
            interval_len,
        };

        lores[lore_id].push(new_lore);
    }

    prev_fold.0.push(lores);

    Ok((prev_fold.clone(), prev_fold_states))
}

fn remove_first(elems: &mut Vec<ResolvedFoldSubTraceLore>, elem: &Rc<JValue>) -> Option<ResolvedFoldSubTraceLore> {
    let elem_pos = elems.iter().position(|e| &e.value == elem)?;
    let result = elems.swap_remove(elem_pos);

    Some(result)
}

struct FoldStateAdder {
    fold_pos: usize,
}

impl FoldStateAdder {
    pub(self) fn new(trace_merger: &mut TraceMerger) -> Self {
        let fold_pos = trace_merger.result_trace.len();
        let fold_state_placeholder = FoldResult::default();

        trace_merger
            .result_trace
            .push_back(ExecutedState::Fold(fold_state_placeholder));

        Self { fold_pos }
    }

    pub(self) fn add(&self, trace_merger: &mut TraceMerger, fold_result: FoldResult) {
        // update the temporary Fold with final result
        trace_merger.result_trace[self.fold_pos] = ExecutedState::Fold(fold_result);
    }
}

 */
