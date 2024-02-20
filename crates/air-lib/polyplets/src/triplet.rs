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

use marine_call_parameters::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

/// ResolvedTriplet represents peer network location with all
/// variables, literals and etc resolved into final string.
/// This structure contains a subset of values that
/// SecurityTetraplet consists of.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ResolvedTriplet {
    pub peer_pk: String,
    pub service_id: String,
    pub function_name: String,
}

impl From<ResolvedTriplet> for SecurityTetraplet {
    fn from(triplet: ResolvedTriplet) -> Self {
        Self {
            peer_pk: triplet.peer_pk,
            service_id: triplet.service_id,
            function_name: triplet.function_name,
            lambda: String::new(),
        }
    }
}
