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

use crate::SecurityTetraplet;

use wasm_bindgen::__rt::std::env::VarError;
use wasm_bindgen::prelude::*;

pub(crate) fn call_service(
    service_id: String,
    fn_name: String,
    args: String,
    security_tetraplets: Vec<Vec<SecurityTetraplet>>,
) -> super::CallServiceResult {
    let security_tetraplets = serde_json::to_string(&security_tetraplets).expect("can't serialize tetraplets");
    let result = call_service_impl(service_id, fn_name, args, security_tetraplets);

    log::trace!("result {}", result);

    serde_json::from_str(&result).expect("Cannot parse CallServiceResult")
}

pub(crate) fn get_current_peer_id() -> std::result::Result<String, VarError> {
    Ok(get_current_peer_id_impl())
}

#[wasm_bindgen]
extern "C" {
    #[link_name = "get_current_peer_id"]
    fn get_current_peer_id_impl() -> String;
}

#[wasm_bindgen(raw_module = "../src/call_service.ts")]
extern "C" {
    #[link_name = "call_service"]
    fn call_service_impl(service_id: String, fn_name: String, args: String, security_tetraplets: String) -> String;
}
