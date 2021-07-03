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
use crate::log_targets::EXECUTED_STATE_CHANGING;
use merger::*;

use air_interpreter_data::InterpreterData;
use air_parser::ast::CallOutputValue;

#[derive(Debug, Default)]
pub(crate) struct TraceHandler {
    data_keeper: DataKeeper,
    state_fsm_queue: FSMQueue,
}

impl TraceHandler {
    pub(crate) fn from_data(prev_data: InterpreterData, current_data: InterpreterData) -> Self {
        let data_keeper = DataKeeper::from_data(prev_data, current_data);

        Self {
            data_keeper,
            state_fsm_queue: <_>::default(),
        }
    }

    /// Should be called at the beginning of a call execution.
    pub(crate) fn meet_call_start(
        &mut self,
        output_value: &CallOutputValue<'_>,
    ) -> TraceHandlerResult<MergerCallResult> {
        try_merge_next_state_as_call(&mut self.data_keeper, output_value).map_err(Into::into)
    }

    /// Should be called when a call instruction was executed successfully. It adds the supplied
    /// state to the result trace.
    pub(crate) fn meet_call_end(&mut self, call_result: CallResult) {
        log::trace!(
            target: EXECUTED_STATE_CHANGING,
            "  adding new call executed state {:?}",
            call_result
        );

        self.data_keeper.result_trace.push(ExecutedState::Call(call_result));
    }

    pub(crate) fn meet_par_start(&mut self) -> TraceHandlerResult<()> {
        let ingredients = merger::try_merge_next_state_as_par(&mut self.data_keeper)?;
        let par_fsm = ParFSM::new(ingredients, &mut self.data_keeper)?;
        self.state_fsm_queue.push_fsm(StateFSM::Par(par_fsm));

        Ok(())
    }

    pub(crate) fn meet_par_subtree_end(&mut self, subtree_type: SubtreeType) -> TraceHandlerResult<()> {
        match subtree_type {
            SubtreeType::Left => {
                let par_fsm = self.state_fsm_queue.last_as_mut_par()?;
                par_fsm.left_completed(&mut self.data_keeper)?;
            }
            SubtreeType::Right => {
                let par_fsm = self.state_fsm_queue.pop_as_par()?;
                par_fsm.right_completed(&mut self.data_keeper)?;
            }
        }

        Ok(())
    }

    pub(crate) fn meet_fold_start(&mut self) -> TraceHandlerResult<()> {
        let ingredients = try_merge_next_state_as_fold(&mut self.data_keeper)?;
        let fold_fsm = FoldFSM::from_fold_start(ingredients, &mut self.data_keeper)?;
        self.state_fsm_queue.push_fsm(StateFSM::Fold(fold_fsm));
        Ok(())
    }

    pub(crate) fn meet_generation_start(&mut self, value: &ValueAndPos) -> TraceHandlerResult<()> {
        let fold_fsm = self.state_fsm_queue.last_as_mut_fold()?;
        fold_fsm.meet_generation_start(value, &mut self.data_keeper)?;
        Ok(())
    }

    pub(crate) fn meet_next(&mut self, value: &ValueAndPos) -> TraceHandlerResult<()> {
        let fold_fsm = self.state_fsm_queue.last_as_mut_fold()?;
        fold_fsm.meet_next(value, &mut self.data_keeper)?;
        Ok(())
    }

    pub(crate) fn meet_prev(&mut self) -> TraceHandlerResult<()> {
        let fold_fsm = self.state_fsm_queue.last_as_mut_fold()?;
        fold_fsm.meet_prev(&mut self.data_keeper)?;
        Ok(())
    }

    pub(crate) fn meet_generation_end(&mut self) -> TraceHandlerResult<()> {
        let fold_fsm = self.state_fsm_queue.last_as_mut_fold()?;
        fold_fsm.meet_generation_end(&mut self.data_keeper);
        Ok(())
    }

    pub(crate) fn meet_fold_end(&mut self) -> TraceHandlerResult<()> {
        let fold_fsm = self.state_fsm_queue.pop_as_fold()?;
        fold_fsm.meet_fold_end(&mut self.data_keeper)?;

        Ok(())
    }

    pub(crate) fn error_exit(&mut self) {
        let state = match self.state_fsm_queue.pop() {
            Some(state) => state,
            None => return,
        };

        match state {
            StateFSM::Par(par) => par.error_exit(&mut self.data_keeper),
            StateFSM::Fold(fold) => fold.error_exit(&mut self.data_keeper),
        }
    }

    /// Returns size of elements inside result trace and intended to provide
    /// a position of next inserted elements.
    pub(crate) fn trace_pos(&self) -> usize {
        self.data_keeper.result_trace.len()
    }

    pub(crate) fn into_result_trace(self) -> ExecutionTrace {
        self.data_keeper.result_trace
    }

    pub(crate) fn as_result_trace(&self) -> &ExecutionTrace {
        &self.data_keeper.result_trace
    }

    pub(crate) fn subtree_sizes(&self) -> (usize, usize) {
        let prev_size = self.data_keeper.prev_ctx.slider.subtrace_len();
        let current_size = self.data_keeper.current_ctx.slider.subtrace_len();

        (prev_size, current_size)
    }
}
