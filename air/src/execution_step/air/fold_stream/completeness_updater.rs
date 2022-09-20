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
use super::ExecutionResult;

pub(super) struct FoldGenerationObserver {
    subtree_complete: bool,
    // keeps either Ok or the last met error
    result: ExecutionResult<()>,
}

impl FoldGenerationObserver {
    pub(super) fn new() -> Self {
        Self {
            subtree_complete: false,
            result: Ok(()),
        }
    }

    pub(super) fn observe_generation_results(&mut self, completeness: bool, result: ExecutionResult<()>) {
        self.subtree_complete |= completeness;
        if result.is_err() {
            self.result = result;
        }
    }

    pub(super) fn update_completeness(&self, exec_ctx: &mut ExecutionCtx<'_>) {
        exec_ctx.subgraph_complete = self.subtree_complete;
    }

    pub(super) fn into_result(self) -> ExecutionResult<()> {
        self.result
    }
}
