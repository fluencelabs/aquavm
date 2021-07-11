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

use super::fold::*;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::log_instruction;

use air_parser::ast::FoldScalar;
use air_parser::ast::Instruction;
use std::rc::Rc;

impl<'i> ExecutableInstruction<'i> for FoldScalar<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        match construct_scalar_iterable_value(&self.iterable, exec_ctx)? {
            FoldIterableScalar::Empty => Ok(()),
            FoldIterableScalar::Scalar(iterable) => fold(
                iterable,
                IterableType::Scalar,
                self.iterator,
                self.instruction.clone(),
                exec_ctx,
                trace_ctx,
            ),
        }
    }
}

pub(super) fn fold<'i>(
    iterable: IterableValue,
    iterable_type: IterableType,
    iterator: &'i str,
    instruction: Rc<Instruction<'i>>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let fold_state = FoldState::from_iterable(iterable, iterable_type, instruction.clone());
    let variable_handler = VariableHandler::init(exec_ctx, iterator, fold_state)?;

    instruction.execute(exec_ctx, trace_ctx)?;

    variable_handler.cleanup(exec_ctx);

    Ok(())
}
