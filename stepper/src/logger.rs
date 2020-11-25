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

use log::Level as LogLevel;
use stepper_lib::log_targets::TARGET_MAP;

use std::collections::HashMap;

const RUST_LOG_ENV_NAME: &str = "RUST_LOG";
const DEFAULT_LOG_LEVEL: &str = "trace";

pub fn init_logger() {
    use std::iter::FromIterator;

    let target_map = HashMap::from_iter(TARGET_MAP.iter().cloned());
    let log_level_str = std::env::var(RUST_LOG_ENV_NAME).unwrap_or(String::from(DEFAULT_LOG_LEVEL));
    let log_level = to_log_level(&log_level_str);

    fluence::WasmLogger::new()
        .with_log_level(log_level)
        .with_target_map(target_map)
        .build()
        .unwrap();
}

fn to_log_level(raw_log_level: &String) -> LogLevel {
    use LogLevel::*;

    match raw_log_level.to_ascii_lowercase().as_str() {
        "error" => Error,
        "warn" => Warn,
        "info" => Info,
        "debug" => Debug,
        "trace" => Trace,
        _ => Trace,
    }
}
