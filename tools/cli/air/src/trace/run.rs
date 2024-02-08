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
pub(crate) mod native;
#[cfg(feature = "near")]
mod near;
#[cfg(feature = "risc0")]
mod risc0;
#[cfg(feature = "wasm")]
pub(crate) mod wasm;

pub(crate) mod runner;

use self::runner::AirRunner;
use avm_interface::CallResults;

use anyhow::Context as _;
use clap::Parser;
use clap::Subcommand;
use fluence_keypair::KeyPair;
use zeroize::Zeroize;

use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about = "Run AIR script with AquaVM")]
pub(crate) struct Args {
    #[clap(long = "call-results")]
    call_results_path: Option<PathBuf>,

    #[clap(long)]
    max_heap_size: Option<u64>,
    #[clap(long, default_value = "info")]
    tracing_params: String,
    #[clap(long, default_value = "warn")]
    runner_tracing_params: String,

    #[clap(flatten)]
    mode: ModeArgs,

    #[clap(
        long = "interpreter",
        env = "AIR_INTERPRETER_WASM_PATH",
        default_value = "target/wasm32-wasi/release/air_interpreter_server.wasm"
    )]
    air_interpreter_path: PathBuf,

    #[clap(
        long = "near-contract",
        env = "AIR_NEAR_CONTRACT_PATH",
        default_value = "tools/wasm/air-near-contract/target/wasm32-unknown-unknown/release/air-near-contract.wasm"
    )]
    air_near_contract_path: PathBuf,

    #[clap(long, help = "Execute several times; great for native profiling")]
    repeat: Option<u32>,
    #[clap(long, help = "Output JSON tracing info")]
    json: bool,

    #[clap(long = "no-fail", help = "Do not fail if AquaVM returns error")]
    no_fail: bool,

    #[command(flatten)]
    keys: Keys,

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

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct Keys {
    #[arg(long)]
    random_key: bool,
    #[arg(long)]
    ed25519_key: Option<PathBuf>,
}

impl Keys {
    fn get_keypair(&self) -> anyhow::Result<KeyPair> {
        match (self.random_key, self.ed25519_key.as_ref()) {
            (true, None) => Ok(KeyPair::generate_ed25519()),
            (false, Some(path)) => load_keypair_ed25519(path),
            _ => unreachable!("clap should allow to provide one and only one key option"),
        }
    }
}

#[derive(clap::Args, Debug, Copy, Clone)]
#[group(multiple = false)]
struct ModeArgs {
    #[arg(long)]
    native: bool,

    #[cfg(feature = "wasm")]
    #[arg(long)]
    wasm: bool,

    #[cfg(feature = "near")]
    #[arg(long)]
    near: bool,

    #[cfg(feature = "risc0")]
    #[arg(long)]
    risc0: bool,
}

impl From<ModeArgs> for Option<Mode> {
    fn from(value: ModeArgs) -> Self {
        if value.native {
            return Some(Mode::Native);
        }

        #[cfg(feature = "wasm")]
        if value.wasm {
            return Some(Mode::Wasm);
        }

        #[cfg(feature = "near")]
        if value.near {
            return Some(Mode::Near);
        }

        #[cfg(feature = "risc0")]
        if value.risc0 {
            return Some(Mode::Risc0);
        }

        None
    }
}

enum Mode {
    Native,

    #[cfg(feature = "wasm")]
    Wasm,

    #[cfg(feature = "near")]
    Near,

    #[cfg(feature = "risc0")]
    Risc0,
}

