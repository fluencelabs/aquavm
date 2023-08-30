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

use super::ErrorDescriptor;
use super::ExecutionCidState;
use super::InstructionError;
use super::LastErrorDescriptor;
use super::Scalars;
use super::StreamMaps;
use super::Streams;
use crate::execution_step::ErrorEffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::ToErrorCode;

use air_execution_info_collector::InstructionTracker;
use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use air_interpreter_data::CidInfo;
use air_interpreter_data::ServiceResultCidAggregate;
use air_interpreter_interface::*;
use air_interpreter_signatures::SignatureStore;
use air_interpreter_signatures::SignatureTracker;

use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
#[derive(Default)]
pub(crate) struct ExecutionCtx<'i> {
    /// Contains all scalars.
    pub(crate) scalars: Scalars<'i>,

    /// Contains all streams.
    pub(crate) streams: Streams,

    /// Contains all stream maps.
    pub(crate) stream_maps: StreamMaps,

    /// Set of peer public keys that should receive resulted data.
    pub(crate) next_peer_pks: Vec<String>,

    /// Parameters passed from a host that describes host and contains info from a particle.
    pub(crate) run_parameters: RcRunParameters,

    /// Last error produced by local service.
    /// There is the special not-an-error value means there was no error.
    pub(crate) last_error_descriptor: LastErrorDescriptor,

    /// Error produced by some instructions, e.g. call, match, fail.
    pub(crate) error_descriptor: ErrorDescriptor,

    /// Indicates that previous executed subgraph is complete.
    /// A subgraph treats as a complete if all subgraph elements satisfy the following rules:
    ///   - at least one of par subgraphs is completed
    ///   - at least one of xor subgraphs is completed without an error
    ///   - all of seq subgraphs are completed
    ///   - call executed successfully (executed state is Executed)
    subgraph_completeness: bool,

    /// Tracker of all met instructions.
    pub(crate) tracker: InstructionTracker,

    /// Last call request id that was used as an id for call request in outcome.
    pub(crate) last_call_request_id: u32,

    /// Contains all executed results from a host side.
    pub(crate) call_results: CallResults,

    /// Tracks all functions that should be called from services.
    pub(crate) call_requests: CallRequests,

    /// CID-to-something trackers.
    pub(crate) cid_state: ExecutionCidState,

    /// Signatures' store.
    ///
    /// It contains peers' signatures for verification.
    pub(crate) signature_store: SignatureStore,

    /// Local signatures tracker.
    ///
    /// It gathers peers' CIDs (call results and canon results) stored in the trace either for signing (current peer's
    /// CIDs) or sign verification (other peers).
    pub(crate) signature_tracker: SignatureTracker,
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(
        prev_ingredients: ExecCtxIngredients,
        current_ingredients: ExecCtxIngredients,
        call_results: CallResults,
        run_parameters: &RunParameters,
    ) -> Self {
        let run_parameters = RcRunParameters::from_run_parameters(run_parameters);
        let streams = Streams::new();

        let cid_state = ExecutionCidState::from_cid_info(prev_ingredients.cid_info, current_ingredients.cid_info);
        // TODO we might keep both stores and merge them only with signature info collected into SignatureTracker
        let signature_store =
            SignatureStore::merge(prev_ingredients.signature_store, current_ingredients.signature_store);

        Self {
            run_parameters,
            subgraph_completeness: true,
            last_call_request_id: prev_ingredients.last_call_request_id,
            call_results,
            streams,
            cid_state,
            signature_store,
            ..<_>::default()
        }
    }

    pub(crate) fn last_error(&self) -> &InstructionError {
        self.last_error_descriptor.error()
    }

    pub(crate) fn error(&self) -> &InstructionError {
        self.error_descriptor.error()
    }

    pub(crate) fn next_call_request_id(&mut self) -> u32 {
        self.last_call_request_id += 1;
        self.last_call_request_id
    }

    pub(crate) fn record_call_cid(&mut self, peer_id: impl Into<Box<str>>, cid: &CID<ServiceResultCidAggregate>) {
        self.signature_tracker.register(peer_id, cid);
    }

    pub(crate) fn record_canon_cid(&mut self, peer_id: impl Into<Box<str>>, cid: &CID<CanonResultCidAggregate>) {
        self.signature_tracker.register(peer_id, cid);
    }
}

