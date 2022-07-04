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

use air_test_utils::CallResults;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

pub const AQUAVM_TRACING_ENV: &str = "WASM_LOG";

#[derive(Parser, Debug)]
#[clap(about = "Run AIR script with AquaVM")]
pub(crate) struct Args {
    #[clap(long)]
    current_peer_id: Option<String>,

    #[clap(long = "call_results")]
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
    #[clap(long, help = "Execute several times; great for native profilng")]
    repeat: Option<u32>,

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
    update_tracing_env(&args.tracing_params);
    init_tracing();

    let current_peer_id = args.current_peer_id.as_deref().unwrap_or("some_id");
    let mut runner = if cfg!(not(feature = "wasm")) || args.native {
        self::native::create_native_avm_runner(current_peer_id)
            .context("Failed to instantiate a native AVM")?
    } else {
        #[cfg(feature = "wasm")]
        let _res = self::wasm::create_wasm_avm_runner(
            current_peer_id,
            &args.air_wasm_runtime_path,
            args.max_heap_size,
        )
        .context("Failed to instantiate WASM AVM")?;
        #[cfg(not(feature = "wasm"))]
        let _res = unreachable!();

        #[allow(unreachable_code)]
        _res
    };

    let execution_data = match &args.source {
        Source::Anomaly(anomaly) => data::anomaly::load(anomaly)?,
        Source::PlainData(raw) => data::plain::load(raw)?,
    };
    let particle = execution_data.particle;

    let call_results = read_call_results(args.call_results_path.as_deref())?;

    if let Some(repeat) = args.repeat {
        for _ in 0..repeat {
            runner
                .call(
                    execution_data.air_script.clone(),
                    execution_data.prev_data.clone().into(),
                    execution_data.current_data.clone().into(),
                    particle.init_peer_id.clone().into_owned(),
                    particle.timestamp,
                    particle.ttl,
                    call_results.clone(),
                )
                .context("Failed to execute the script")?;
        }
    } else {
        let results = runner
            .call(
                execution_data.air_script,
                execution_data.prev_data.into(),
                execution_data.current_data.into(),
                particle.init_peer_id.into_owned(),
                particle.timestamp,
                particle.ttl,
                call_results,
            )
            .context("Failed to execute the script")?;
        println!("{:?}", results);
    }

    Ok(())
}

fn update_tracing_env(tracing_params: &str) {
    std::env::set_var("WASM_LOG", tracing_params);
}

fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env(AQUAVM_TRACING_ENV))
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();
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
