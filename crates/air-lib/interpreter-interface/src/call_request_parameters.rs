/*
 * Copyright 2021 Fluence Labs Limited
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

use marine_rs_sdk::marine;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type CallRequests = HashMap<u32, CallRequestParams>;

/// Contains arguments of a call instruction and all other necessary information
/// required for calling a service.
#[marine]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallRequestParams {
    /// Id of a service that should be called.
    pub service_id: String,

    /// Name of a function from service identified by service_id that should be called.
    pub function_name: String,

    /// Serialized to JSON string Vec<JValue> of arguments that should be passed to a service.
    pub arguments: String,

    /// Serialized to JSON string Vec<Vec<SecurityTetraplet>> that should be passed to a service.
    pub tetraplets: String,
}

impl CallRequestParams {
    pub fn new(
        service_id: String,
        function_name: String,
        arguments: String,
        tetraplets: String,
    ) -> Self {
        Self {
            service_id,
            function_name,
            arguments,
            tetraplets,
        }
    }
}
