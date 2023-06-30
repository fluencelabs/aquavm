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
use crate::execution_step::boxed_value::ConflictResolustionPolicy;
use crate::execution_step::boxed_value::ConflictResolustionPolicy::FWW;
use crate::execution_step::boxed_value::JValuable;
use crate::execution_step::instructions::canon::CanonEpilogClosure;
use crate::execution_step::instructions::canon::StreamWithSerializedView;
use crate::execution_step::CanonResultAggregate;
use crate::execution_step::Stream;
use crate::execution_step::ValueAggregate;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::UncatchableError;

use air_interpreter_data::CanonResult;
use air_parser::ast;
use air_parser::AirPos;
use air_trace_handler::merger::MergerCanonResult;

use std::borrow::Cow;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for ast::CanonStreamMapScalar<'i> {
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

            let value = JValuable::as_jvalue(&&canon_stream).into_owned();
            let tetraplet = canon_stream.tetraplet().clone();
            let position = trace_ctx.trace_pos().map_err(UncatchableError::from)?;
            let value = CanonResultAggregate::new(
                Rc::new(value),
                tetraplet.peer_pk.as_str().into(),
                &tetraplet.json_path,
                position,
            );
            let result = ValueAggregate::from_canon_result(value, canon_result_cid.clone());

            exec_ctx.scalars.set_scalar_value(self.scalar.name, result)?;
            trace_ctx.meet_canon_end(CanonResult::new(canon_result_cid));
            Ok(())
        };

        match canon_result {
            MergerCanonResult::CanonResult(canon_result_cid) => {
                handle_seen_canon(epilog, canon_result_cid, exec_ctx, trace_ctx)
            }
            MergerCanonResult::Empty => {
                let get_stream_or_default: Box<GetStreamClosure<'_>> =
                    get_stream_or_default_function(self.stream_map.name, self.stream_map.position, FWW);
                handle_unseen_canon(epilog, &get_stream_or_default, &self.peer_id, exec_ctx, trace_ctx)
            }
        }
    }
}

/// The resulting closure gets underlying stream from a StreamMap in a context
/// or returns a default empty stream,
/// it is crucial for deterministic behaviour, for more info see
/// github.com/fluencelabs/aquavm/issues/346.
pub(super) fn get_stream_or_default_function<'obj, 'n: 'obj>(
    stream_map_name: &'n str,
    position: AirPos,
    policy: ConflictResolustionPolicy,
) -> Box<GetStreamClosure<'obj>> {
    Box::new(move |exec_ctx: &mut ExecutionCtx<'_>| -> Cow<'_, Stream> {
        exec_ctx
            .stream_maps
            .get_mut(stream_map_name, position)
            .map(|stream_map| stream_map.get_unique_map_keys_stream(policy))
            .or_else(<_>::default)
            .unwrap()
    })
}
