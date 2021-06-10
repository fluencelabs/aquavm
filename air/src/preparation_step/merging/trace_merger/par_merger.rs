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
    pub(super) fn merge_pars(&mut self, prev_par: ParResult, current_par: ParResult) -> MergeResult<()> {
        let subtree_size_updater = ParSubtreeSizeUpdater::new(self, prev_par, current_par)?;
        let par_state_adder = ParStateAdder::new(self);

        let merge_result = ParMerger::merge_pars(self, prev_par, current_par)?;

        subtree_size_updater.update(self);
        par_state_adder.add_state(self, merge_result);

        Ok(())
    }
}

struct ParSubtreeSizeUpdater {
    new_prev_size: usize,
    new_current_size: usize,
}

impl ParSubtreeSizeUpdater {
    pub(crate) fn new(trace_merger: &TraceMerger, prev_par: ParResult, current_par: ParResult) -> MergeResult<Self> {
        let new_prev_size = Self::compute_new_subtree_size(&trace_merger.prev_ctx.slider, prev_par)?;
        let new_current_size = Self::compute_new_subtree_size(&trace_merger.current_ctx.slider, current_par)?;

        let updater = Self {
            new_prev_size,
            new_current_size,
        };
        Ok(updater)
    }

    pub(crate) fn update(&self, trace_merger: &mut TraceMerger) {
        trace_merger.prev_ctx.slider.set_interval_len(self.new_prev_size);
        trace_merger.current_ctx.slider.set_interval_len(self.new_current_size);
    }

    fn compute_new_subtree_size(slider: &TraceSlider, par: ParResult) -> MergeResult<usize> {
        let subtree_size = slider.interval_len();
        let prev_par_entire_len = par.size().ok_or(DataMergingError::ParLenOverflow(par))?;
        let new_subtree_size = subtree_size
            .checked_sub(prev_par_entire_len)
            .ok_or(DataMergingError::ParSubtreeUnderflow(par, subtree_size))?;

        Ok(new_subtree_size)
    }
}

struct ParStateAdder {
    par_position: usize,
}

impl ParStateAdder {
    pub(crate) fn new(trace_merger: &mut TraceMerger) -> Self {
        let par_position = trace_merger.result_trace.len();
        // place a temporary Par value to avoid insertion in the middle
        trace_merger.result_trace.push_back(ExecutedState::Par(ParResult(0, 0)));

        Self { par_position }
    }

    pub(crate) fn add_state(&self, trace_merger: &mut TraceMerger, merge_result: ParMergeResult) {
        let left_par_size = merge_result.left_par_size;
        let right_par_size = merge_result.right_par_size;

        // update the temporary Par with final values
        trace_merger.result_trace[self.par_position] = ExecutedState::Par(ParResult(left_par_size, right_par_size));
    }
}

struct ParMergeResult {
    pub(crate) left_par_size: usize,
    pub(crate) right_par_size: usize,
}

struct ParMerger {
    initial_trace_len: usize,
    left_par_size: usize,
    right_par_size: usize,
    prev_par: ParResult,
    current_par: ParResult,
}

impl ParMerger {
    pub(crate) fn merge_pars(
        trace_merger: &mut TraceMerger,
        prev_par: ParResult,
        current_par: ParResult,
    ) -> MergeResult<ParMergeResult> {
        let initial_trace_len = trace_merger.result_trace.len();

        let mut merger = Self {
            initial_trace_len,
            left_par_size: 0,
            right_par_size: 0,
            prev_par,
            current_par,
        };

        merger.merge_left_subtree(trace_merger)?;
        merger.merge_right_subtree(trace_merger)?;

        let result = merger.into_merge_result();
        Ok(result)
    }

    fn merge_left_subtree(&mut self, trace_merger: &mut TraceMerger) -> MergeResult<()> {
        let prev_par_left_len = self.prev_par.0;
        let current_par_left_len = self.current_par.0;

        Self::merge_subtree(trace_merger, prev_par_left_len, current_par_left_len)?;
        self.left_par_size = trace_merger.result_trace.len() - self.initial_trace_len;

        Ok(())
    }

    fn merge_right_subtree(&mut self, trace_merger: &mut TraceMerger) -> MergeResult<()> {
        let prev_par_right_len = self.prev_par.1;
        let current_par_right_len = self.current_par.1;

        Self::merge_subtree(trace_merger, prev_par_right_len, current_par_right_len)?;
        self.right_par_size = trace_merger.result_trace.len() - self.initial_trace_len - self.left_par_size;

        Ok(())
    }

    fn merge_subtree(trace_merger: &mut TraceMerger, prev_interval: usize, current_interval: usize) -> MergeResult<()> {
        trace_merger.prev_ctx.slider.set_interval_len(prev_interval);
        trace_merger.current_ctx.slider.set_interval_len(current_interval);
        trace_merger.merge_subtree()
    }

    fn into_merge_result(self) -> ParMergeResult {
        ParMergeResult {
            left_par_size: self.left_par_size,
            right_par_size: self.right_par_size,
        }
    }
}
