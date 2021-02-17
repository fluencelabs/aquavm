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

use fluence::fce;
use interpreter_lib::execute_aqua;
use interpreter_lib::InterpreterOutcome;
use logger::DEFAULT_LOG_LEVEL;

use log::Level as LogLevel;

const WASM_LOG_ENV_NAME: &str = "WASM_LOG";

pub fn main() {
    logger::init_logger();
}

#[fce]
pub fn invoke(init_peer_id: String, aqua: String, prev_data: Vec<u8>, data: Vec<u8>) -> InterpreterOutcome {
    let log_level = get_log_level();
    log::set_max_level(log_level.to_level_filter());

    execute_aqua(init_peer_id, aqua, prev_data, data)
}

#[fce]
pub fn ast(script: String) -> String {
    ast::ast(script)
}

fn get_log_level() -> LogLevel {
    use std::str::FromStr;

    match std::env::var(WASM_LOG_ENV_NAME) {
        Ok(log_level_str) => LogLevel::from_str(&log_level_str).unwrap_or(DEFAULT_LOG_LEVEL),
        Err(_) => DEFAULT_LOG_LEVEL,
    }
}
