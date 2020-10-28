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

#[cfg(feature = "fce")]
mod fce;
#[cfg(not(feature = "fce"))]
mod wasm_bindgen;

use serde_derive::Deserialize;
use serde_derive::Serialize;

pub const CALL_SERVICE_SUCCESS: i32 = 0;

#[fluence::fce]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallServiceResult {
    pub ret_code: i32,
    pub result: String,
}

#[cfg(feature = "fce")]
pub(crate) use fce::call_service;
#[cfg(feature = "fce")]
pub(crate) use fce::get_current_peer_id;

#[cfg(not(feature = "fce"))]
pub(crate) use wasm_bindgen::call_service;
#[cfg(not(feature = "fce"))]
pub(crate) use wasm_bindgen::get_current_peer_id;
