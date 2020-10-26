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

#![allow(unused_attributes)]
#![warn(rust_2018_idioms)]
#![deny(
    // dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    // unused_unsafe,
    unreachable_patterns
)]

mod air;
mod call_evidence;
mod defines;
mod errors;
mod execution;
mod stepper_outcome;

pub(crate) use crate::defines::*;

use crate::execution::execute_aqua;

use wasm_bindgen::__rt::std::env::VarError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    fluence::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[wasm_bindgen]
pub fn invoke(init_user_id: String, aqua: String, prev_data: String, data: String) -> String {
    let outcome = execute_aqua(init_user_id, aqua, prev_data, data);
    serde_json::to_string(&outcome).expect("Cannot parse StepperOutcome")
}

pub fn call_service(service_id: String, fn_name: String, args: String) -> CallServiceResult {
    let result = call_service_impl(service_id, fn_name, args);
    log::info!("result {}", result);
    serde_json::from_str(&result).expect("Cannot parse CallServiceResult")
}

pub fn get_current_peer_id() -> std::result::Result<String, VarError> {
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
    fn call_service_impl(service_id: String, fn_name: String, args: String) -> String;
}
