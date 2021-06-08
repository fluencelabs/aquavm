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

use super::*;

impl TraceMerger {
    pub(super) fn merge_folds(&mut self, prev_fold: &FoldResult, current_fold: &FoldResult) -> MergeResult<()> {
        let prev_tale = read_fold_tale(&self.prev_slider, prev_fold)?;
        let current_tale = read_fold_tale(&self.current_slider, current_fold)?;
        let subtree_size_updater = FoldSubtreeSizeUpdater::new(self, &prev_tale, &current_tale)?;

        merge_folds(self, prev_tale.fold_lore, current_tale.fold_lore)?;

        subtree_size_updater.update(self);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FoldTale {
    pub fold_lore: Vec<ResolvedFoldSubTraceLore>,
    pub states_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFoldSubTraceLore {
    value: Rc<JValue>,
    begin_pos: usize,
    interval_len: usize,
}

fn read_fold_tale(slider: &TraceSlider, fold: &FoldResult) -> MergeResult<FoldTale> {
    let mut fold_lore = Vec::with_capacity(fold.0.len());
    let mut states_count: usize = 0;

    for subtrace_lores in fold.0.iter() {
        for subtrace_lore in subtrace_lores.iter() {
            let value = slider.call_result_by_pos(subtrace_lore.value_pos)?.clone();
            let fold_value = ResolvedFoldSubTraceLore {
                value,
                begin_pos: subtrace_lore.begin_pos,
                interval_len: subtrace_lore.interval_len,
            };

            states_count = states_count
                .checked_add(subtrace_lore.interval_len)
                .ok_or_else(|| DataMergingError::FoldLenOverflow(fold.clone()))?;

            fold_lore.push(fold_value);
        }
    }

    let fold_tale = FoldTale {
        fold_lore,
        states_count,
    };

    Ok(fold_tale)
}

struct FoldSubtreeSizeUpdater {
    new_prev_size: usize,
    new_current_size: usize,
}

impl FoldSubtreeSizeUpdater {
    pub(crate) fn new(trace_merger: &TraceMerger, prev_fold: &FoldTale, current_fold: &FoldTale) -> MergeResult<Self> {
        let new_prev_size = Self::compute_new_subtree_size(&trace_merger.prev_slider, prev_fold)?;
        let new_current_size = Self::compute_new_subtree_size(&trace_merger.current_slider, current_fold)?;

        let updater = Self {
            new_prev_size,
            new_current_size,
        };
        Ok(updater)
    }

    pub(crate) fn update(&self, trace_merger: &mut TraceMerger) {
        trace_merger.prev_slider.set_interval(self.new_prev_size);
        trace_merger.current_slider.set_interval(self.new_current_size);
    }

    fn compute_new_subtree_size(slider: &TraceSlider, fold_tale: &FoldTale) -> MergeResult<usize> {
        let subtree_size = slider.subtree_size();
        let new_subtree_size = subtree_size
            .checked_sub(fold_tale.states_count)
            .ok_or_else(|| DataMergingError::FoldSubtreeUnderflow(fold_tale.clone(), subtree_size))?;

        Ok(new_subtree_size)
    }
}

fn merge_folds(
    trace_merger: &mut TraceMerger,
    prev_fold_lore: Vec<ResolvedFoldSubTraceLore>,
    mut current_fold_lore: Vec<ResolvedFoldSubTraceLore>,
) -> MergeResult<()> {
    for prev_value in prev_fold_lore.iter() {
        match remove_first(&mut current_fold_lore, prev_value) {
            Some(current_value) => {
                trace_merger.prev_slider.set_position(prev_value.begin_pos);
                trace_merger.prev_slider.set_interval(prev_value.interval_len);

                trace_merger.current_slider.set_position(current_value.begin_pos);
                trace_merger.current_slider.set_interval(current_value.interval_len);

                trace_merger.merge_subtree()?;
            }
            None => {
                trace_merger.prev_slider.set_position(prev_value.begin_pos);
                trace_merger.prev_slider.set_interval(prev_value.interval_len);

                trace_merger.current_slider.set_interval(0);

                trace_merger.merge_subtree()?;
            }
        }
    }

    Ok(())
}

fn remove_first(
    elems: &mut Vec<ResolvedFoldSubTraceLore>,
    elem: &ResolvedFoldSubTraceLore,
) -> Option<ResolvedFoldSubTraceLore> {
    let elem_pos = elems.iter().position(|e| e == elem)?;
    let result = elems.swap_remove(elem_pos);

    Some(result)
}