impl ExecutionCtx<'_> {
    pub(crate) fn make_subgraph_incomplete(&mut self) {
        self.subgraph_completeness = false;
    }

    pub(crate) fn is_subgraph_complete(&self) -> bool {
        self.subgraph_completeness
    }

    pub(crate) fn set_subgraph_completeness(&mut self, subgraph_complete: bool) {
        self.subgraph_completeness = subgraph_complete;
    }

    pub(crate) fn flush_subgraph_completeness(&mut self) {
        self.subgraph_completeness = true;
    }

    // Tetraplet option is an implicit source of error source peer_id information.
    pub(crate) fn set_errors_w_peerid(
        &mut self,
        error: &(impl ErrorEffectable + ToErrorCode + ToString),
        instruction: &str,
        tetraplet: Option<RcSecurityTetraplet>,
    ) -> String {
        let peer_id = match &tetraplet {
            // use tetraplet if they set, because an error could be propagated from data
            // (from CallServiceFailed state) and exec_ctx.run_parameters.current_peer_id won't mean
            // a peer where the error was occurred
            Some(tetraplet) => tetraplet.peer_pk.clone(),
            None => self.run_parameters.current_peer_id.to_string(),
        };
        self.last_error_descriptor.try_to_set_last_error_from_exec_error(
            error,
            instruction,
            Some(&peer_id),
            tetraplet.clone(),
        );
        self.error_descriptor
            .try_to_set_error_from_exec_error(error, instruction, Some(&peer_id), tetraplet);
        peer_id
    }

    // This routine sets %last_error%.$.peerid but does not set this field for :error:.
    pub(crate) fn set_errors(
        &mut self,
        error: &(impl ErrorEffectable + ToErrorCode + ToString),
        instruction: &str,
        tetraplet: Option<RcSecurityTetraplet>,
    ) {
        let peer_id = self.run_parameters.current_peer_id.as_ref();
        self.last_error_descriptor.try_to_set_last_error_from_exec_error(
            error,
            instruction,
            Some(peer_id),
            tetraplet.clone(),
        );
        self.error_descriptor
            .try_to_set_error_from_exec_error(error, instruction, None, tetraplet);
    }
}

/// Helper struct for ExecCtx construction.
#[derive(Debug, Clone)]
pub(crate) struct ExecCtxIngredients {
    pub(crate) last_call_request_id: u32,
    pub(crate) cid_info: CidInfo,
    pub(crate) signature_store: SignatureStore,
}

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// It reflects RunParameters structure due to limitation of the marine macro to support Rc.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct RcRunParameters {
    pub(crate) init_peer_id: Rc<str>,
    pub(crate) current_peer_id: Rc<String>,
    pub(crate) timestamp: u64,
    pub(crate) ttl: u32,
}

impl RcRunParameters {
    pub(crate) fn from_run_parameters(run_parameters: &RunParameters) -> Self {
        Self {
            init_peer_id: run_parameters.init_peer_id.as_str().into(),
            current_peer_id: Rc::new(run_parameters.current_peer_id.clone()),
            timestamp: run_parameters.timestamp,
            ttl: run_parameters.ttl,
        }
    }
}

impl Default for RcRunParameters {
    fn default() -> Self {
        Self {
            init_peer_id: "".into(),
            current_peer_id: Default::default(),
            timestamp: Default::default(),
            ttl: Default::default(),
        }
    }
}

impl<'i> Display for ExecutionCtx<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "scalars:")?;
        writeln!(f, "  {}", self.scalars)?;

        writeln!(f, "streams:")?;
        writeln!(f, "  {}", self.streams)?;

        writeln!(f, "current peer id: {}", self.run_parameters.current_peer_id)?;
        writeln!(f, "init peer id: {}", self.run_parameters.init_peer_id)?;
        writeln!(f, "timestamp: {}", self.run_parameters.timestamp)?;
        writeln!(f, "subgraph complete: {}", self.subgraph_completeness)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
