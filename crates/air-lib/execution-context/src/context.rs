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

use air_execution_info_collector::InstructionTracker;
use air_interpreter_interface::*;

use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
#[derive(Default)]
pub struct ExecutionCtx<'i> {
    /// Contains all scalars.
    pub scalars: Scalars<'i>,

    /// Contains all streams.
    pub streams: Streams,

    /// Set of peer public keys that should receive resulted data.
    pub next_peer_pks: Vec<String>,

    /// PeerId of a peer executing this AIR script at the moment.
    pub current_peer_id: Rc<String>,

    /// PeerId of a peer send this AIR script.
    pub init_peer_id: Rc<String>,

    /// Last error produced by local service.
    /// None means that there weren't any error.
    pub last_error_descriptor: LastErrorDescriptor,

    /// Indicates that previous executed subtree is complete.
    /// A subtree treats as a complete if all subtree elements satisfy the following rules:
    ///   - at least one of par subtrees is completed
    ///   - at least one of xor subtrees is completed without an error
    ///   - all of seq subtrees are completed
    ///   - call executed successfully (executed state is Executed)
    pub subtree_complete: bool,

    /// Tracker of all met instructions.
    pub tracker: InstructionTracker,

    /// Last call request id that was used as an id for call request in outcome.
    pub last_call_request_id: u32,

    /// Contains all executed results from a host side.
    pub call_results: CallResults,

    /// Tracks all functions that should be called from services.
    pub call_requests: CallRequests,
}

impl<'i> ExecutionCtx<'i> {
    pub fn new(
        current_peer_id: String,
        init_peer_id: String,
        call_results: CallResults,
        last_call_request_id: u32,
    ) -> Self {
        let current_peer_id = Rc::new(current_peer_id);

        Self {
            current_peer_id,
            init_peer_id: Rc::new(init_peer_id),
            subtree_complete: true,
            last_call_request_id,
            call_results,
            ..<_>::default()
        }
    }

    pub fn last_error(&self) -> &LastError {
        self.last_error_descriptor.last_error()
    }

    pub fn next_call_request_id(&mut self) -> u32 {
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
        writeln!(f, "  {}", self.streams)?;

        writeln!(f, "current peer id: {}", self.current_peer_id)?;
        writeln!(f, "subtree complete: {}", self.subtree_complete)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
