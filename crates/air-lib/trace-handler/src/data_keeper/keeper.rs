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

use super::ExecutionTrace;
use super::MergeCtx;
use super::TraceSlider;
use crate::TracePos;

use bimap::BiHashMap;

/// Keeps all necessary data for merging.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct DataKeeper {
    pub(crate) prev_ctx: MergeCtx,
    pub(crate) current_ctx: MergeCtx,
    pub(crate) new_to_prev_pos: BiHashMap<TracePos, TracePos>,
    pub(crate) new_to_current_pos: BiHashMap<TracePos, TracePos>,
    pub(crate) result_trace: ExecutionTrace,
}

impl DataKeeper {
    pub(crate) fn from_trace(prev_trace: ExecutionTrace, current_trace: ExecutionTrace) -> Self {
        let prev_ctx = MergeCtx::from_trace(prev_trace);
        let current_ctx = MergeCtx::from_trace(current_trace);

        Self {
            prev_ctx,
            current_ctx,
            new_to_prev_pos: <_>::default(),
            new_to_current_pos: <_>::default(),
            result_trace: <_>::default(),
        }
    }

    pub(crate) fn result_states_count(&self) -> usize {
        self.result_trace.len()
    }

    pub(crate) fn result_trace_next_pos(&self) -> TracePos {
        self.result_trace.trace_states_count().into()
    }

    pub(crate) fn prev_slider(&self) -> &TraceSlider {
        &self.prev_ctx.slider
    }

    pub(crate) fn prev_slider_mut(&mut self) -> &mut TraceSlider {
        &mut self.prev_ctx.slider
    }

    pub(crate) fn current_slider(&self) -> &TraceSlider {
        &self.current_ctx.slider
    }

    pub(crate) fn current_slider_mut(&mut self) -> &mut TraceSlider {
        &mut self.current_ctx.slider
    }
}
