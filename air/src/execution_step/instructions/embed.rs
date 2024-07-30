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

use air_interpreter_starlark::execute as starlark_execute;
use air_parser::ast::Embed;
use air_parser::ast::ImmutableValue;

use crate::execution_step::errors::Joinable as _;
use crate::execution_step::resolver::Resolvable as _;
use crate::execution_step::LiteralAggregate;
use crate::execution_step::RcSecurityTetraplets;
use crate::execution_step::ValueAggregate;
use crate::joinable;
use crate::CatchableError;
use crate::ExecutionError;
use crate::JValue;
use crate::UncatchableError;

use super::ExecutableInstruction;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;

impl<'i> ExecutableInstruction<'i> for Embed<'i> {
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, _trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        let args = joinable!(collect_args(&self.args, exec_ctx), exec_ctx, ())?;

        let output_value = starlark_execute(self.script, args).map_err(classify_starlark_error)?;
        maybe_set_output_value(&self.output, output_value, exec_ctx)
    }
}

fn collect_args(
    args: &[ImmutableValue<'_>],
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<Vec<(JValue, RcSecurityTetraplets)>> {
    let mut result = Vec::with_capacity(args.len());

    for instruction_value in args {
        let (arg, tetraplet, _) = instruction_value.resolve(exec_ctx)?;
        result.push((arg, tetraplet));
    }
    Ok(result)
}

fn classify_starlark_error(err: air_interpreter_starlark::ExecutionError) -> ExecutionError {
    use air_interpreter_starlark::ExecutionError::*;

    match err {
        Fail(_, _) => todo!("handle it separately"),
        Value(_) | Function(_) | Other(_) => ExecutionError::Catchable(CatchableError::StalarkError(err).into()),
        Scope(_) | Lexer(_) | Internal(_) => ExecutionError::Uncatchable(UncatchableError::StalarkError(err).into()),
    }
}

fn maybe_set_output_value(
    embed_output_value: &air_parser::ast::EmbedOutputValue<'_>,
    result_value: air_interpreter_value::JValue,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> Result<(), ExecutionError> {
    match embed_output_value {
        air_parser::ast::EmbedOutputValue::Scalar(scalar) => {
            // TODO for now, we treat value produced by Starlark as a literal, as it has to be
            // same on every peer.
            let result = ValueAggregate::from_literal_result(LiteralAggregate::new(
                result_value,
                exec_ctx.run_parameters.init_peer_id.clone(),
                0.into(), // TODO is it correct?
            ));

            exec_ctx.scalars.set_scalar_value(scalar.name, result)?;
        }
        air_parser::ast::EmbedOutputValue::None => {}
    }
    Ok(())
}
