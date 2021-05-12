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

mod avalue;

pub(crate) use avalue::AValue;
pub(crate) use avalue::ResolvedCallResult;

use crate::execution::ExecutionError;
use crate::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

use std::collections::HashMap;
use std::collections::VecDeque;

use std::rc::Rc;

/// Contains all necessary state needed to execute AIR script.
#[derive(Default)]
pub(crate) struct ExecutionCtx<'i> {
    /// Contains all set variables.
    // TODO: use shared string (Rc<String>) to avoid copying.
    pub data_cache: HashMap<String, AValue<'i>>,

    /// Set of peer public keys that should receive resulted data.
    pub next_peer_pks: Vec<String>,

    /// PeerId of a peer executing this AIR script at the moment.
    pub current_peer_id: String,

    /// PeerId of a peer send this AIR script.
    pub init_peer_id: String,

    /// Last error produced by local service.
    /// None means that there weren't any error.
    pub last_error: Option<LastErrorDescriptor>,

    /// True, if last error could be set. This flag is used to distinguish
    /// whether an error is being bubbled up from the bottom or just encountered.
    pub last_error_could_be_set: bool,

    /// Indicates that previous executed subtree is complete.
    /// A subtree treats as a complete if all subtree elements satisfy the following rules:
    ///   - at least one of par subtrees is completed
    ///   - at least one of xor substree is completed without an error
    ///   - all of seq subtrees are completed
    ///   - call executed successfully (executed state is Executed)
    pub subtree_complete: bool,

    /// List of met folds used to determine whether a variable can be shadowed.
    pub met_folds: VecDeque<&'i str>,
}

#[derive(Debug)]
pub(crate) struct LastErrorDescriptor {
    pub(crate) error: Rc<ExecutionError>,
    pub(crate) instruction: String,
    pub(crate) tetraplet: Option<SecurityTetraplet>,
}

impl LastErrorDescriptor {
    pub(crate) fn new(error: Rc<ExecutionError>, instruction: String, tetraplet: Option<SecurityTetraplet>) -> Self {
        Self {
            error,
            instruction,
            tetraplet,
        }
    }

    pub(crate) fn serialize(&self) -> String {
        #[derive(Serialize, Deserialize)]
        pub(crate) struct LastError<'s> {
            pub(crate) error: String,
            pub(crate) instruction: &'s str,
        }

        let error = format!("{}", &self.error);
        let error_with_location = LastError {
            error,
            instruction: &self.instruction,
        };

        serde_json::to_string(&error_with_location).expect("default serializer shouldn't fail")
    }
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(current_peer_id: String, init_peer_id: String) -> Self {
        Self {
            current_peer_id,
            init_peer_id,
            subtree_complete: true,
            last_error_could_be_set: true,
            ..<_>::default()
        }
    }
}

use std::fmt::Display;
use std::fmt::Formatter;

impl<'i> Display for ExecutionCtx<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "data cache:")?;
        for (key, value) in self.data_cache.iter() {
            writeln!(f, "  {} => {}", key, value)?;
        }
        writeln!(f, "current peer id: {}", self.current_peer_id)?;
        writeln!(f, "subtree complete: {}", self.subtree_complete)?;
        writeln!(f, "next peer public keys: {:?}", self.next_peer_pks)?;

        Ok(())
    }
}
