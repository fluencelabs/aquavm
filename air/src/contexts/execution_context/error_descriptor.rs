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

use super::ExecutionCtx;
use super::ExecutionError;
use super::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

/// This struct is intended to track the last arisen error.
#[derive(Debug)]
pub(crate) struct LastErrorDescriptor {
    pub(crate) error: Rc<ExecutionError>,
    pub(crate) instruction: String,
    pub(crate) tetraplet: Option<SecurityTetraplet>,
}

/// This type is a serialization target for last error. It means that on the AIR script side
/// %last_error% will have such type.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LastError {
    /// text representation of an instruction that caused the last error
    pub instruction: String,

    /// text representation of an error message
    pub msg: String,
}

/// Helper struct to return last error with tetraplets from the last_error ExecutionCtx method.
#[derive(Debug, Default)]
pub(crate) struct LastErrorWithTetraplets {
    pub(crate) last_error: LastError,
    pub(crate) tetraplets: Vec<SecurityTetraplet>,
}

impl<'s> LastErrorWithTetraplets {
    pub(crate) fn from_error_descriptor(descriptor: &LastErrorDescriptor, ctx: &ExecutionCtx<'_>) -> Self {
        let last_error = descriptor.serialize();
        let tetraplets = descriptor
            .tetraplet
            .clone()
            .unwrap_or_else(|| SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.clone()));
        let tetraplets = vec![tetraplets];

        Self { last_error, tetraplets }
    }
}

impl LastErrorDescriptor {
    pub(crate) fn new(error: Rc<ExecutionError>, instruction: String, tetraplet: Option<SecurityTetraplet>) -> Self {
        Self {
            error,
            instruction,
            tetraplet,
        }
    }

    // serialize error to LastError wrapped in JValue
    pub(crate) fn serialize(&self) -> LastError {
        let error = self.error.to_string();

        LastError {
            msg: error,
            instruction: self.instruction.clone(),
        }
    }
}
