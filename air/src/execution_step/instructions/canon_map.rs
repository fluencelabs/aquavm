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

use super::canon::handle_seen_canon;
use super::canon::handle_unseen_canon;
use super::canon::GetStreamClosure;
use super::ExecutionCtx;
use super::ExecutionResult;
use super::TraceHandler;
use crate::execution_step::boxed_value::CanonStreamMap;
use crate::execution_step::boxed_value::CanonStreamMapWithProvenance;
use crate::execution_step::boxed_value::ConflictResolustionPolicy::Lww;
use crate::execution_step::instructions::canon::CanonEpilogClosure;
use crate::execution_step::instructions::canon::StreamWithSerializedView;
use crate::execution_step::instructions::canon_stream_map_scalar::get_stream_or_default_function;
use crate::log_instruction;
use crate::trace_to_exec_err;

use air_interpreter_data::CanonResult;
use air_parser::ast;
use air_trace_handler::merger::MergerCanonResult;

impl<'i> super::ExecutableInstruction<'i> for ast::CanonMap<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        let canon_result = trace_to_exec_err!(trace_ctx.meet_canon_start(), self)?;

        let epilog: &CanonEpilogClosure<'_> = &|stream_with_positions: StreamWithSerializedView,
                                                exec_ctx: &mut ExecutionCtx<'_>,
                                                trace_ctx: &mut TraceHandler|
         -> ExecutionResult<()> {
            let StreamWithSerializedView {
                canon_stream,
                canon_result_cid,
            } = stream_with_positions;

            let canon_stream_map: CanonStreamMap<'_> = CanonStreamMap::from_canon_stream(&canon_stream)?;

            let value = CanonStreamMapWithProvenance::new(canon_stream_map, canon_result_cid.clone());
            exec_ctx
                .scalars
                .set_canon_map_value(self.canon_stream_map.name, value)?;

            trace_ctx.meet_canon_end(CanonResult::new(canon_result_cid));

            Ok(())
        };

        match canon_result {
            MergerCanonResult::CanonResult(canon_result_cid) => {
                handle_seen_canon(epilog, canon_result_cid, exec_ctx, trace_ctx)
            }
            MergerCanonResult::Empty => {
                let get_stream_or_default: Box<GetStreamClosure<'_>> =
                    get_stream_or_default_function(self.stream_map.name, self.stream_map.position, Lww);
                handle_unseen_canon(epilog, &get_stream_or_default, &self.peer_id, exec_ctx, trace_ctx)
            }
        }
    }
}
