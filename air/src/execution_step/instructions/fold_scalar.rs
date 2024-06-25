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

use super::fold::*;
use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::value_types::IterableValue;
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
            FoldScalarIterable::Scalar(scalar) => {
                joinable!(create_scalar_iterable(exec_ctx, scalar.name), exec_ctx, ())?
            }
            FoldScalarIterable::ScalarWithLambda(scalar) => {
                joinable!(create_scalar_wl_iterable(scalar, exec_ctx), exec_ctx, ())?
            }
            FoldScalarIterable::CanonStream(canon_stream) => {
                joinable!(create_canon_stream_iterable_value(canon_stream, exec_ctx), exec_ctx, ())?
            }
            FoldScalarIterable::CanonStreamMap(canon_stream_map) => joinable!(
                create_canon_stream_map_iterable_value(canon_stream_map, exec_ctx),
                exec_ctx,
                ()
            )?,
            FoldScalarIterable::CanonStreamMapWithLambda(canon_stream_map) => joinable!(
                create_canon_stream_map_wl_iterable_value(canon_stream_map, exec_ctx),
                exec_ctx,
                ()
            )?,
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
                self.last_instruction.clone(),
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
    last_instruction: Option<Rc<Instruction<'i>>>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let fold_state = FoldState::from_iterable(iterable, iterable_type, instruction.clone(), last_instruction);
    exec_ctx.scalars.meet_fold_start();
    exec_ctx.scalars.set_iterable_value(iterator, fold_state)?;

    let result = instruction.execute(exec_ctx, trace_ctx);

    exec_ctx.scalars.remove_iterable_value(iterator);
    exec_ctx.scalars.meet_fold_end();

    result
}
