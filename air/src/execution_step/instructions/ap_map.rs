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

use air_interpreter_data::GenerationIdx;
use air_parser::ast::ApMap;
use air_parser::ast::ApMapKey;
use air_parser::ast::Number;
use air_parser::ast::StreamMap;
use air_trace_handler::merger::MergerApResult;

impl<'i> super::ExecutableInstruction<'i> for ApMap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(ap, exec_ctx, trace_ctx);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = apply_to_arg(&self.value, exec_ctx, trace_ctx, true)?;

        let merger_ap_result = to_merger_ap_map_result(&self, trace_ctx)?;
        let key = resolve_if_needed(&self.key, exec_ctx, self.map.name)?;
        let generation = populate_context(key, &self.map, &merger_ap_result, result, exec_ctx)?;
        maybe_update_trace(generation, trace_ctx);

        Ok(())
    }
}

fn to_merger_ap_map_result(instr: &impl ToString, trace_ctx: &mut TraceHandler) -> ExecutionResult<MergerApResult> {
    let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
    Ok(merger_ap_result)
}

fn populate_context<'ctx>(
    key: StreamMapKey<'ctx>,
    ap_map_result: &StreamMap<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<GenerationIdx> {
    let value_descriptor = generate_map_value_descriptor(result, ap_map_result, merger_ap_result);
    exec_ctx.stream_maps.add_stream_map_value(key, value_descriptor)
}

fn resolve_if_needed<'ctx>(
    key: &ApMapKey<'ctx>,
    exec_ctx: &mut ExecutionCtx<'ctx>,
    map_name: &str,
) -> Result<StreamMapKey<'ctx>, ExecutionError> {
    match key {
        &ApMapKey::Literal(s) => Ok(s.into()),
        ApMapKey::Number(n) => match n {
            &Number::Int(i) => Ok(i.into()),
            Number::Float(_) => Err(CatchableError::StreamMapError(unsupported_map_key_type(map_name)).into()),
        },
        ApMapKey::Scalar(s) => resolve(s, exec_ctx, map_name),
        ApMapKey::ScalarWithLambda(s) => resolve(s, exec_ctx, map_name),
        ApMapKey::CanonStreamWithLambda(c) => resolve(c, exec_ctx, map_name),
    }
}

fn resolve<'ctx>(
    resolvable: &impl Resolvable,
    exec_ctx: &mut ExecutionCtx<'_>,
    map_name: &str,
) -> Result<StreamMapKey<'ctx>, ExecutionError> {
    let (value, _, _) = resolvable.resolve(exec_ctx)?;
    StreamMapKey::from_value(value, map_name)
}

fn maybe_update_trace(generation: GenerationIdx, trace_ctx: &mut TraceHandler) {
    use air_interpreter_data::ApResult;

    let final_ap_result = ApResult::new(generation);
    trace_ctx.meet_ap_end(final_ap_result);
}
