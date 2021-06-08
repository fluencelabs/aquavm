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

mod call_merger;
mod fold_merger;
mod par_merger;

pub use fold_merger::FoldTale;

use super::CallResult;
use super::DataMergingError;
use super::ExecutedState;
use super::ExecutionTrace;
use super::FoldResult;
use super::MergeResult;
use super::ParResult;
use super::TraceSlider;
use crate::JValue;

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct TraceMerger {
    prev_slider: TraceSlider,
    current_slider: TraceSlider,
    result_trace: ExecutionTrace,
}

impl TraceMerger {
    pub(crate) fn new(prev_trace: ExecutionTrace, current_trace: ExecutionTrace) -> Self {
        let max_trace_len = std::cmp::max(prev_trace.len(), current_trace.len());
        let result_trace = ExecutionTrace::with_capacity(max_trace_len);

        let prev_slider = TraceSlider::new(prev_trace);
        let current_slider = TraceSlider::new(current_trace);

        Self {
            prev_slider,
            current_slider,
            result_trace,
        }
    }

    pub(crate) fn merge(mut self) -> MergeResult<ExecutionTrace> {
        self.merge_subtree()?;

        log::trace!(
            target: crate::log_targets::EXECUTED_TRACE_MERGE,
            "merged trace: {:?}",
            self.result_trace
        );

        Ok(self.result_trace)
    }

    fn merge_subtree(&mut self) -> MergeResult<()> {
        use DataMergingError::IncompatibleExecutedStates;
        use ExecutedState::*;

        loop {
            let prev_state = self.prev_slider.next_state();
            let current_state = self.current_slider.next_state();

            match (&prev_state, &current_state) {
                (Some(Call(prev_call)), Some(Call(current_call))) => self.merge_calls(prev_call, current_call)?,
                (Some(Par(prev_par)), Some(Par(current_par))) => self.merge_pars(*prev_par, *current_par)?,
                (Some(Fold(prev_fold)), Some(Fold(current_fold))) => self.merge_folds(prev_fold, current_fold)?,
                (None, Some(state)) => self.merge_tail(state.clone(), SliderType::Current)?,
                (Some(state), None) => self.merge_tail(state.clone(), SliderType::Previous)?,
                (None, None) => break,

                // this match arm represents incompatible (Call, Par), (Par, Call), (Fold, Call) ... states
                (Some(prev_state), Some(current_state)) => {
                    return Err(IncompatibleExecutedStates(prev_state.clone(), current_state.clone()))
                }
            }
        }

        Ok(())
    }

    fn merge_tail(&mut self, state: ExecutedState, slider_type: SliderType) -> MergeResult<()> {
        self.result_trace.push_back(state);

        let slider = match slider_type {
            SliderType::Current => &mut self.current_slider,
            SliderType::Previous => &mut self.prev_slider,
        };

        let current_states = slider.remaining_interval()?;
        self.result_trace.extend(current_states);

        Ok(())
    }
}

enum SliderType {
    Current,
    Previous,
}
