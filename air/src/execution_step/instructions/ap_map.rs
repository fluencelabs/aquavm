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

use super::ap::utils::generate_map_value_descriptor;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::execution_context::errors::StreamMapError::FloatMapKeyIsUnsupported;
use crate::execution_step::execution_context::errors::StreamMapError::UnsupportedMapKeyType;
use crate::execution_step::instructions::ap::apply_to_arguments::apply_to_arg;
use crate::execution_step::ValueAggregate;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::CatchableError;

// use crate::log_instruction;

use air_interpreter_data::GenerationIdx;
use air_parser::ast::ApMap;
use air_parser::ast::ApMapKey;
use air_parser::ast::Number;
use air_parser::ast::StreamMap;
use air_trace_handler::merger::MergerApResult;

impl<'i> super::ExecutableInstruction<'i> for ApMap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        // this applying should be at the very beginning of this function,
        // because it's necessary to check argument lambda, for more details see
        // https://github.com/fluencelabs/aquavm/issues/216
        let result = apply_to_arg(&self.value, exec_ctx, trace_ctx, true)?;

        let merger_ap_result = to_merger_ap_map_result(&self, trace_ctx)?;
        let maybe_generation = populate_context(&self.key, &self.map, &merger_ap_result, result, exec_ctx)?;
        maybe_update_trace(maybe_generation, trace_ctx);

        Ok(())
    }
}

fn to_merger_ap_map_result(instr: &impl ToString, trace_ctx: &mut TraceHandler) -> ExecutionResult<MergerApResult> {
    // WIP this won't work with MapAp
    let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
    Ok(merger_ap_result)
}

fn populate_context<'ctx>(
    key: &ApMapKey<'ctx>,
    ap_map_result: &StreamMap<'ctx>,
    merger_ap_result: &MergerApResult,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<Option<GenerationIdx>> {
    let value_descriptor = generate_map_value_descriptor(result, ap_map_result, merger_ap_result);
    match key {
        ApMapKey::Literal(s) => exec_ctx.stream_maps.add_stream_map_value(s, value_descriptor).map(Some),
        ApMapKey::Number(n) => match n {
            Number::Int(int) => exec_ctx
                .stream_maps
                .add_stream_map_value(int, value_descriptor)
                .map(Some),
            Number::Float(_) => Err(CatchableError::StreamMapError(FloatMapKeyIsUnsupported {
                variable_name: String::from(ap_map_result.name),
            })
            .into()),
        },
        _ => Err(CatchableError::StreamMapError(UnsupportedMapKeyType {
            variable_name: String::from(ap_map_result.name),
        })
        .into()),
    }
}

fn maybe_update_trace(maybe_generation: Option<GenerationIdx>, trace_ctx: &mut TraceHandler) {
    use air_interpreter_data::ApResult;

    if let Some(generation) = maybe_generation {
        let final_ap_result = ApResult::new(generation);
        trace_ctx.meet_ap_end(final_ap_result);
    }
}
