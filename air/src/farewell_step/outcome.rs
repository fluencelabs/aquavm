/*
 * Copyright 2020 Fluence Labs Limited
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

use super::FarewellError;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::TraceHandler;
use crate::InterpreterOutcome;
use crate::ToErrorCode;
use crate::INTERPRETER_SUCCESS;

use air_interpreter_data::InterpreterData;
use air_interpreter_interface::CallRequests;
use air_utils::measure;

use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

/// Create InterpreterOutcome from supplied execution context and trace handler,
/// set ret_code to INTERPRETER_SUCCESS.
#[tracing::instrument(skip_all)]
pub(crate) fn from_success_result(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    let (ret_code, error_message) = if exec_ctx.call_results.is_empty() {
        (INTERPRETER_SUCCESS, String::new())
    } else {
        let farewell_error = Rc::new(FarewellError::CallResultsNotEmpty(exec_ctx.call_results.clone()));
        (farewell_error.to_error_code(), farewell_error.to_string())
    };

    let outcome = populate_outcome_from_contexts(exec_ctx, trace_handler, ret_code, error_message);
    Ok(outcome)
}

/// Create InterpreterOutcome from supplied data and error,
/// set ret_code based on the error.
#[tracing::instrument]
pub(crate) fn from_uncatchable_error(
    data: impl Into<Vec<u8>> + Debug,
    error: impl ToErrorCode + ToString + Debug,
) -> InterpreterOutcome {
    let ret_code = error.to_error_code();
    let data = data.into();
    let call_requests = serde_json::to_vec(&CallRequests::new()).expect("default serializer shouldn't fail");

    InterpreterOutcome {
        ret_code,
        error_message: error.to_string(),
        data,
        next_peer_pks: vec![],
        call_requests,
    }
}

/// Create InterpreterOutcome from supplied execution context, trace handler, and error,
/// set ret_code based on the error.
#[tracing::instrument(skip(exec_ctx, trace_handler))]
pub(crate) fn from_execution_error(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
    error: impl ToErrorCode + ToString + Debug,
) -> InterpreterOutcome {
    populate_outcome_from_contexts(exec_ctx, trace_handler, error.to_error_code(), error.to_string())
}

#[tracing::instrument(skip(exec_ctx, trace_handler))]
fn populate_outcome_from_contexts(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
    ret_code: i64,
    error_message: String,
) -> InterpreterOutcome {
    let (global_streams, restricted_streams) = exec_ctx.streams.into_streams_data();
    let data = InterpreterData::from_execution_result(
        trace_handler.into_result_trace(),
        global_streams,
        restricted_streams,
        exec_ctx.last_call_request_id,
    );
    let data = measure!(
        serde_json::to_vec(&data).expect("default serializer shouldn't fail"),
        tracing::Level::INFO,
        "serde_json::to_vec(data)"
    );
    let next_peer_pks = dedup(exec_ctx.next_peer_pks);
    let call_requests = measure!(
        serde_json::to_vec(&exec_ctx.call_requests).expect("default serializer shouldn't fail"),
        tracing::Level::INFO,
        "serde_json::to_vec(call_results)",
    );

    InterpreterOutcome {
        ret_code,
        error_message,
        data,
        next_peer_pks,
        call_requests,
    }
}

/// Deduplicate values in a supplied vector.
fn dedup<T: Eq + Hash + Debug>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}
