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
use crate::execution_step::boxed_value::CanonStreamWithProvenance;
use crate::execution_step::instructions::resolve_peer_id_to_string;
use crate::execution_step::Stream;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::UncatchableError;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::CanonResult;
use air_interpreter_data::CanonResultCidAggregate;
use air_parser::ast;
use air_parser::ast::ResolvableToPeerIdVariable;
use air_parser::AirPos;
use air_trace_handler::merger::MergerCanonResult;
use polyplets::SecurityTetraplet;

use std::borrow::Cow;
use std::rc::Rc;

pub(super) type CanonEpilogClosure<'ctx> =
    dyn for<'i> Fn(StreamWithSerializedView, &mut ExecutionCtx<'i>, &mut TraceHandler) -> ExecutionResult<()> + 'ctx;

impl<'i> super::ExecutableInstruction<'i> for ast::Canon<'i> {
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

            let value = CanonStreamWithProvenance::new(canon_stream, canon_result_cid.clone());
            exec_ctx.scalars.set_canon_value(self.canon_stream.name, value)?;

            trace_ctx.meet_canon_end(CanonResult::new(canon_result_cid));
            Ok(())
        };

        match canon_result {
            MergerCanonResult::CanonResult(canon_result_cid) => {
                handle_seen_canon(&self.peer_id, epilog, canon_result_cid, exec_ctx, trace_ctx)
            }
            MergerCanonResult::Empty => {
                let get_stream_or_default: Box<GetStreamClosure<'_>> =
                    get_stream_or_default_function(self.stream.name, self.stream.position);
                handle_unseen_canon(epilog, &get_stream_or_default, &self.peer_id, exec_ctx, trace_ctx)
            }
        }
    }
}

pub(super) fn handle_seen_canon(
    peer_id_var: &ast::ResolvableToPeerIdVariable<'_>,
    epilog: &CanonEpilogClosure<'_>,
    canon_result_cid: Rc<CID<CanonResultCidAggregate>>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let peer_id = crate::execution_step::instructions::resolve_peer_id_to_string(peer_id_var, exec_ctx)?;
    let expected_tetraplet = SecurityTetraplet::new(peer_id, "", "", "");

    let canon_result_agg = exec_ctx.cid_state.get_canon_result_by_cid(&canon_result_cid)?;
    let tetraplet_cid = canon_result_agg.tetraplet.clone();
    let tetraplet = exec_ctx.cid_state.get_tetraplet_by_cid(&tetraplet_cid)?;

    verify_canon(&expected_tetraplet, &tetraplet)?;

    exec_ctx.record_canon_cid(&tetraplet.peer_pk, &canon_result_cid);

    let value_cids = canon_result_agg.values.clone();
    let values = value_cids
        .iter()
        .map(|canon_value_cid| exec_ctx.cid_state.get_canon_value_by_cid(canon_value_cid))
        .collect::<Result<Vec<_>, _>>()?;

    let canon_stream = CanonStream::new(values, tetraplet);

    let canon_stream_with_se = StreamWithSerializedView {
        canon_stream,
        canon_result_cid,
    };

    epilog(canon_stream_with_se, exec_ctx, trace_ctx)
}

pub(super) fn handle_unseen_canon(
    epilog: &CanonEpilogClosure<'_>,
    get_stream_or_default: &GetStreamClosure<'_>,
    peer_id: &ResolvableToPeerIdVariable<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let peer_id = resolve_peer_id_to_string(peer_id, exec_ctx)?;

    if exec_ctx.run_parameters.current_peer_id.as_str() != peer_id {
        exec_ctx.make_subgraph_incomplete();
        exec_ctx.next_peer_pks.push(peer_id);
        //this branch is executed only when
        //  this canon instruction executes for the first time
        //  a peer is different from one set in peer_id of a this canon instruction
        //
        // the former means that there wasn't canon associated state in data, the latter that it
        // can't be obtained on this peer, so it's intended not to call meet_canon_end here.
        return Ok(());
    }

    let stream_with_positions = create_canon_stream_from_name(get_stream_or_default, peer_id, exec_ctx)?;
    epilog(stream_with_positions, exec_ctx, trace_ctx)
}

pub(super) struct StreamWithSerializedView {
    pub(super) canon_stream: CanonStream,
    pub(super) canon_result_cid: Rc<CID<CanonResultCidAggregate>>,
}

fn create_canon_stream_from_name(
    get_stream_or_default: &GetStreamClosure<'_>,
    peer_id: String,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<StreamWithSerializedView> {
    let stream = get_stream_or_default(exec_ctx);

    let canon_stream = CanonStream::from_stream(stream.as_ref(), peer_id);

    let value_cids = canon_stream
        .iter()
        .map(|val| -> Result<_, UncatchableError> {
            let canon_value_aggregate = CanonCidAggregate {
                value: exec_ctx
                    .cid_state
                    .value_tracker
                    .record_value(val.get_result().clone())?,
                tetraplet: exec_ctx.cid_state.tetraplet_tracker.record_value(val.get_tetraplet())?,
                provenance: val.get_provenance(),
            };
            Ok(exec_ctx
                .cid_state
                .canon_element_tracker
                .record_value(canon_value_aggregate)?)
        })
        .collect::<Result<_, _>>()?;
    let tetraplet = canon_stream.tetraplet();
    let tetraplet_cid = exec_ctx
        .cid_state
        .tetraplet_tracker
        .record_value(tetraplet.clone())
        .map_err(UncatchableError::from)?;

    let canon_result = CanonResultCidAggregate::new(tetraplet_cid, value_cids);
    let canon_result_cid = exec_ctx
        .cid_state
        .canon_result_tracker
        .record_value(canon_result)
        .map_err(UncatchableError::from)?;

    exec_ctx.record_canon_cid(&tetraplet.peer_pk, &canon_result_cid);

    let result = StreamWithSerializedView {
        canon_stream,
        canon_result_cid,
    };

    Ok(result)
}

pub(super) type GetStreamClosure<'obj> = dyn for<'ctx> Fn(&'ctx mut ExecutionCtx<'_>) -> Cow<'ctx, Stream> + 'obj;

/// The resulting closure gets underlying stream in a context
/// or returns a default empty stream,
/// it is crucial for deterministic behaviour, for more info see
/// github.com/fluencelabs/aquavm/issues/346.
fn get_stream_or_default_function<'obj, 'n: 'obj>(
    stream_name: &'n str,
    position: AirPos,
) -> Box<GetStreamClosure<'obj>> {
    Box::new(move |exec_ctx: &mut ExecutionCtx<'_>| -> Cow<'_, Stream> {
        exec_ctx
            .streams
            .get(stream_name, position)
            .map(Cow::Borrowed)
            .unwrap_or_default()
    })
}

pub(crate) fn verify_canon(
    expected_tetraplet: &SecurityTetraplet,
    stored_tetraplet: &SecurityTetraplet,
) -> Result<(), UncatchableError> {
    if expected_tetraplet != stored_tetraplet {
        return Err(UncatchableError::InstructionParametersMismatch {
            param: "canon tetraplet",
            expected_value: format!("{expected_tetraplet:?}"),
            stored_value: format!("{stored_tetraplet:?}"),
        });
    }
    Ok(())
}
