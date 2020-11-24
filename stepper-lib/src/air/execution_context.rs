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

use crate::AValue;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;

/// Execution context contains all necessary information needed to execute aqua script.
// #[derive(Clone, Default, Debug)]
pub(crate) struct ExecutionCtx<'i> {
    /// Contains all set variables.
    pub data_cache: HashMap<String, AValue<'i>>,

    /// Set of peer public keys that should receive resulted data.
    pub next_peer_pks: Vec<String>,

    /// PeerId of a peer executing this aqua script at the moment.
    pub current_peer_id: String,

    /// PeerId of a peer send this aqua script.
    pub init_peer_id: String,

    /// Indicates that previous executed subtree is complete.
    /// A subtree treats as a complete if all subtree elements satisfy the following rules:
    ///   - at least one of par subtrees is complete
    ///   - non-thrown subtree of xor is complete
    ///   - all of seq subtrees are complete
    ///   - call executes successfully (call evidence equals to Executed)
    pub subtree_complete: bool,

    /// List of met folds used to determine whether a variable can be shadowed.
    pub met_folds: VecDeque<&'i str>,
}

impl<'i> ExecutionCtx<'i> {
    pub(crate) fn new(current_peer_id: String, init_peer_id: String) -> Self {
        Self {
            data_cache: HashMap::new(),
            next_peer_pks: vec![],
            current_peer_id,
            init_peer_id,
            subtree_complete: true,
            met_folds: VecDeque::new(),
        }
    }
}

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
