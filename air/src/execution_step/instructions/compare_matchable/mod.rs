/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::execution_step::resolver::Resolvable;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;

use air_parser::ast;

#[tracing::instrument(skip_all)]
pub(crate) fn are_matchable_eq<'ctx>(
    left: &ast::ImmutableValue<'_>,
    right: &ast::ImmutableValue<'_>,
    exec_ctx: &'ctx ExecutionCtx<'_>,
) -> ExecutionResult<bool> {
    let (left_value, _, _) = left.resolve(exec_ctx)?;
    let (right_value, _, _) = right.resolve(exec_ctx)?;

    Ok(left_value == right_value)
}
