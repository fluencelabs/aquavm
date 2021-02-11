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

use crate::ResolvedTriplet;

use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;

/// Describes an origin returned corresponding value.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SecurityTetraplet {
    /// Describes origin of the value in the network.
    #[serde(flatten)]
    pub triplet: Rc<ResolvedTriplet>,

    /// Value was produced by applying this `json_path` to the output from `call_service`.
    pub json_path: String,
}

impl SecurityTetraplet {
    /// Create a tetraplet for string literals defined in the script
    /// such as variable here `(call ("" "") "" ["variable_1"])`.
    pub fn literal_tetraplet(init_peer_id: String) -> Self {
        let triplet = ResolvedTriplet {
            // these variables represent the initiator peer
            peer_pk: init_peer_id,
            service_id: String::new(),
            function_name: String::new(),
        };
        let triplet = Rc::new(triplet);

        Self {
            triplet,
            // json path can't be applied to the string literals
            json_path: String::new(),
        }
    }

    pub fn from_triplet(triplet: Rc<ResolvedTriplet>) -> Self {
        Self {
            triplet,
            json_path: String::new(),
        }
    }
}
