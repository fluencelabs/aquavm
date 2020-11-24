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

use crate::air::ExecutionCtx;

use fluence::fce;

use serde::Deserialize;
use serde::Serialize;

/// Describes an origin returns corresponding value.
#[fce]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecurityTetraplet {
    pub pub_key: String,
    pub service_id: String,
    pub function_name: String,
    pub json_path: String,
}

impl SecurityTetraplet {
    // This one is used for creating tetraplet for host identified by init peer id.
    pub(crate) fn initiator_tetraplet(exec_ctx: &ExecutionCtx<'_>) -> Self {
        Self {
            pub_key: exec_ctx.init_peer_id.clone(),
            service_id: String::new(),
            function_name: String::new(),
            json_path: String::new(),
        }
    }
}
