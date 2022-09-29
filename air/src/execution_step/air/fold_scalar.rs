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
use crate::execution_step::Joinable;
use crate::joinable;
use crate::log_instruction;

use air_parser::ast::FoldScalar;
use air_parser::ast::FoldScalarIterable;
use air_parser::ast::Instruction;

use std::rc::Rc;

impl<'i> ExecutableInstruction<'i> for FoldScalar<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(fold, exec_ctx, trace_ctx);

        let iterable = match &self.iterable {
            FoldScalarIterable::Scalar(scalar) => joinable!(create_scalar_iterable(scalar, exec_ctx), exec_ctx, ())?,
            FoldScalarIterable::ScalarWithLambda(scalar) => {
                joinable!(create_scalar_wl_iterable(scalar, exec_ctx), exec_ctx, ())?
            }
            FoldScalarIterable::CanonStream(canon_stream) => {
                joinable!(create_canon_stream_iterable_value(canon_stream, exec_ctx), exec_ctx, ())?
            }
            // just do nothing on an empty array
            FoldScalarIterable::EmptyArray => return Ok(()),
        };

        match iterable {
            // just exit on empty iterable
            FoldIterableScalar::Empty => Ok(()),
            FoldIterableScalar::ScalarBased(iterable) => fold(
                iterable,
                IterableType::Scalar,
                self.iterator.name,
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
    exec_ctx.scalars.meet_fold_start();
    exec_ctx.scalars.set_iterable_value(iterator, fold_state)?;

    let result = instruction.execute(exec_ctx, trace_ctx);

    exec_ctx.scalars.remove_iterable_value(iterator);
    exec_ctx.scalars.meet_fold_end();

    result
}
