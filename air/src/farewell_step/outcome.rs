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

use super::FarewellError;
use crate::execution_step::ExecutionCtx;
use crate::execution_step::TraceHandler;
use crate::ExecutionError;
use crate::InterpreterOutcome;
use crate::ToErrorCode;
use crate::INTERPRETER_SUCCESS;

use air_interpreter_data::InterpreterDataEnvelope;
use air_interpreter_interface::CallRequests;
use air_interpreter_interface::CallRequestsRepr;
use air_interpreter_interface::SoftLimitsTriggering;
use air_interpreter_sede::ToSerialized;
use air_interpreter_signatures::KeyPair;
use air_utils::measure;
use fluence_keypair::error::SigningError;

use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

/// Create InterpreterOutcome from supplied execution context and trace handler,
/// set ret_code to INTERPRETER_SUCCESS.
#[tracing::instrument(skip_all)]
pub(crate) fn from_success_result(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
    keypair: &KeyPair,
    soft_limits_triggering: SoftLimitsTriggering,
) -> Result<InterpreterOutcome, InterpreterOutcome> {
    let (ret_code, error_message) = if exec_ctx.call_results.is_empty() {
        (INTERPRETER_SUCCESS, String::new())
    } else {
        let farewell_error = Rc::new(FarewellError::UnprocessedCallResult(exec_ctx.call_results.clone()));
        (farewell_error.to_error_code(), farewell_error.to_string())
    };

    let outcome = populate_outcome_from_contexts(
        exec_ctx,
        trace_handler,
        ret_code,
        error_message,
        keypair,
        soft_limits_triggering,
    );
    Ok(outcome)
}

/// Create InterpreterOutcome from supplied data and error,
/// set ret_code based on the error.
#[tracing::instrument]
pub(crate) fn from_uncatchable_error(
    data: impl Into<Vec<u8>> + Debug,
    error: impl ToErrorCode + ToString + Debug,
    soft_limits_triggering: SoftLimitsTriggering,
) -> InterpreterOutcome {
    let ret_code = error.to_error_code();
    let data = data.into();
    let call_requests = CallRequestsRepr
        .serialize(&CallRequests::new())
        .expect("default serializer shouldn't fail");

    InterpreterOutcome::new(
        ret_code,
        error.to_string(),
        data,
        vec![],
        call_requests,
        soft_limits_triggering,
    )
}

/// Create InterpreterOutcome from supplied execution context, trace handler, and error,
/// set ret_code based on the error.
#[tracing::instrument(skip(exec_ctx, trace_handler, keypair))]
pub(crate) fn from_execution_error(
    exec_ctx: ExecutionCtx<'_>,
    trace_handler: TraceHandler,
    error: impl ToErrorCode + ToString + Debug,
    keypair: &KeyPair,
    soft_limits_triggering: SoftLimitsTriggering,
) -> InterpreterOutcome {
    populate_outcome_from_contexts(
        exec_ctx,
        trace_handler,
        error.to_error_code(),
        error.to_string(),
        keypair,
        soft_limits_triggering,
    )
}

#[tracing::instrument(skip(exec_ctx, trace_handler, keypair), level = "info")]
fn populate_outcome_from_contexts(
    mut exec_ctx: ExecutionCtx<'_>,
    mut trace_handler: TraceHandler,
    ret_code: i64,
    error_message: String,
    keypair: &KeyPair,
    soft_limits_triggering: SoftLimitsTriggering,
) -> InterpreterOutcome {
    match compactify_streams(&mut exec_ctx, &mut trace_handler, soft_limits_triggering) {
        Ok(()) => {}
        Err(outcome) => return outcome,
    };

    match sign_result(&mut exec_ctx, keypair, soft_limits_triggering) {
        Ok(()) => {}
        Err(outcome) => return outcome,
    };

    let data = InterpreterDataEnvelope::from_execution_result(
        trace_handler.into_result_trace(),
        exec_ctx.cid_state.into(),
        exec_ctx.signature_store,
        exec_ctx.last_call_request_id,
        semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("cargo version is valid"),
    );
    let data = measure!(
        data.serialize().expect("default serializer shouldn't fail"),
        tracing::Level::INFO,
        "InterpreterDataEnv::serialize"
    );

    let next_peer_pks = dedup(exec_ctx.next_peer_pks);
    let call_requests = measure!(
        CallRequestsRepr
            .serialize(&exec_ctx.call_requests)
            .expect("default serializer shouldn't fail"),
        tracing::Level::INFO,
        "CallRequestsRepr.serialize",
    );
    InterpreterOutcome::new(
        ret_code,
        error_message,
        data,
        next_peer_pks,
        call_requests,
        soft_limits_triggering,
    )
}

fn compactify_streams(
    exec_ctx: &mut ExecutionCtx<'_>,
    trace_ctx: &mut TraceHandler,
    soft_limits_triggering: SoftLimitsTriggering,
) -> Result<(), InterpreterOutcome> {
    exec_ctx
        .streams
        .compactify(trace_ctx)
        .and_then(|_| exec_ctx.stream_maps.compactify(trace_ctx))
        .map_err(|err| execution_error_into_outcome(err, soft_limits_triggering))
}

fn sign_result(
    exec_ctx: &mut ExecutionCtx<'_>,
    keypair: &KeyPair,
    soft_limits_triggering: SoftLimitsTriggering,
) -> Result<(), InterpreterOutcome> {
    let current_signature = exec_ctx
        .peer_cid_tracker
        .gen_signature(&exec_ctx.run_parameters.salt, keypair)
        .map_err(|err| signing_error_into_outcome(err, soft_limits_triggering))?;

    let current_pubkey = keypair.public();
    exec_ctx.signature_store.put(current_pubkey, current_signature);

    Ok(())
}

// these methods are called only if there is an internal error in the interpreter and
// new execution trace was corrupted
fn execution_error_into_outcome(
    error: ExecutionError,
    soft_limits_triggering: SoftLimitsTriggering,
) -> InterpreterOutcome {
    InterpreterOutcome::new(
        error.to_error_code(),
        error.to_string(),
        vec![],
        vec![],
        <_>::default(),
        soft_limits_triggering,
    )
}

fn signing_error_into_outcome(error: SigningError, soft_limits_triggering: SoftLimitsTriggering) -> InterpreterOutcome {
    InterpreterOutcome::new(
        error.to_error_code(),
        error.to_string(),
        vec![],
        vec![],
        <_>::default(),
        soft_limits_triggering,
    )
}

/// Deduplicate values in a supplied vector.
fn dedup<T: Eq + Hash + Debug>(mut vec: Vec<T>) -> Vec<T> {
    use std::collections::HashSet;

    let set: HashSet<_> = vec.drain(..).collect();
    set.into_iter().collect()
}
