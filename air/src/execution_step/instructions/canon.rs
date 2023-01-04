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
use crate::execution_step::Stream;
use crate::log_instruction;
use crate::trace_to_exec_err;
use crate::UncatchableError;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonCidAggregate;
use air_interpreter_data::CanonResult;
use air_parser::ast;
use air_trace_handler::merger::MergerCanonResult;
use polyplets::SecurityTetraplet;

use std::borrow::Cow;
use std::rc::Rc;

impl<'i> super::ExecutableInstruction<'i> for ast::Canon<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        let canon_result = trace_to_exec_err!(trace_ctx.meet_canon_start(), self)?;

        match canon_result {
            MergerCanonResult::CanonResult { tetraplet, values } => {
                handle_seen_canon(self, tetraplet, values, exec_ctx, trace_ctx)
            }
            MergerCanonResult::Empty => handle_unseen_canon(self, exec_ctx, trace_ctx),
        }
    }
}

fn handle_seen_canon(
    ast_canon: &ast::Canon<'_>,
    tetraplet_cid: Rc<CID<SecurityTetraplet>>,
    value_cids: Vec<Rc<CID<CanonCidAggregate>>>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let tetraplet = exec_ctx.cid_state.get_tetraplet_by_cid(&tetraplet_cid)?;
    let values = value_cids
        .iter()
        .map(|canon_value_cid| exec_ctx.cid_state.get_canon_value_by_cid(canon_value_cid))
        .collect::<Result<Vec<_>, _>>()?;

    let canon_stream = CanonStream::new(values, tetraplet);

    let canon_stream_with_se = StreamWithSerializedView {
        canon_stream,
        tetraplet_cid,
        value_cids,
    };

    epilog(ast_canon.canon_stream.name, canon_stream_with_se, exec_ctx, trace_ctx)
}

fn handle_unseen_canon(
    ast_canon: &ast::Canon<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let peer_id = crate::execution_step::instructions::resolve_peer_id_to_string(&ast_canon.peer_id, exec_ctx)?;

    if exec_ctx.run_parameters.current_peer_id.as_str() != peer_id {
        exec_ctx.subgraph_complete = false;
        exec_ctx.next_peer_pks.push(peer_id);
        //this branch is executed only when
        //  this canon instruction executes for the first time
        //  a peer is different from one set in peer_id of a this canon instruction
        //
        // the former means that there wasn't canon associated state in data, the latter that it
        // can't be obtained on this peer, so it's intended not to call meet_canon_end here.
        return Ok(());
    }

    let stream_with_positions = create_canon_stream_from_name(ast_canon, peer_id, exec_ctx)?;
    epilog(ast_canon.canon_stream.name, stream_with_positions, exec_ctx, trace_ctx)
}

fn epilog(
    canon_stream_name: &str,
    stream_with_positions: StreamWithSerializedView,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let StreamWithSerializedView {
        canon_stream,
        tetraplet_cid,
        value_cids,
    } = stream_with_positions;

    exec_ctx
        .scalars
        .set_canon_value(canon_stream_name, canon_stream)
        .map(|_| ())?;
    trace_ctx.meet_canon_end(CanonResult::new(tetraplet_cid, value_cids));
    Ok(())
}

struct StreamWithSerializedView {
    canon_stream: CanonStream,
    tetraplet_cid: Rc<CID<SecurityTetraplet>>,
    value_cids: Vec<Rc<CID<CanonCidAggregate>>>,
}

fn create_canon_stream_from_name(
    ast_canon: &ast::Canon<'_>,
    peer_id: String,
    exec_ctx: &mut ExecutionCtx<'_>,
) -> ExecutionResult<StreamWithSerializedView> {
    let stream = get_stream_or_default(ast_canon, exec_ctx);

    let canon_stream = CanonStream::from_stream(stream.as_ref(), peer_id);

    let value_cids = canon_stream
        .iter()
        .map(|val| -> Result<_, UncatchableError> {
            let canon_value_aggregate = CanonCidAggregate {
                value: exec_ctx.cid_state.value_tracker.record_value(val.result.clone())?,
                tetraplet: exec_ctx
                    .cid_state
                    .tetraplet_tracker
                    .record_value(val.tetraplet.clone())?,
            };
            Ok(exec_ctx.cid_state.canon_tracker.record_value(canon_value_aggregate)?)
        })
        .collect::<Result<_, _>>()?;
    let tetraplet_cid = exec_ctx
        .cid_state
        .tetraplet_tracker
        .record_value(canon_stream.tetraplet().clone())
        .map_err(UncatchableError::from)?;

    let result = StreamWithSerializedView {
        canon_stream,
        tetraplet_cid,
        value_cids,
    };

    Ok(result)
}

/// This function gets a stream from context or return a default empty stream,
/// it's crucial for deterministic behaviour, for more info see
/// github.com/fluencelabs/aquavm/issues/346
fn get_stream_or_default<'ctx>(ast_canon: &ast::Canon<'_>, exec_ctx: &'ctx ExecutionCtx<'_>) -> Cow<'ctx, Stream> {
    let maybe_stream = exec_ctx.streams.get(ast_canon.stream.name, ast_canon.stream.position);
    maybe_stream.map(Cow::Borrowed).unwrap_or_default()
}
