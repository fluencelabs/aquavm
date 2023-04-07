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

use super::ExecutedState;
use crate::state_automata::TraceLen;
use crate::TracePos;
use thiserror::Error as ThisError;

/// Errors arose out while accessing various interpreter data.
#[derive(ThisError, Debug, PartialEq, Eq)]
pub enum KeeperError {
    /// Errors occurred when trace_len - trace_position < requested_subtrace_len.
    #[error(
        "executed trace has {trace_len} elements and current position is {trace_position}, \
        but tried to set {requested_subtrace_len} subtrace_len"
    )]
    SetSubtraceLenFailed {
        requested_subtrace_len: TraceLen,
        trace_position: TracePos,
        trace_len: usize,
    },

    /// Errors occurred when
    ///     requested_subtrace_len != 0 && requested_pos + requested_subtrace_len > trace_len.
    #[error(
        "executed trace has {trace_len} elements, \
        but tried to set {requested_subtrace_len} subtrace_len and {requested_pos} position"
    )]
    SetSubtraceLenAndPosFailed {
        requested_pos: TracePos,
        requested_subtrace_len: TraceLen,
        trace_len: usize,
    },

    /// Errors occurred when Fold FSM tries to obtain stream generation by value_pos from a trace,
    /// but this value_pos is bigger than the trace length.
    #[error("requested an element at position '{position}', but executed trace contains only '{trace_len}' elements")]
    NoElementAtPosition { position: TracePos, trace_len: TraceLen },

    /// Errors occurred when Fold FSM tries to obtain stream generation by value_pos from a trace,
    /// but such state doesn't belong to values in streams (it doesn't contain a generation).
    #[error("expected a state of CallResult(Value::Stream) or Ap types but '{state}' obtained")]
    NoStreamState { state: ExecutedState },
}
