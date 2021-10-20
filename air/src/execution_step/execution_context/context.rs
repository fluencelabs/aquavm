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

use super::LastErrorDescriptor;
use super::LastErrorWithTetraplet;
use super::Scalars;
use crate::execution_step::boxed_value::Stream;

use air_execution_info_collector::InstructionTracker;
use air_interpreter_interface::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
#[derive(Default)]
pub(crate) struct ExecutionCtx<'i> {
    /// Contains all scalars.
    pub(crate) scalars: Scalars<'i>,

    /// Contains all streams.
    // TODO: use shared string (Rc<String>) to avoid copying.
    pub(crate) streams: HashMap<String, RefCell<Stream>>,

    /// Set of peer public keys that should receive resulted data.
    pub(crate) next_peer_pks: Vec<String>,

    /// PeerId of a peer executing this AIR script at the moment.
    pub(crate) current_peer_id: Rc<String>,

    /// PeerId of a peer send this AIR script.
    pub(crate) init_peer_id: String,

    /// Last error produced by local service.
    /// None means that there weren't any error.
    pub(crate) last_error: Option<LastErrorDescriptor>,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    pub(crate) last_error_could_be_set: bool,

    /// Indicates that previous executed subtree is complete.
    /// A subtree treats as a complete if all subtree elements satisfy the following rules:
    ///   - at least one of par subtrees is completed
    ///   - at least one of xor subtrees is completed without an error
    ///   - all of seq subtrees are completed
    ///   - call executed successfully (executed state is Executed)
    pub(crate) subtree_complete: bool,

    /// Tracker of all met instructions.
    pub(crate) tracker: InstructionTracker,

    /// Last call request id that was used as an id for call request in outcome.
    pub(crate) last_call_request_id: u32,

    /// Contains all executed results from a host side.
    pub(crate) call_results: CallResults,

    /// Tracks all functions that should be called from services.
    pub(crate) call_requests: CallRequests,
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(
        current_peer_id: String,
        init_peer_id: String,
        call_results: CallResults,
        last_call_request_id: u32,
    ) -> Self {
        let current_peer_id = Rc::new(current_peer_id);

        Self {
            current_peer_id,
            init_peer_id,
            subtree_complete: true,
            last_error_could_be_set: true,
            last_call_request_id,
            call_results,
            ..<_>::default()
        }
    }

    pub(crate) fn last_error(&self) -> LastErrorWithTetraplet {
        match &self.last_error {
            Some(error_descriptor) => LastErrorWithTetraplet::from_error_descriptor(error_descriptor, self),
            None => <_>::default(),
        }
    }

    pub(crate) fn next_call_request_id(&mut self) -> u32 {
        self.last_call_request_id += 1;
        self.last_call_request_id
    }
}

use std::fmt::Display;
use std::fmt::Formatter;

impl<'i> Display for ExecutionCtx<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "scalars:")?;
        writeln!(f, "  {}", self.scalars)?;

        writeln!(f, "streams:")?;
        for (name, stream) in self.streams.iter() {
            writeln!(f, "  {} => {}", name, stream.borrow())?;
        }

        writeln!(f, "current peer id: {}", self.current_peer_id)?;
        writeln!(f, "subtree complete: {}", self.subtree_complete)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
