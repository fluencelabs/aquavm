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

#[cfg(feature = "marine")]
mod marine_target;
#[cfg(not(feature = "marine"))]
mod wasm_bindgen_target;

use serde::Deserialize;
use serde::Serialize;

pub const CALL_SERVICE_SUCCESS: i32 = 0;

#[marine_rs_sdk::marine]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

impl std::fmt::Display for CallServiceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ret_code: {}, result: '{}'", self.ret_code, self.result)
    }
}

#[cfg(feature = "marine")]
pub(crate) use marine_target::call_service;
#[cfg(feature = "marine")]
pub(crate) use marine_target::get_current_peer_id;

#[cfg(not(feature = "marine"))]
pub(crate) use wasm_bindgen_target::call_service;
#[cfg(not(feature = "marine"))]
pub(crate) use wasm_bindgen_target::get_current_peer_id;
