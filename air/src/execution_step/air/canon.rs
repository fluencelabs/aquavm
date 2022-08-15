/*
 * Copyright 2022 Fluence Labs Limited
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

use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::boxed_value::CanonStream;
use crate::execution_step::Generation;
use crate::trace_to_exec_err;
use crate::log_instruction;
use crate::CatchableError;

use air_parser::ast::Canon;
use air_trace_handler::MergerCanonResult;

use air_interpreter_data::{CanonResult, TracePos};

impl<'i> super::ExecutableInstruction<'i> for Canon<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        let canon_result = trace_to_exec_err!(trace_ctx.meet_canon_start(), self)?;

        let stream_with_positions = match canon_result {
            MergerCanonResult::CanonResult { stream_element_pos } => {
                let canon_stream = create_canon_stream_from_ids(&stream_element_pos, exec_ctx)?;
                StreamWithPositions {
                    canon_stream,
                    positions: stream_element_pos,
                }
            }
            MergerCanonResult::Empty => {
                create_canon_stream_from_name(self.stream.name, self.stream.position, exec_ctx)?
            }
        };

        exec_ctx
            .streams
            .add_canon(self.canon_stream.name.to_string(), stream_with_positions.canon_stream);
        trace_ctx.meet_canon_end(CanonResult {
            stream_element_ids: stream_with_positions.positions,
        });

        Ok(())
    }
}

fn create_canon_stream_from_ids(
    stream_elements_pos: &[TracePos],
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<CanonStream> {
    let values = stream_elements_pos
        .iter()
        .map(|pos| {
            exec_ctx
                .scalars
                .get_non_iterable_value_by_pos(*pos)
                .map(|value| value.clone())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let canon_stream = CanonStream::new(values);
    Ok(canon_stream)
}

struct StreamWithPositions {
    canon_stream: CanonStream,
    positions: Vec<TracePos>,
}

fn create_canon_stream_from_name(
    stream_name: &str,
    position: usize,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<StreamWithPositions> {
    let stream = exec_ctx
        .streams
        .get(stream_name, position)
        .ok_or_else(|| CatchableError::StreamsForCanonNotFound(stream_name.to_string()).into())?;
    let canon_stream = CanonStream::from_stream(stream);
    let positions = stream
        .iter(Generation::Last)
        .unwrap()
        .map(|value| value.trace_pos)
        .collect::<Vec<_>>();

    let result = StreamWithPositions {
        canon_stream,
        positions,
    };

    Ok(result)
}
