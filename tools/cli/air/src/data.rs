/*
 * Copyright 2024 Fluence Labs Limited
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

use anyhow::Context;
use clap::Parser;
use std::path::Path;
use std::path::PathBuf;

use crate::trace::run::load_data;
use crate::trace::run::runner::DataToHumanReadable;

#[derive(clap::Args, Debug, Copy, Clone)]
#[group(multiple = false)]
struct ModeArgs {
    #[arg(long)]
    native: bool,

    #[cfg(feature = "wasm")]
    #[arg(long)]
    wasm: bool,
}

enum Mode {
    Native,

    #[cfg(feature = "wasm")]
    Wasm,
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

        None
    }
}

#[derive(Parser)]
#[clap(about = "Print human-readable AquaVM data")]
pub(crate) struct Args {
    #[clap(
        long = "interpreter",
        env = "AIR_INTERPRETER_WASM_PATH",
        default_value = "target/wasm32-wasi/release/air_interpreter_server.wasm"
    )]
    air_interpreter_path: PathBuf,

    #[clap(flatten)]
    mode: ModeArgs,
    // TODO be able to read from stdin
    #[arg(help = "Input path")]
    input: PathBuf,
}

pub(crate) fn to_human_readable_data(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    init_tracing("warn");

    let data: Vec<u8> = load_data(&args.input)?;

    if data.is_empty() {
        Err(anyhow::anyhow!("empty input data: {:?}", args.input))?;
    }

    let mut runner = create_runner(args.mode.into(), &args.air_interpreter_path)?;
    let out = runner.to_human_readable(data)?;
    println!("{out}");

    Ok(())
}

fn init_tracing(tracing_params: &str) {
    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_writer(std::io::stderr);
    builder.init();
}

fn create_runner(
    mode: Option<Mode>,
    _air_interpreter_wasm_path: &Path,
) -> anyhow::Result<Box<dyn DataToHumanReadable>> {
    #[cfg(not(feature = "wasm"))]
    let default_mode = Mode::Native;
    #[cfg(feature = "wasm")]
    let default_mode = Mode::Wasm;

    let mode = mode.unwrap_or(default_mode);
    let runner = match mode {
        Mode::Native => crate::trace::run::native::create_native_avm_runner()
            .context("Failed to instantiate a native AVM")? as _,
        #[cfg(feature = "wasm")]
        Mode::Wasm => {
            crate::trace::run::wasm::create_wasm_avm_runner(_air_interpreter_wasm_path, None)
                .context("Failed to instantiate WASM AVM")? as _
        }
    };

    Ok(runner)
}