pub(crate) async fn run(args: Args) -> anyhow::Result<()> {
    let tracing_json = (!args.json) as u8;
    #[cfg(feature = "wasm")]
    let global_tracing_params = if args.mode.wasm {
        // for native and other, there is single tracing configuration, and no runner
        args.tracing_params.clone()
    } else {
        args.runner_tracing_params
    };
    #[cfg(not(feature = "wasm"))]
    let global_tracing_params = args.tracing_params.clone();
    init_tracing(global_tracing_params, tracing_json);

    let mut runner = create_runner(
        args.mode.into(),
        &args.air_interpreter_path,
        &args.air_near_contract_path,
        args.max_heap_size,
    ).await?;

    let execution_data = match &args.source {
        Source::Anomaly(anomaly) => data::anomaly::load(anomaly)?,
        Source::PlainData(raw) => data::plain::load(raw)?,
    };
    let particle = execution_data.particle;

    let call_results = read_call_results(args.call_results_path.as_deref())?;

    let key_pair = args
        .keys
        .get_keypair()
        .context("failed to get the keypair")?;

    let repeat = args.repeat.unwrap_or(1);
    for _ in 0..repeat {
        let result = runner
            .call_tracing(
                execution_data.air_script.clone(),
                execution_data.prev_data.clone(),
                execution_data.current_data.clone(),
                particle.init_peer_id.clone().into_owned(),
                particle.timestamp,
                particle.ttl,
                particle.current_peer_id.clone().into(),
                call_results.clone(),
                args.tracing_params.clone(),
                tracing_json,
                &key_pair,
                particle.particle_id.clone().into_owned(),
            )
            .await
            .context("Failed to execute the script")?;
        if args.repeat.is_none() {
            println!("{result:?}");
        }
        if !args.no_fail && (result.ret_code != 0) {
            std::process::exit(2);
        }
    }

    Ok(())
}

async fn create_runner(
    mode: Option<Mode>,
    _air_interpreter_wasm_path: &Path,
    _air_contract_wasm_path: &Path,
    _max_heap_size: Option<u64>,
) -> anyhow::Result<Box<dyn AirRunner>> {
    #[cfg(not(feature = "wasm"))]
    let default_mode = Mode::Native;
    #[cfg(feature = "wasm")]
    let default_mode = Mode::Wasm;

    let mode = mode.unwrap_or(default_mode);
    let runner = match mode {
        Mode::Native => self::native::create_native_avm_runner()
            .context("Failed to instantiate a native AVM")? as _,
        #[cfg(feature = "wasm")]
        Mode::Wasm => {
            self::wasm::create_wasm_avm_runner(_air_interpreter_wasm_path, _max_heap_size)
                .await
                .context("Failed to instantiate WASM AVM")? as _
        }
        #[cfg(feature = "near")]
        Mode::Near => self::near::create_near_runner(_air_contract_wasm_path)
            .context("Failed to instantiate NEAR AVM")?,
        #[cfg(feature = "risc0")]
        Mode::Risc0 => Box::new(self::risc0::Risc0Runner::new()),
    };
    Ok(runner)
}

// TODO This is a copy of function from air_interpreter/marine.rs.  It should be moved to the marine_rs_sdk.
pub fn init_tracing(tracing_params: String, trace_mode: u8) {
    use tracing_subscriber::fmt::format::FmtSpan;

    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_writer(std::io::stderr);
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
            // call resuls are may be manually crafted, so JSON representation
            // of avm_interface::CallResults is more user-friendly
            Ok(serde_json::from_slice(&call_results_json)
                .context("failed to parse call_results data")?)
        }
    }
}

fn load_data_or_default(
    data_path: Option<impl AsRef<Path>>,
    default: &[u8],
) -> anyhow::Result<Vec<u8>> {
    match data_path {
        None => Ok(default.to_owned()),
        Some(data_path) => load_data(data_path.as_ref()),
    }
}

pub(crate) fn load_data(data_path: &Path) -> anyhow::Result<Vec<u8>> {
    std::fs::read(data_path).with_context(|| data_path.to_string_lossy().into_owned())
}

fn load_keypair_ed25519(path: &PathBuf) -> Result<KeyPair, anyhow::Error> {
    use fluence_keypair::KeyFormat;

    // It follows rust-peer format
    let mut file = std::fs::File::open(path)?;

    let mut file_content = String::with_capacity(
        file.metadata()?
            .len()
            .try_into()
            .context("failed to convert file length")?,
    );
    file.read_to_string(&mut file_content)?;

    let key_data = bs58::decode(file_content.trim())
        .into_vec()
        .context("failed to decode the base58 key material")?;
    file_content.zeroize();

    KeyPair::from_vec(key_data, KeyFormat::Ed25519).context("malformed key data")
}
