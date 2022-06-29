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

use air_test_utils::avm_runner::AVMRunner;
use avm_server::CallResults;

use anyhow::Context as _;
use clap::Parser;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

pub const AQUAVM_TRACING_ENV: &str = "WASM_LOG";
const DEFAULT_DATA: &str =
    r#"{"trace":[],"streams":{},"version":"0.2.2","lcid":0,"r_streams":{"$nodes":{}}}"#;

#[derive(Parser, Debug)]
#[clap(about = "Run AIR script with AquaVM")]
pub(crate) struct Args {
    #[clap(long)]
    init_peer_id: Option<String>,
    #[clap(long)]
    current_peer_id: Option<String>,
    #[clap(long, help = "default: current time")]
    timestamp: Option<u64>,
    #[clap(long, help = "default: max possible ttl")]
    ttl: Option<u32>,
    #[clap(long = "call_results")]
    call_results_path: Option<PathBuf>,

    #[clap(long)]
    max_heap_size: Option<u64>,
    #[clap(long, default_value = "info")]
    tracing_params: String,

    #[clap(
        long = "runtime",
        env = "AIR_WASM_RUNTIME_PATH",
        default_value = "target/wasm32-wasi/release/air_interpreter_server.wasm"
    )]
    air_wasm_runtime_path: PathBuf,
    #[clap(long = "prev_data")]
    prev_data_path: Option<PathBuf>,
    #[clap(long = "data")]
    data_path: PathBuf,
    #[clap(long = "script", help = "read from stdin by default")]
    air_script_path: Option<PathBuf>,
}

pub(crate) fn run(args: Args) -> anyhow::Result<()> {
    update_tracing_env(&args.tracing_params);
    init_tracing();

    let mut runner = create_avm_runner(&args).context("Failed to instantiate AIR runner")?;

    let air_script =
        read_air_script(args.air_script_path.as_deref()).context("failed to read AIR script")?;
    let prev_data = match &args.prev_data_path {
        None => DEFAULT_DATA.to_owned(),
        Some(prev_data_path) => load_data(prev_data_path).context("failed to read prev_data")?,
    };
    let current_data = load_data(&args.data_path).context("failed to read data")?;

    let init_peer_id = args.init_peer_id.unwrap_or_else(|| "some_id".to_owned());
    let timestamp = args
        .timestamp
        .unwrap_or_else(crate::utils::unix_timestamp_now);
    let ttl = args.ttl.unwrap_or(u32::MAX);
    let call_results = read_call_results(args.call_results_path.as_deref())?;

    let results = runner
        .call(
            air_script,
            prev_data,
            current_data,
            init_peer_id,
            timestamp,
            ttl,
            call_results,
        )
        .context("Failed to execute the script")?;

    println!("{:?}", results);

    Ok(())
}

fn update_tracing_env(tracing_params: &str) {
    std::env::set_var("WASM_LOG", tracing_params);
}

fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env(AQUAVM_TRACING_ENV))
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();
}

fn load_data(data_path: &Path) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(data_path)?)
}

fn read_air_script(air_input: Option<&Path>) -> anyhow::Result<String> {
    use std::io::Read;

    let air_script = match air_input {
        Some(in_path) => std::fs::read_to_string(in_path)?,
        None => {
            let mut buffer = String::new();
            let mut stdin = std::io::stdin().lock();

            stdin.read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(air_script)
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

fn create_avm_runner(args: &Args) -> anyhow::Result<AVMRunner> {
    let current_peer_id = args
        .current_peer_id
        .clone()
        .unwrap_or_else(|| "some_id".to_owned());

    Ok(AVMRunner::new(
        args.air_wasm_runtime_path.clone(),
        current_peer_id,
        args.max_heap_size,
        0,
    )?)
}
