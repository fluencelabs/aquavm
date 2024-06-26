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

use super::ap::utils::generate_map_value_descriptor;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::execution_context::stream_map_key::StreamMapKey;
use crate::execution_step::instructions::ap::apply_to_arguments::apply_to_arg;
use crate::execution_step::resolver::Resolvable;
use crate::execution_step::ValueAggregate;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::unsupported_map_key_type;
use crate::CatchableError;
use crate::ExecutionError;

use air_interpreter_data::ApResult;
use air_parser::ast::ApMap;
use air_parser::ast::StreamMap;
use air_parser::ast::StreamMapKeyClause;
use air_trace_handler::merger::MergerApResult;

impl<'i> super::ExecutableInstruction<'i> for ApMap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        use crate::execution_step::Joinable;
        use crate::joinable;

        log_instruction!(ap, exec_ctx, trace_ctx);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = joinable!(apply_to_arg(&self.value, exec_ctx, trace_ctx, true), exec_ctx, ())?;

        let merger_ap_result = to_merger_ap_map_result(&self, trace_ctx)?;
        let key = joinable!(resolve_key_if_needed(&self.key, exec_ctx, self.map.name), exec_ctx, ())?;
        populate_context(key, &self.map, &merger_ap_result, result, exec_ctx)?;
        trace_ctx.meet_ap_end(ApResult::stub());

        Ok(())
    }
}

fn to_merger_ap_map_result(instr: &impl ToString, trace_ctx: &mut TraceHandler) -> ExecutionResult<MergerApResult> {
    let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
    Ok(merger_ap_result)
}

fn populate_context<'ctx>(
    key: StreamMapKey,
    ap_map_result: &StreamMap<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<()> {
    let value_descriptor = generate_map_value_descriptor(result, ap_map_result, merger_ap_result);
    exec_ctx.stream_maps.add_stream_map_value(key, value_descriptor)
}

fn resolve_key_if_needed<'ctx>(
    key: &StreamMapKeyClause<'ctx>,
    exec_ctx: &ExecutionCtx<'ctx>,
    map_name: &str,
) -> Result<StreamMapKey, ExecutionError> {
    match key {
        StreamMapKeyClause::Literal(s) => Ok(s.clone().into()),
        StreamMapKeyClause::Int(i) => Ok(i.to_owned().into()),
        StreamMapKeyClause::Scalar(s) => resolve(s, exec_ctx, map_name),
        StreamMapKeyClause::ScalarWithLambda(s) => resolve(s, exec_ctx, map_name),
        StreamMapKeyClause::CanonStreamWithLambda(c) => resolve(c, exec_ctx, map_name),
    }
}

fn resolve(
    resolvable: &impl Resolvable,
    exec_ctx: &ExecutionCtx<'_>,
    map_name: &str,
) -> Result<StreamMapKey, ExecutionError> {
    let (value, _, _) = resolvable.resolve(exec_ctx)?;
    StreamMapKey::from_value(value).ok_or(CatchableError::StreamMapError(unsupported_map_key_type(map_name)).into())
}
