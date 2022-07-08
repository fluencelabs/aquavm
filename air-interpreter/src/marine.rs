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

#![allow(improper_ctypes)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod ast;
mod logger;

use air::execute_air;
use air::InterpreterOutcome;
use air::RunParameters;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

/*
   _initialize function that calls __wasm_call_ctors is required to mitigate memory leak
   that is described in https://github.com/WebAssembly/wasi-libc/issues/298.

   In short, without this code rust wraps every export function
   with __wasm_call_ctors/__wasm_call_dtors calls. This causes memory leaks. When compiler sees
   an explicit call to __wasm_call_ctors in _initialize function, it disables export wrapping.

   TODO: remove when updating to marine-rs-sdk with fix
*/
extern "C" {
    fn __wasm_call_ctors();
}

#[no_mangle]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}

pub fn main() {
    _initialize(); // As __wasm_call_ctors still does necessary work, we call it at the start of the module.
    logger::init_logger(None);
}

#[marine]
pub fn invoke(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: RunParameters,
    call_results: Vec<u8>,
) -> InterpreterOutcome {
    execute_air(air, prev_data, data, params, call_results)
}

#[marine]
pub fn ast(script: String) -> String {
    ast::ast(script)
}
