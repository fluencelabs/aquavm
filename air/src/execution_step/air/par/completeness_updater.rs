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
            SubgraphType::Left => self.left_subgraph_complete = exec_ctx.subgraph_complete,
            SubgraphType::Right => self.right_subgraph_complete = exec_ctx.subgraph_complete,
        }
    }

    pub(super) fn set_completeness(self, exec_ctx: &mut ExecutionCtx<'_>) {
        // par is completed if at least one of its subgraphs is completed
        let subgraph_complete = self.left_subgraph_complete || self.right_subgraph_complete;
        exec_ctx.subgraph_complete = subgraph_complete;
    }
}
