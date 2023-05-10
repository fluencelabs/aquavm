/*
 * Copyright 2023 Fluence Labs Limited
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

// mod apply_to_arguments;
// mod utils;

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
// use crate::execution_step::execution_context::errors::StreamMapError::FloatMapKeyIsUnsupported;
// use crate::execution_step::execution_context::errors::StreamMapError::MapKeyIsAbsent;
// use crate::execution_step::execution_context::errors::StreamMapError::UnsupportedMapKeyType;
// use crate::execution_step::instructions::ap::apply_to_arguments::apply_to_arg;
use crate::execution_step::instructions::ap::execute_ap_kind;

// use crate::log_instruction;

use air_parser::ast::ApMap;

impl<'i> super::ExecutableInstruction<'i> for ApMap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        execute_ap_kind(
            &self,
            Some(&self.key),
            &self.value,
            &self.map,
            exec_ctx,
            trace_ctx,
        )
    }
}
