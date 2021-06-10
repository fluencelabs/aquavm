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
use DataMergingError::IncompatibleCallResults;

impl TraceMerger {
    pub(super) fn merge_calls(
        &mut self,
        prev_call_result: &CallResult,
        current_call_result: &CallResult,
    ) -> MergeResult<()> {
        use CallResult::*;

        let call_result = match (prev_call_result, current_call_result) {
            (CallServiceFailed(..), CallServiceFailed(..)) => {
                check_for_equal(prev_call_result, current_call_result)?;
                current_call_result
            }
            (RequestSentBy(_), CallServiceFailed(..)) => current_call_result,
            (CallServiceFailed(..), RequestSentBy(_)) => prev_call_result,
            (RequestSentBy(_), RequestSentBy(_)) => {
                check_for_equal(prev_call_result, current_call_result)?;
                prev_call_result
            }
            (RequestSentBy(_), Executed(_)) => {
                self.update_correspondence(MergeCtxType::Current);
                current_call_result
            }
            (Executed(_), RequestSentBy(_)) => {
                self.update_correspondence(MergeCtxType::Previous);
                prev_call_result
            }
            (Executed(_), Executed(_)) => {
                self.update_correspondence(MergeCtxType::Current);
                self.update_correspondence(MergeCtxType::Previous);

                check_for_equal(prev_call_result, current_call_result)?;
                prev_call_result
            }
            (Executed(_), CallServiceFailed(..)) | (CallServiceFailed(..), Executed(_)) => {
                return Err(IncompatibleCallResults(
                    prev_call_result.clone(),
                    current_call_result.clone(),
                ))
            }
        };

        // clone is cheap here
        self.result_trace.push_back(ExecutedState::Call(call_result.clone()));

        Ok(())
    }

    pub(super) fn update_correspondence(&mut self, ctx_type: MergeCtxType) {
        let ctx = match ctx_type {
            MergeCtxType::Current => &mut self.current_ctx,
            MergeCtxType::Previous => &mut self.prev_ctx,
        };

        let old_pos = ctx.slider.position() - 1;
        let new_pos = self.result_trace.len();
        ctx.add_correspondence(old_pos, new_pos);
    }
}

fn check_for_equal(prev_result: &CallResult, current_result: &CallResult) -> MergeResult<()> {
    if prev_result != current_result {
        Err(IncompatibleCallResults(prev_result.clone(), current_result.clone()))
    } else {
        Ok(())
    }
}
