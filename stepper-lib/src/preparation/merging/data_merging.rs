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

use crate::log_targets::EXECUTED_TRACE_MERGE;
use crate::preparation::CallResult;
use crate::preparation::DataMergingError;
use crate::preparation::ExecutedState;
use crate::preparation::ExecutionTrace;
use crate::preparation::ParResult;
use crate::JValue;

use air_parser::ast::Instruction;

use std::collections::HashMap;

type MergeResult<T> = Result<T, DataMergingError>;

pub(crate) fn merge_execution_traces<'i>(
    prev_trace: ExecutionTrace,
    current_trace: ExecutionTrace,
    aqua: &Instruction<'i>,
) -> MergeResult<ExecutionTrace> {
    let mut merged_trace = ExecutionTrace::new();

    let mut prev_ctx = MergeCtx::new(prev_trace);
    let mut current_ctx = MergeCtx::new(current_trace);

    merge_subtree(&mut prev_ctx, &mut current_ctx, aqua, &mut merged_trace)?;

    log::trace!(target: EXECUTED_TRACE_MERGE, "merged trace: {:?}", merged_trace);

    Ok(merged_trace)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MergeCtx<'i> {
    trace: ExecutionTrace,
    subtree_size: usize,
    streams: HashMap<String, Vec<&'i JValue>>,
}

impl MergeCtx<'_> {
    pub(crate) fn new(trace: ExecutionTrace) -> Self {
        let subtree_size = trace.len();

        Self {
            trace,
            subtree_size,
            streams: HashMap::new(),
        }
    }

    pub(crate) fn next_subtree_state(&mut self) -> Option<ExecutedState> {
        if self.subtree_size != 0 {
            self.subtree_size -= 1;
            self.trace.pop_front()
        } else {
            None
        }
    }

    pub(crate) fn set_subtree_size(&mut self, new_subtree_size: usize) {
        self.subtree_size = new_subtree_size;
    }

    pub(crate) fn drain_subtree_states(&mut self) -> MergeResult<impl Iterator<Item = ExecutedState> + '_> {
        use DataMergingError::ExecutedTraceTooSmall;

        if self.trace.len() < self.subtree_size {
            return Err(ExecutedTraceTooSmall(self.trace.len(), self.subtree_size));
        }

        Ok(self.trace.drain(..self.subtree_size))
    }

    pub(crate) fn subtree_size(&self) -> usize {
        self.subtree_size
    }
}

fn merge_subtree<'i>(
    prev_merge_ctx: &mut MergeCtx<'i>,
    current_merge_ctx: &mut MergeCtx<'i>,
    aqua: &Instruction<'i>,
    result_trace: &mut ExecutionTrace,
) -> MergeResult<()> {
    use DataMergingError::IncompatibleExecutedStates;
    use ExecutedState::*;

    loop {
        let prev_state = prev_merge_ctx.next_subtree_state();
        let current_state = current_merge_ctx.next_subtree_state();

        match (prev_state, current_state) {
            (Some(Call(prev_call)), Some(Call(call))) => {
                let resulted_call = merge_call(prev_call, call)?;
                result_trace.push_back(Call(resulted_call));
            }
            (Some(Par(prev_par)), Some(Par(current_par))) => merge_par(
                prev_par,
                current_par,
                prev_merge_ctx,
                current_merge_ctx,
                aqua,
                result_trace,
            )?,
            (None, Some(s)) => {
                result_trace.push_back(s);

                let current_states = current_merge_ctx.drain_subtree_states()?;
                result_trace.extend(current_states);
                break;
            }
            (Some(s), None) => {
                result_trace.push_back(s);

                let prev_states = prev_merge_ctx.drain_subtree_states()?;
                result_trace.extend(prev_states);
                break;
            }
            (None, None) => break,
            // this match arm represents (Call, Par) and (Par, Call) states
            (Some(prev_state), Some(current_state)) => {
                return Err(IncompatibleExecutedStates(prev_state, current_state))
            }
        }
    }

    Ok(())
}

fn merge_call(prev_call_result: CallResult, current_call_result: CallResult) -> MergeResult<CallResult> {
    use crate::preparation::CallResult::*;
    use DataMergingError::IncompatibleCallResults;

    match (&prev_call_result, &current_call_result) {
        (CallServiceFailed(prev_err_msg), CallServiceFailed(err_msg)) => {
            if prev_err_msg != err_msg {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }
            Ok(current_call_result)
        }
        (RequestSentBy(_), CallServiceFailed(_)) => Ok(current_call_result),
        (CallServiceFailed(_), RequestSentBy(_)) => Ok(prev_call_result),
        (RequestSentBy(prev_sender), RequestSentBy(sender)) => {
            if prev_sender != sender {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }

            Ok(prev_call_result)
        }
        (RequestSentBy(_), Executed(..)) => Ok(current_call_result),
        (Executed(..), RequestSentBy(_)) => Ok(prev_call_result),
        (Executed(prev_result), Executed(result)) => {
            if prev_result != result {
                return Err(IncompatibleCallResults(prev_call_result, current_call_result));
            }

            Ok(prev_call_result)
        }
        (CallServiceFailed(_), Executed(..)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
        (Executed(..), CallServiceFailed(_)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
    }
}

fn merge_par<'i>(
    prev_par: ParResult,
    current_par: ParResult,
    prev_merge_ctx: &mut MergeCtx<'i>,
    current_merge_ctx: &mut MergeCtx<'i>,
    aqua: &Instruction<'i>,
    result_trace: &mut ExecutionTrace,
) -> MergeResult<()> {
    let prev_subtree_size = prev_merge_ctx.subtree_size();
    let current_subtree_size = current_merge_ctx.subtree_size();

    let par_position = result_trace.len();
    // place a temporary Par value to avoid insertion in the middle
    result_trace.push_back(ExecutedState::Par(ParResult(0, 0)));

    let len_before_merge = result_trace.len();

    prev_merge_ctx.set_subtree_size(prev_par.0);
    current_merge_ctx.set_subtree_size(current_par.0);
    merge_subtree(prev_merge_ctx, current_merge_ctx, aqua, result_trace)?;

    let left_par_size = result_trace.len() - len_before_merge;

    prev_merge_ctx.set_subtree_size(prev_par.1);
    current_merge_ctx.set_subtree_size(current_par.1);
    merge_subtree(prev_merge_ctx, current_merge_ctx, aqua, result_trace)?;

    let right_par_size = result_trace.len() - left_par_size - len_before_merge;

    // update the temporary Par with final values
    result_trace[par_position] = ExecutedState::Par(ParResult(left_par_size, right_par_size));

    prev_merge_ctx.set_subtree_size(prev_subtree_size - prev_par.0 - prev_par.1);
    current_merge_ctx.set_subtree_size(current_subtree_size - current_par.0 - current_par.1);

    Ok(())
}
