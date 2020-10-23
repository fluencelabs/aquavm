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

use super::fold::FoldState;
use crate::AquaData;

use std::collections::HashMap;

/// Execution context contains all necessary information needed to execute aqua script.
#[derive(Clone, Default, Debug)]
pub(crate) struct ExecutionCtx {
    /// Contains all set variables.
    pub data: AquaData,

    /// Set of peer public keys that should receive resulted data.
    pub next_peer_pks: Vec<String>,

    /// PeerId of a peer executing this aqua script.
    pub current_peer_id: String,

    /// Describes all met folds on the current execution step.
    pub folds: HashMap<String, FoldState>,

    /// Indicates that previous executed subtree is complete.
    /// A subtree treats as a complete if all subtree elements satisfy the following rules:
    ///   - at least one of par subtrees is complete
    ///   - all of seq subtrees are complete
    ///   - call executes successfully (call evidence equals to Executed)
    pub subtree_complete: bool,
}

impl ExecutionCtx {
    pub(crate) fn new(data: AquaData, current_peer_id: String) -> Self {
        Self {
            data,
            next_peer_pks: vec![],
            current_peer_id,
            folds: HashMap::new(),
            subtree_complete: true,
        }
    }
}
