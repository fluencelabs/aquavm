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

use super::ErrorDescriptor;
use super::ExecutionCidState;
use super::InstructionError;
use super::LastErrorDescriptor;
use super::Scalars;
use super::StreamMaps;
use super::Streams;
use crate::execution_step::ErrorAffectable;
use crate::execution_step::RcSecurityTetraplet;
use crate::ToErrorCode;

use air_execution_info_collector::InstructionTracker;
use air_interpreter_cid::CID;
use air_interpreter_data::CanonResultCidAggregate;
use air_interpreter_data::CidInfo;
use air_interpreter_data::ServiceResultCidAggregate;
use air_interpreter_interface::*;
use air_interpreter_signatures::PeerCidTracker;
use air_interpreter_signatures::SignatureStore;

use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
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

    /// Current peer's CID tracker.
    ///
    /// It gathers current peer's CIDs (call results and canon results) for further signing.
    pub(crate) peer_cid_tracker: PeerCidTracker,
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(
        prev_ingredients: ExecCtxIngredients,
        current_ingredients: ExecCtxIngredients,
        call_results: CallResults,
        signature_store: SignatureStore,
        run_parameters: &RunParameters,
    ) -> Self {
        let run_parameters = RcRunParameters::from_run_parameters(run_parameters);
        let streams = Streams::new();

        let cid_state = ExecutionCidState::from_cid_info(prev_ingredients.cid_info, current_ingredients.cid_info);

        let peer_cid_tracker = PeerCidTracker::new(run_parameters.current_peer_id.clone());

        Self {
            run_parameters,
            subgraph_completeness: true,
            last_call_request_id: prev_ingredients.last_call_request_id,
            call_results,
            streams,
            stream_maps: <_>::default(),
            cid_state,
            signature_store,
            peer_cid_tracker,
            scalars: <_>::default(),
            next_peer_pks: <_>::default(),
            last_error_descriptor: <_>::default(),
            error_descriptor: <_>::default(),
            tracker: <_>::default(),
            call_requests: <_>::default(),
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

    pub(crate) fn record_call_cid(&mut self, peer_id: &str, cid: &CID<ServiceResultCidAggregate>) {
        self.peer_cid_tracker.register(peer_id, cid);
    }

    pub(crate) fn record_canon_cid(&mut self, peer_id: &str, cid: &CID<CanonResultCidAggregate>) {
        self.peer_cid_tracker.register(peer_id, cid);
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

    // This routine sets %last_error% and :error:.
    // Most instructions, except Call, Canon, CanonMapScalar does not set :error:.$.peer_id b/c
    // it would be a non-deterministic peer_id.
    pub(crate) fn set_errors(
        &mut self,
        error: &(impl ErrorAffectable + ToErrorCode + ToString),
        instruction: &str,
        tetraplet: Option<RcSecurityTetraplet>,
        use_tetraplet_and_log_peer_id: bool,
    ) {
        let last_error_peer_id = match &tetraplet {
            // use tetraplet if they set, because an error could be propagated from data
            // (from CallServiceFailed state) and exec_ctx.run_parameters.current_peer_id won't mean
            // a peer where the error was occurred
            Some(tetraplet) if use_tetraplet_and_log_peer_id => Some(tetraplet.peer_pk.as_str()),
            _ => Some(self.run_parameters.current_peer_id.as_str()),
        };

        self.last_error_descriptor.try_to_set_last_error_from_exec_error(
            error,
            instruction,
            last_error_peer_id,
            tetraplet.clone(),
        );

        let peer_id = if use_tetraplet_and_log_peer_id {
            last_error_peer_id
        } else {
            None
        };

        self.error_descriptor
            .try_to_set_error_from_exec_error(error, instruction, peer_id, tetraplet.clone());

        self.error_descriptor.disable_error_setting();
    }
}

/// Helper struct for ExecCtx construction.
#[derive(Debug, Clone)]
pub(crate) struct ExecCtxIngredients {
    pub(crate) last_call_request_id: u32,
    pub(crate) cid_info: CidInfo,
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
    // TODO use [u8]
    pub(crate) salt: Rc<str>,
    pub(crate) timestamp: u64,
    pub(crate) ttl: u32,
}

impl RcRunParameters {
    pub(crate) fn from_run_parameters(run_parameters: &RunParameters) -> Self {
        Self {
            init_peer_id: run_parameters.init_peer_id.as_str().into(),
            current_peer_id: Rc::new(run_parameters.current_peer_id.clone()),
            salt: run_parameters.particle_id.as_str().into(),
            timestamp: run_parameters.timestamp,
            ttl: run_parameters.ttl,
        }
    }
}

impl<'i> Display for ExecutionCtx<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "scalars:")?;
        writeln!(f, "  {}", self.scalars)?;

        writeln!(f, "streams:")?;
        writeln!(f, "  {}", self.streams)?;

        writeln!(f, "stream_maps:")?;
        writeln!(f, "  {}", self.stream_maps)?;

        writeln!(f, "current peer id: {}", self.run_parameters.current_peer_id)?;
        writeln!(f, "init peer id: {}", self.run_parameters.init_peer_id)?;
        writeln!(f, "timestamp: {}", self.run_parameters.timestamp)?;
        writeln!(f, "subgraph complete: {}", self.subgraph_completeness)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
