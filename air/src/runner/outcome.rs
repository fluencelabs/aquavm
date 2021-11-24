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

use crate::execution_step::ExecutionCtx;
use crate::execution_step::ExecutionError;
use crate::execution_step::TraceHandler;
use crate::preparation_step::PreparationError;
use crate::InterpreterOutcome;
use crate::INTERPRETER_SUCCESS;

use air_interpreter_data::InterpreterData;
use air_interpreter_interface::CallRequests;

use std::hash::Hash;
use std::rc::Rc;

/// Create InterpreterOutcome from supplied execution context and trace handler,
/// set ret_code to INTERPRETER_SUCCESS.
pub(crate) fn from_success_result(exec_ctx: ExecutionCtx<'_>, trace_handler: TraceHandler) -> InterpreterOutcome {
    let (global_streams, restricted_streams) = exec_ctx.streams.into_streams_data();
    let data = InterpreterData::from_execution_result(
        trace_handler.into_result_trace(),
        global_streams,
        restricted_streams,
        exec_ctx.last_call_request_id,
    );
    let data = serde_json::to_vec(&data).expect("default serializer shouldn't fail");
    let next_peer_pks = dedup(exec_ctx.next_peer_pks);
    let call_requests = serde_json::to_vec(&exec_ctx.call_requests).expect("default serializer shouldn't fail");

    InterpreterOutcome {
        ret_code: INTERPRETER_SUCCESS,
        error_message: String::new(),
        data,
        next_peer_pks,
        call_requests,
    }
}

/// Create InterpreterOutcome from supplied data and error,
/// set ret_code based on the error.
pub(crate) fn from_preparation_error(data: impl Into<Vec<u8>>, err: PreparationError) -> InterpreterOutcome {
    let ret_code = err.to_error_code() as i32;
    let data = data.into();
    let call_requests = serde_json::to_vec(&CallRequests::new()).expect("default serializer shouldn't fail");

    InterpreterOutcome {
        ret_code,
        error_message: format!("{}", err),
        data,
        next_peer_pks: vec![],
        call_requests,
    }
}

/// Create InterpreterOutcome from supplied data and error,
/// set ret_code based on the error.
pub(crate) fn from_trace_error(data: impl Into<Vec<u8>>, err: Rc<ExecutionError>) -> InterpreterOutcome {
    let ret_code = err.to_error_code() as i32;
    let data = data.into();
    let call_requests = serde_json::to_vec(&CallRequests::new()).expect("default serializer shouldn't fail");

    InterpreterOutcome {
        ret_code,
        error_message: format!("{}", err),
        data,
        next_peer_pks: vec![],
        call_requests,
    }
}

/// Create InterpreterOutcome from supplied execution context, trace handler, and error,
/// set ret_code based on the error.
pub(crate) fn from_execution_error(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
    err: Rc<ExecutionError>,
) -> InterpreterOutcome {
    let ret_code = err.to_error_code() as i32;

    let (global_streams, restricted_streams) = exec_ctx.streams.into_streams_data();
    let data = InterpreterData::from_execution_result(
        trace_handler.into_result_trace(),
        global_streams,
        restricted_streams,
        exec_ctx.last_call_request_id,
    );
    let data = serde_json::to_vec(&data).expect("default serializer shouldn't fail");
    let next_peer_pks = dedup(exec_ctx.next_peer_pks);
    let call_requests = serde_json::to_vec(&exec_ctx.call_requests).expect("default serializer shouldn't fail");

    InterpreterOutcome {
        ret_code,
        error_message: format!("{}", err),
        data,
        next_peer_pks,
        call_requests,
    }
}

/// Deduplicate values in a supplied vector.
fn dedup<T: Eq + Hash>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}
