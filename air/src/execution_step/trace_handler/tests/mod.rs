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

mod call_merging;
mod fold_merging;
mod par_merging;
mod slider_tests;

use super::merge_execution_traces;
use super::DataMergingError;
use super::ExecutionTrace;
use super::TraceSlider;
use crate::preparation_step::merging::MergeResult;

use super::CallResult;
use super::ExecutedState;
use super::FoldResult;
use super::ParResult;
use crate::contexts::execution_trace::FoldSubTraceLore;
use crate::JValue;

use std::rc::Rc;

pub fn scalar_jvalue(result: JValue) -> ExecutedState {
    ExecutedState::Call(CallResult::Executed(Rc::new(result)))
}

pub fn request_sent_by(sender: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::RequestSentBy(Rc::new(sender.into())))
}

pub fn par(left: usize, right: usize) -> ExecutedState {
    ExecutedState::Par(ParResult(left, right))
}

pub fn service_failed(ret_code: i32, error_message: impl Into<String>) -> ExecutedState {
    ExecutedState::Call(CallResult::CallServiceFailed(ret_code, Rc::new(error_message.into())))
}

pub fn fold(lores: Vec<Vec<Vec<FoldSubTraceLore>>>) -> ExecutedState {
    ExecutedState::Fold(FoldResult(lores))
}
