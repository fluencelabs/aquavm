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

use crate::execution_step::errors::UncatchableError;
use crate::execution_step::instructions::resolve_peer_id_to_string;
use crate::execution_step::value_types::CanonStream;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::execution_step::TraceHandler;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use air_parser::ast::ResolvableToPeerIdVariable;

use std::rc::Rc;

pub(crate) type CanonEpilogClosure<'closure> = dyn Fn(CanonStream, Rc<CID<CanonResultCidAggregate>>, &mut ExecutionCtx<'_>, &mut TraceHandler) -> ExecutionResult<()>
    + 'closure;

pub(crate) type CreateCanonStreamClosure<'closure> = dyn Fn(&mut ExecutionCtx<'_>, String) -> CanonStream + 'closure;

pub(crate) fn handle_seen_canon(
    epilog: &CanonEpilogClosure<'_>,
    canon_result_cid: Rc<CID<CanonResultCidAggregate>>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let canon_result_agg = exec_ctx.cid_state.get_canon_result_by_cid(&canon_result_cid)?;
    let tetraplet = exec_ctx.cid_state.get_tetraplet_by_cid(&canon_result_agg.tetraplet)?;

    exec_ctx.record_canon_cid(&*tetraplet.peer_pk, &canon_result_cid);

    let value_cids = canon_result_agg.values.clone();
    let values = value_cids
        .iter()
        .map(|canon_value_cid| exec_ctx.cid_state.get_canon_value_by_cid(canon_value_cid))
        .collect::<Result<Vec<_>, _>>()?;
    let canon_stream = CanonStream::new(values, tetraplet);

    epilog(canon_stream, canon_result_cid, exec_ctx, trace_ctx)
}

pub(crate) fn handle_unseen_canon(
    epilog: &CanonEpilogClosure<'_>,
    create_canon_stream: &CreateCanonStreamClosure<'_>,
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
        // the former means that there was no canon associated state in data, and the latter means
        // that it can't be obtained on this peer, so it's intended not to call meet_canon_end here.
        return Ok(());
    }

    let canon_stream = create_canon_stream(exec_ctx, peer_id);
    let canon_result_cid = populate_cid_context(exec_ctx, &canon_stream)?;
    epilog(canon_stream, canon_result_cid, exec_ctx, trace_ctx)
}

fn populate_cid_context(
    exec_ctx: &mut ExecutionCtx<'_>,
    canon_stream: &CanonStream,
) -> ExecutionResult<Rc<CID<CanonResultCidAggregate>>> {
    let value_cids = canon_stream
        .iter()
        .map(|canon_value| exec_ctx.cid_state.track_canon_value(canon_value))
        .collect::<Result<_, _>>()?;

    let tetraplet = canon_stream.tetraplet();
    let tetraplet_cid = exec_ctx
        .cid_state
        .tetraplet_tracker
        .track_value(tetraplet.clone())
        .map_err(UncatchableError::from)?;

    let canon_result = CanonResultCidAggregate::new(tetraplet_cid, value_cids);
    let canon_result_cid = exec_ctx
        .cid_state
        .canon_result_tracker
        .track_value(canon_result)
        .map_err(UncatchableError::from)?;

    exec_ctx.record_canon_cid(&*tetraplet.peer_pk, &canon_result_cid);
    Ok(canon_result_cid)
}
