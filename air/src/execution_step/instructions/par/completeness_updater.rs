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
use super::SubgraphType;

#[derive(Debug, Default, Clone)]
pub(super) struct ParCompletenessUpdater {
    left_subgraph_complete: bool,
    right_subgraph_complete: bool,
}

impl ParCompletenessUpdater {
    pub(super) fn new() -> Self {
        Self {
            left_subgraph_complete: false,
            right_subgraph_complete: false,
        }
    }

    pub(super) fn observe_completeness(&mut self, exec_ctx: &ExecutionCtx<'_>, subgraph_type: SubgraphType) {
        match subgraph_type {
            SubgraphType::Left => self.left_subgraph_complete = exec_ctx.is_subgraph_complete(),
            SubgraphType::Right => self.right_subgraph_complete = exec_ctx.is_subgraph_complete(),
        }
    }

    pub(super) fn set_completeness(self, exec_ctx: &mut ExecutionCtx<'_>) {
        // par is completed if at least one of its subgraphs is completed
        let subgraph_complete = self.left_subgraph_complete || self.right_subgraph_complete;
        exec_ctx.set_subgraph_completeness(subgraph_complete);
    }
}
