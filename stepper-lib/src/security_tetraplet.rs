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
use crate::air::ResolvedTriplet;

use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;

/// Describes an origin returned corresponding value.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SecurityTetraplet {
    // describes location of the value in the network.
    pub triplet: Rc<ResolvedTriplet>,

    // json path used to obtain supplied to call_service values from the value.
    pub json_path: String,
}

impl SecurityTetraplet {
    /// Create tetraplet for string variables defined in the script such as variable_1, variable_2 here
    /// "(call ("" "") "" ["variable_1" "variable_2"])"
    pub(crate) fn initiator_tetraplet(exec_ctx: &ExecutionCtx<'_>) -> Self {
        let triplet = ResolvedTriplet {
            // these variables set by the initiator peer
            peer_pk: exec_ctx.init_peer_id.clone(),
            service_id: String::new(),
            function_name: String::new(),
        };
        let triplet = Rc::new(triplet);

        Self {
            triplet,
            // json path can't applied to the string literals
            json_path: String::new(),
        }
    }
}
