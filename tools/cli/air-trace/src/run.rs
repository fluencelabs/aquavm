/*
 * Copyright 2022 Fluence Labs Limited
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

mod data;
mod native;
mod runner;
#[cfg(feature = "wasm")]
mod wasm;

use self::runner::AirRunner;
use air_test_utils::CallResults;

use anyhow::Context as _;
use clap::{Parser, Subcommand};

use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(about = "Run AIR script with AquaVM")]
pub(crate) struct Args {
    #[clap(long)]
    current_peer_id: Option<String>,

    #[clap(long = "call-results")]
    call_results_path: Option<PathBuf>,

    #[clap(long)]
    max_heap_size: Option<u64>,
    #[clap(long, default_value = "info")]
    tracing_params: String,
    #[clap(long)]
    native: bool,
    #[clap(
        long = "runtime",
        env = "AIR_WASM_RUNTIME_PATH",
        default_value = "target/wasm32-wasi/release/air_interpreter_server.wasm"
    )]
    air_wasm_runtime_path: PathBuf,
    #[clap(long, help = "Execute several times; great for native profiling")]
    repeat: Option<u32>,
    #[clap(long, help = "Output JSON tracing info")]
    json: bool,

    #[clap(subcommand)]
    source: Source,
}

#[derive(Subcommand, Debug)]
#[allow(clippy::large_enum_variant)]
enum Source {
    #[clap(name = "--anomaly")]
    Anomaly(self::data::anomaly::AnomalyDataArgs),
    #[clap(name = "--plain")]
    PlainData(self::data::plain::PlainDataArgs),
}

pub(crate) fn run(args: Args) -> anyhow::Result<()> {
    let tracing_json = (!args.json) as u8;
    init_tracing(args.tracing_params.clone(), tracing_json);

    let current_peer_id = args.current_peer_id.as_deref().unwrap_or("some_id");
    let mut runner = get_runner(
        args.native,
        current_peer_id,
        &args.air_wasm_runtime_path,
        args.max_heap_size,
    )?;

    let execution_data = match &args.source {
        Source::Anomaly(anomaly) => data::anomaly::load(anomaly)?,
        Source::PlainData(raw) => data::plain::load(raw)?,
    };
    let particle = execution_data.particle;

    let call_results = read_call_results(args.call_results_path.as_deref())?;

    let repeat = args.repeat.unwrap_or(1);
    for _ in 0..repeat {
        let result = runner
            .call_tracing(
                execution_data.air_script.clone(),
                execution_data.prev_data.clone().into(),
                execution_data.current_data.clone().into(),
                particle.init_peer_id.clone().into_owned(),
                particle.timestamp,
                particle.ttl,
                call_results.clone(),
                args.tracing_params.clone(),
                tracing_json,
            )
            .context("Failed to execute the script")?;
        if args.repeat.is_none() {
            println!("{:?}", result);
        }
    }

    Ok(())
}

#[cfg(feature = "wasm")]
fn get_runner(
    native: bool,
    current_peer_id: impl Into<String>,
    air_wasm_runtime_path: &Path,
    max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    if native {
        self::native::create_native_avm_runner(current_peer_id)
            .context("Failed to instantiate a native AVM")
    } else {
        self::wasm::create_wasm_avm_runner(current_peer_id, air_wasm_runtime_path, max_heap_size)
            .context("Failed to instantiate WASM AVM")
    }
}

#[cfg(not(feature = "wasm"))]
fn get_runner(
    native: bool,
    current_peer_id: impl Into<String>,
    air_wasm_runtime_path: &Path,
    max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    self::native::create_native_avm_runner(current_peer_id)
        .context("Failed to instantiate a native AVM")
}

// TODO This is a copy of function from air_interpreter/marine.rs.  It should be moved to the marine_rs_sdk.
pub fn init_tracing(tracing_params: String, trace_mode: u8) {
    use tracing_subscriber::fmt::format::FmtSpan;

    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE);
    if trace_mode == 0 {
        builder.json().init();
    } else {
        // Human-readable output.
        builder.init();
    }
}
fn read_call_results(call_results_path: Option<&Path>) -> anyhow::Result<CallResults> {
    match call_results_path {
        None => Ok(CallResults::default()),
        Some(call_results_path) => {
            let call_results_json =
                load_data(call_results_path).context("failed to read call_results")?;
            Ok(serde_json::from_str(&call_results_json)
                .context("failed to parse call_results data")?)
        }
    }
}

fn load_data(data_path: &Path) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(data_path)?)
}
