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

use wasm_bindgen::__rt::std::env::VarError;
use wasm_bindgen::prelude::*;

pub(crate) fn call_service(service_id: &str, fn_name: &str, args: &str, security_tetraplets: &str, call_id: u32) {
    call_service_impl(service_id, fn_name, args, security_tetraplets, call_id);
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn get_current_peer_id() -> std::result::Result<String, VarError> {
    Ok(get_current_peer_id_impl())
}

#[wasm_bindgen]
extern "C" {
    #[allow(unused_attributes)]
    #[link_name = "get_current_peer_id"]
    fn get_current_peer_id_impl() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[allow(unused_attributes)]
    #[link_name = "call_service"]
    fn call_service_impl(
        service_id: &str,
        fn_name: &str,
        args: &str,
        security_tetraplets: &str,
        call_id: u32,
    ) -> String;
}
