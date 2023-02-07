/*
 * Copyright 2022 Fluence Labs Limited
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
