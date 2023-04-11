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

pub(crate) mod apply_to_arguments;
mod utils;

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::execution_context::errors::StreamMapError::FloatMapKeyIsUnsupported;
use crate::execution_step::execution_context::errors::StreamMapError::MapKeyIsAbsent;
use crate::execution_step::execution_context::errors::StreamMapError::UnsupportedMapKeyType;
use crate::execution_step::instructions::ValueAggregate;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::CatchableError;
use crate::JValue;
use crate::SecurityTetraplet;

use air_parser::ast::ApArgument;
use air_parser::ast::Number;
use apply_to_arguments::apply_to_arg;
use utils::*;

use air_interpreter_data::GenerationIdx;
use air_parser::ast;
use air_parser::ast::Ap;
use air_trace_handler::merger::MergerApResult;

use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for Ap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        execute_ap_kind(&self, None, &self.argument, &self.result, exec_ctx, trace_ctx)
    }
}

pub(crate) fn execute_ap_kind<'i>(
    instr: &impl ToString,
    ap_key: Option<&ast::ApArgument<'i>>,
    ap_argument: &ast::ApArgument<'_>,
    ap_result: &ast::ApResult<'i>,
    exec_ctx: &mut ExecutionCtx<'i>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    log_instruction!(call, exec_ctx, trace_ctx);
    let should_touch_trace = should_touch_trace(ap_result);
    // this applying should be at the very beginning of this function,
    // because it's necessary to check argument lambda, for more details see
    // https://github.com/fluencelabs/aquavm/issues/216
    let result = apply_to_arg(ap_argument, exec_ctx, trace_ctx, should_touch_trace)?;

    let merger_ap_result = to_merger_ap_result(ap_result, instr, trace_ctx)?;
    let maybe_generation = populate_context(ap_result, &merger_ap_result, ap_key, result, exec_ctx)?;
    maybe_update_trace(maybe_generation, trace_ctx);

    Ok(())
}

/// This function is intended to check whether a Ap instruction should produce
/// a new state in data.
fn should_touch_trace(ap_result: &ast::ApResult<'_>) -> bool {
    matches!(ap_result, ast::ApResult::Stream(_) | ast::ApResult::StreamMap(_))
}

fn to_merger_ap_result(
    ap_result: &ast::ApResult<'_>,
    instr: &impl ToString,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<MergerApResult> {
    match ap_result {
        ast::ApResult::Scalar(_) => Ok(MergerApResult::NotMet),
        ast::ApResult::Stream(_) | ast::ApResult::StreamMap(_) => {
            let merger_ap_result = trace_to_exec_err!(trace_ctx.meet_ap_start(), instr)?;
            Ok(merger_ap_result)
        }
    }
}

fn populate_context<'ctx>(
    ap_result: &ast::ApResult<'ctx>,
    merger_ap_result: &MergerApResult,
    key_argument: Option<&ApArgument<'ctx>>,
    result: ValueAggregate,
    exec_ctx: &mut ExecutionCtx<'ctx>,
) -> ExecutionResult<Option<GenerationIdx>> {
    match ap_result {
        ast::ApResult::Scalar(scalar) => exec_ctx.scalars.set_scalar_value(scalar.name, result).map(|_| None),
        ast::ApResult::Stream(stream) => {
            let value_descriptor = generate_value_descriptor(result, stream, merger_ap_result);
            exec_ctx.streams.add_stream_value(value_descriptor).map(Some)
        }
        ast::ApResult::StreamMap(stream_map) => {
            let value_descriptor = generate_map_value_descriptor(result, stream_map, merger_ap_result);
            match key_argument {
                Some(key) => match key {
                    ApArgument::Literal(s) => exec_ctx.stream_maps.add_stream_map_value(s, value_descriptor).map(Some),
                    ApArgument::Number(n) => match n {
                        Number::Int(int) => exec_ctx
                            .stream_maps
                            .add_stream_map_value(int, value_descriptor)
                            .map(Some),
                        Number::Float(_) => Err(CatchableError::StreamMapError(FloatMapKeyIsUnsupported {
                            variable_name: String::from(stream_map.name),
                        })
                        .into()),
                    },
                    _ => Err(CatchableError::StreamMapError(UnsupportedMapKeyType {
                        variable_name: String::from(stream_map.name),
                    })
                    .into()),
                },
                None => Err(CatchableError::StreamMapError(MapKeyIsAbsent {
                    variable_name: String::from(stream_map.name),
                })
                .into()),
            }
        }
    }
}

fn maybe_update_trace(maybe_generation: Option<GenerationIdx>, trace_ctx: &mut TraceHandler) {
    use air_interpreter_data::ApResult;

    if let Some(generation) = maybe_generation {
        let final_ap_result = ApResult::new(generation);
        trace_ctx.meet_ap_end(final_ap_result);
    }
}
