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

#[marine]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallRequestParams {
    pub service_name: String,
    pub function_name: String,
    pub arguments: String,
    pub tetraplets: String,
}

impl CallRequestParams {
    pub fn new(
        service_name: String,
        function_name: String,
        arguments: String,
        tetraplets: String,
    ) -> Self {
        Self {
            service_name,
            function_name,
            arguments,
            tetraplets,
        }
    }
}
