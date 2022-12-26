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
use crate::ExecutionError;
use crate::JValue;
use crate::UncatchableError;

use air_interpreter_data::CanonResult;
use air_parser::ast;
use air_trace_handler::merger::MergerCanonResult;

use std::borrow::Cow;

impl<'i> super::ExecutableInstruction<'i> for ast::Canon<'i> {
    #[tracing::instrument(level = "debug", skip(exec_ctx, trace_ctx))]
    fn execute(&self, exec_ctx: &mut ExecutionCtx<'i>, trace_ctx: &mut TraceHandler) -> ExecutionResult<()> {
        log_instruction!(call, exec_ctx, trace_ctx);
        let canon_result = trace_to_exec_err!(trace_ctx.meet_canon_start(), self)?;

        match canon_result {
            MergerCanonResult::CanonResult { canonicalized_element } => {
                handle_seen_canon(self, canonicalized_element, exec_ctx, trace_ctx)
            }
            MergerCanonResult::Empty => handle_unseen_canon(self, exec_ctx, trace_ctx),
        }
    }
}

fn handle_seen_canon(
    ast_canon: &ast::Canon<'_>,
    se_canon_stream: JValue,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let canon_stream = serde_json::from_value(se_canon_stream.clone()).map_err(|de_error| {
        ExecutionError::Uncatchable(UncatchableError::InvalidCanonStreamInData {
            canonicalized_stream: se_canon_stream.clone(),
            de_error,
        })
    })?;
    let canon_stream_with_se = StreamWithSerializedView {
        canon_stream,
        se_canon_stream,
    };

    epilog(ast_canon.canon_stream.name, canon_stream_with_se, exec_ctx, trace_ctx)
}

fn handle_unseen_canon(
    ast_canon: &ast::Canon<'_>,
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
) -> ExecutionResult<()> {
    let peer_id = crate::execution_step::air::resolve_peer_id_to_string(&ast_canon.peer_id, exec_ctx)?;

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
        se_canon_stream,
    } = stream_with_positions;

    exec_ctx
        .scalars
        .set_canon_value(canon_stream_name, canon_stream)
        .map(|_| ())?;
    trace_ctx.meet_canon_end(CanonResult::new(se_canon_stream));
    Ok(())
}

struct StreamWithSerializedView {
    canon_stream: CanonStream,
    se_canon_stream: JValue,
}

fn create_canon_stream_from_name(
    ast_canon: &ast::Canon<'_>,
    peer_id: String,
    exec_ctx: &ExecutionCtx<'_>,
) -> ExecutionResult<StreamWithSerializedView> {
    let stream = get_stream_or_default(ast_canon, exec_ctx);

    let canon_stream = CanonStream::from_stream(stream.as_ref(), peer_id);
    let se_canon_stream = serde_json::to_value(&canon_stream).expect("default serialized shouldn't fail");

    let result = StreamWithSerializedView {
        canon_stream,
        se_canon_stream,
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
