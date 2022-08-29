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

use marine_macro::marine;
use serde::Deserialize;
use serde::Serialize;

/// Describes an origin that set corresponding value.
#[marine]
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SecurityTetraplet {
    /// Id of a peer where corresponding value was set.
    pub peer_pk: String,

    /// Id of a service that set corresponding value.
    pub service_id: String,

    /// Name of a function that returned corresponding value.
    pub function_name: String,

    /// Value was produced by applying this `json_path` to the output from `call_service`.
    // TODO: since it's not a json path anymore, it's needed to rename it to lambda
    pub json_path: String,
}

impl SecurityTetraplet {
    pub fn new(
        peer_pk: impl Into<String>,
        service_id: impl Into<String>,
        function_name: impl Into<String>,
        json_path: impl Into<String>,
    ) -> Self {
        Self {
            peer_pk: peer_pk.into(),
            service_id: service_id.into(),
            function_name: function_name.into(),
            json_path: json_path.into(),
        }
    }

    /// Create a tetraplet for string literals defined in the script
    /// such as variable here `(call ("" "") "" ["variable_1"])`.
    pub fn literal_tetraplet(init_peer_id: impl Into<String>) -> Self {
        Self {
            // these variables represent the initiator peer
            peer_pk: init_peer_id.into(),
            service_id: String::new(),
            function_name: String::new(),
            // json path can't be applied to the string literals
            json_path: String::new(),
        }
    }

    pub fn from_triplet(triplet: ResolvedTriplet) -> Self {
        Self {
            peer_pk: triplet.peer_pk,
            service_id: triplet.service_id,
            function_name: triplet.function_name,
            json_path: String::new(),
        }
    }

    pub fn add_lambda(&mut self, json_path: &str) {
        self.json_path.push_str(json_path)
    }
}

use std::fmt;

impl fmt::Display for SecurityTetraplet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "peer_pk: {}, service_id: {}, function_name: {}, json_path: {}",
            self.peer_pk, self.service_id, self.function_name, self.json_path
        )
    }
}
