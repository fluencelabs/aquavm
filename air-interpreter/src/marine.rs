/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

pub fn main() {
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
    execute_air(air, prev_data, data, params, call_results.into())
}

#[allow(clippy::too_many_arguments)]
#[marine]
pub fn invoke_tracing(
    air: String,
    prev_data: Vec<u8>,
    data: Vec<u8>,
    params: RunParameters,
    call_results: Vec<u8>,
    tracing_params: String,
    tracing_output_mode: u8,
) -> InterpreterOutcome {
    use tracing::Dispatch;
    use tracing_subscriber::fmt::format::FmtSpan;

    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_writer(std::io::stderr);

    let dispatch = if logger::json_output_mode(tracing_output_mode) {
        let subscriber = builder.json().finish();
        Dispatch::new(subscriber)
    } else {
        // Human-readable output.
        let subscriber = builder.finish();
        Dispatch::new(subscriber)
    };
    tracing::dispatcher::with_default(&dispatch, || {
        execute_air(air, prev_data, data, params, call_results.into())
    })
}

#[marine]
pub fn ast(script: String) -> String {
    ast::ast(script)
}

/// Like ast, this function is intended to be run localy by tools.
#[marine]
pub fn to_human_readable_data(data: Vec<u8>) -> String {
    match air::to_human_readable_data(data) {
        Ok(text) => text,
        Err(err) => err.to_string(),
    }
}
