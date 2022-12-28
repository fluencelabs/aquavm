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

use super::LastError;
use super::LastErrorDescriptor;
use super::Scalars;
use super::Streams;
use crate::JValue;

use air_execution_info_collector::InstructionTracker;
use air_interpreter_cid::CID;
use air_interpreter_data::CidStore;
use air_interpreter_data::CidTracker;
use air_interpreter_data::GlobalStreamGens;
use air_interpreter_data::RestrictedStreamGens;
use air_interpreter_interface::*;

use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
#[derive(Default)]
pub(crate) struct ExecutionCtx<'i> {
    /// Contains all scalars.
    pub(crate) scalars: Scalars<'i>,

    /// Contains all streams.
    pub(crate) streams: Streams,

    /// Set of peer public keys that should receive resulted data.
    pub(crate) next_peer_pks: Vec<String>,

    /// Parameters passed from a host that describes host and contains info from a particle.
    pub(crate) run_parameters: RcRunParameters,

    /// Last error produced by local service.
    /// None means that there weren't any error.
    pub(crate) last_error_descriptor: LastErrorDescriptor,

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

    /// Merged CID-to-value dictionaries
    pub(crate) cid_tracker: CidTracker,
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(
        prev_ingredients: ExecCtxIngredients,
        current_ingredients: ExecCtxIngredients,
        call_results: CallResults,
        run_parameters: RunParameters,
    ) -> Self {
        let run_parameters = RcRunParameters::from_run_parameters(run_parameters);
        let streams = Streams::from_data(
            prev_ingredients.global_streams,
            current_ingredients.global_streams,
            prev_ingredients.restricted_streams,
            current_ingredients.restricted_streams,
        );

        let cid_tracker = CidTracker::from_cid_stores(prev_ingredients.cid_store, current_ingredients.cid_store);

        Self {
            run_parameters,
            subgraph_completeness: true,
            last_call_request_id: prev_ingredients.last_call_request_id,
            call_results,
            streams,
            cid_tracker,
            ..<_>::default()
        }
    }

    pub(crate) fn last_error(&self) -> &LastError {
        self.last_error_descriptor.last_error()
    }

    pub(crate) fn next_call_request_id(&mut self) -> u32 {
        self.last_call_request_id += 1;
        self.last_call_request_id
    }

    pub(crate) fn get_value_by_cid(&self, cid: &CID) -> Option<Rc<JValue>> {
        self.cid_tracker.get(cid)
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
}

/// Helper struct for ExecCtx construction.
#[derive(Debug, Clone)]
pub(crate) struct ExecCtxIngredients {
    pub(crate) global_streams: GlobalStreamGens,
    pub(crate) last_call_request_id: u32,
    pub(crate) restricted_streams: RestrictedStreamGens,
    pub(crate) cid_store: CidStore<JValue>,
}

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// It reflects RunParameters structure due to limitation of the marine macro to support Rc.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct RcRunParameters {
    pub(crate) init_peer_id: Rc<String>,
    pub(crate) current_peer_id: Rc<String>,
    pub(crate) timestamp: u64,
    pub(crate) ttl: u32,
}

impl RcRunParameters {
    pub(crate) fn from_run_parameters(run_parameters: RunParameters) -> Self {
        Self {
            init_peer_id: Rc::new(run_parameters.init_peer_id),
            current_peer_id: Rc::new(run_parameters.current_peer_id),
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

        writeln!(f, "current peer id: {}", self.run_parameters.current_peer_id)?;
        writeln!(f, "init peer id: {}", self.run_parameters.init_peer_id)?;
        writeln!(f, "timestamp: {}", self.run_parameters.timestamp)?;
        writeln!(f, "subgraph complete: {}", self.subgraph_completeness)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
