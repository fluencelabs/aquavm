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

use crate::execution_step::errors::UncatchableError;
use crate::execution_step::instructions::resolve_peer_id_to_string;
use crate::execution_step::value_types::CanonStream;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionResult;
use crate::execution_step::TraceHandler;

use air_interpreter_cid::CID;
use air_interpreter_data::CanonResult;
use air_interpreter_data::CanonResultCidAggregate;
use air_parser::ast::ResolvableToPeerIdVariable;
use polyplets::SecurityTetraplet;

pub(crate) type CanonEpilogClosure<'closure> = dyn Fn(CanonStream, CID<CanonResultCidAggregate>, &mut ExecutionCtx<'_>, &mut TraceHandler) -> ExecutionResult<()>
    + 'closure;

pub(crate) type CreateCanonStreamClosure<'closure> = dyn Fn(&mut ExecutionCtx<'_>, String) -> CanonStream + 'closure;

pub(crate) fn handle_seen_canon(
    peer_id_var: &ResolvableToPeerIdVariable<'_>,
    epilog: &CanonEpilogClosure<'_>,
    create_canon_stream: &CreateCanonStreamClosure<'_>,
    canon_result: CanonResult,
    peer_id: &ResolvableToPeerIdVariable<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    match canon_result {
        CanonResult::RequestSentBy(..) => {
            handle_canon_request_sent_by(epilog, create_canon_stream, peer_id, canon_result, exec_ctx, trace_ctx)
        }
        CanonResult::Executed(canon_result_cid) => {
            handle_canon_executed(peer_id_var, epilog, canon_result_cid, exec_ctx, trace_ctx)
        }
    }
}

pub(crate) fn handle_canon_request_sent_by(
    epilog: &CanonEpilogClosure<'_>,
    create_canon_stream: &CreateCanonStreamClosure<'_>,
    peer_id: &ResolvableToPeerIdVariable<'_>,
    canon_result: CanonResult,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    // we do not apply join behavior here because if state exists, the variable have been defined;
    // it cannot become undefined due to INV-1
    let peer_id = resolve_peer_id_to_string(peer_id, exec_ctx)?;

    if exec_ctx.run_parameters.current_peer_id.as_str() != peer_id {
        // nothing to execute yet; just leave the canon_result as is
        exec_ctx.make_subgraph_incomplete();
        trace_ctx.meet_canon_end(canon_result);
        Ok(())
    } else {
        create_canon_stream_for_first_time(epilog, create_canon_stream, peer_id, exec_ctx, trace_ctx)
    }
}

pub(crate) fn handle_canon_executed(
    peer_id_var: &ResolvableToPeerIdVariable<'_>,
    epilog: &CanonEpilogClosure<'_>,
    canon_result_cid: CID<CanonResultCidAggregate>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let peer_id = crate::execution_step::instructions::resolve_peer_id_to_string(peer_id_var, exec_ctx)?;
    let expected_tetraplet = SecurityTetraplet::new(peer_id, "", "", "");

    let canon_result_agg = exec_ctx.cid_state.get_canon_result_by_cid(&canon_result_cid)?;
    let tetraplet_cid = canon_result_agg.tetraplet.clone();
    let tetraplet = exec_ctx.cid_state.get_tetraplet_by_cid(&tetraplet_cid)?;

    verify_canon(&expected_tetraplet, &tetraplet)?;

    let value_cids = canon_result_agg.values.clone();
    let values = value_cids
        .iter()
        .map(|canon_value_cid| exec_ctx.cid_state.get_canon_value_by_cid(canon_value_cid))
        .collect::<Result<Vec<_>, _>>()?;

    populate_seen_cid_context(exec_ctx, &tetraplet.peer_pk, &canon_result_cid);

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
    use crate::execution_step::Joinable;
    use crate::joinable;

    let peer_id = joinable!(resolve_peer_id_to_string(peer_id, exec_ctx), exec_ctx, ())?;

    if exec_ctx.run_parameters.current_peer_id.as_str() != peer_id {
        exec_ctx.make_subgraph_incomplete();
        exec_ctx.next_peer_pks.push(peer_id);

        let canon_result = CanonResult::request_sent_by(exec_ctx.run_parameters.current_peer_id.clone());
        trace_ctx.meet_canon_end(canon_result);
        Ok(())
    } else {
        create_canon_stream_for_first_time(epilog, create_canon_stream, peer_id, exec_ctx, trace_ctx)
    }
}

fn create_canon_stream_for_first_time(
    epilog: &CanonEpilogClosure<'_>,
    create_canon_stream: &CreateCanonStreamClosure<'_>,
    peer_id: String,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let canon_stream = create_canon_stream(exec_ctx, peer_id);
    let canon_result_cid = populate_unseen_cid_context(exec_ctx, &canon_stream)?;
    epilog(canon_stream, canon_result_cid, exec_ctx, trace_ctx)
}

fn populate_seen_cid_context(
    exec_ctx: &mut ExecutionCtx<'_>,
    peer_id: &str,
    canon_result_cid: &CID<CanonResultCidAggregate>,
) {
    exec_ctx.record_canon_cid(peer_id, canon_result_cid)
}

fn populate_unseen_cid_context(
    exec_ctx: &mut ExecutionCtx<'_>,
    canon_stream: &CanonStream,
) -> ExecutionResult<CID<CanonResultCidAggregate>> {
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

    exec_ctx.record_canon_cid(&tetraplet.peer_pk, &canon_result_cid);
    Ok(canon_result_cid)
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
