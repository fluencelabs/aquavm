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

use super::CallEvidencePath;

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// Encapsulates all necessary state regarding to the call pathes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CallEvidenceCtx {
    /// Contains path (serialized tree of states) after merging current and previous data,
    /// stepper used it to realize which instructions've been already executed.
    pub(crate) current_path: CallEvidencePath,

    /// Size of a current considered subtree inside current path.
    pub(crate) current_subtree_size: usize,

    // TODO: consider change it to Vec for optimization
    /// Accumulator for resulted path produced by the stepper after execution.
    pub(crate) new_path: CallEvidencePath,
}

impl CallEvidenceCtx {
    pub fn new(current_path: CallEvidencePath) -> Self {
        let current_subtree_size = current_path.len();
        Self {
            current_path,
            current_subtree_size,
            new_path: CallEvidencePath::new(),
        }
    }
}

impl Display for CallEvidenceCtx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "current path:\n{:?}", self.current_path)?;
        writeln!(f, "current subtree elements count:\n{:?}", self.current_subtree_size)?;
        writeln!(f, "new path:\n{:?}", self.new_path)
    }
}
