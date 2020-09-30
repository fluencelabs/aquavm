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
mod air;
mod execution;
mod instructions;
mod stepper;
mod stepper_outcome;

use crate::execution::exec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn invoke(init_user_id: String, aqua: String, data: String) -> String {
    let outcome = exec(init_user_id, aqua, data);
    serde_json::to_string(&outcome).unwrap()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(raw_module = "../src/call_service.ts")]
extern "C" {
    pub fn call_service(service_id: String, fn_name: String, args: String) -> String;
}
