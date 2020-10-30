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

use super::CallServiceResult;
use std::env::VarError;

const CURRENT_PEER_ID_ENV_NAME: &str = "CURRENT_PEER_ID";

pub fn get_current_peer_id() -> std::result::Result<String, VarError> {
    std::env::var(CURRENT_PEER_ID_ENV_NAME)
}

#[fluence::fce]
#[link(wasm_import_module = "host")]
extern "C" {
    pub(crate) fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult;
}
