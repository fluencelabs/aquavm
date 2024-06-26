/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::ExecutedState;
use crate::TracePos;
use air_interpreter_data::TraceLen;
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
        trace_len: TraceLen,
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
        trace_len: TraceLen,
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
