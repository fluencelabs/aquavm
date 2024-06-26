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

use super::ExecutionCtx;

pub(super) struct FoldGenerationObserver {
    subgraph_complete: bool,
}

impl FoldGenerationObserver {
    pub(super) fn new() -> Self {
        Self {
            subgraph_complete: false,
        }
    }

    pub(super) fn observe_completeness(&mut self, completeness: bool) {
        self.subgraph_complete |= completeness;
    }

    pub(super) fn update_completeness(self, exec_ctx: &mut ExecutionCtx<'_>) {
        exec_ctx.set_subgraph_completeness(self.subgraph_complete);
    }
}
