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
use air::RunParameters;

use log::LevelFilter;
use wasm_bindgen::prelude::*;

pub const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Trace;

#[wasm_bindgen(start)]
pub fn main() {
    // it's necessary to initialize it with the minimal allowed log level,
    // because otherwise it's impossible to set less level than used during initialization.

    // const MINIMAL_LOG_LEVEL: LevelFilter = LevelFilter::Trace;
    // logger::init_logger(Some(MINIMAL_LOG_LEVEL));

    // this one is just a guard for possible changes of the invoke function where some log-prone
    // code could added before the setting max log level from a function parameter.
    log::set_max_level(LevelFilter::Info);
}

#[wasm_bindgen]
pub fn invoke(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: Vec<u8>,
    call_results: Vec<u8>,
    log_level: &str,
) -> String {
    use std::str::FromStr;

    let log_level = log::LevelFilter::from_str(log_level).unwrap_or(DEFAULT_LOG_LEVEL);
    log::set_max_level(log_level);

    let params: RunParameters = serde_json::from_slice(&params).expect("cannot parse RunParameters");

    let outcome = execute_air(air, prev_data, data, params, call_results);
    serde_json::to_string(&outcome).expect("Cannot parse InterpreterOutcome")
}

#[wasm_bindgen]
pub fn ast(script: String) -> String {
    ast::ast(script)
}
