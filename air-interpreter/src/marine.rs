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
use marine_rs_sdk::marine;

pub fn main() {
    logger::init_logger(None);
}

#[marine]
pub fn invoke(init_peer_id: String, air: String, prev_data: Vec<u8>, data: Vec<u8>) -> InterpreterOutcome {
    execute_air(init_peer_id, air, prev_data, data)
}

#[marine]
pub fn ast(script: String) -> String {
    ast::ast(script)
}
