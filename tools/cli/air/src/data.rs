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

use clap::Parser;
use eyre::Context;
use std::path::Path;
use std::path::PathBuf;

use crate::trace::run::load_data;
use crate::trace::run::runner::DataToHumanReadable;
use crate::trace::run::runner::TestInitParameters;

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

pub(crate) async fn to_human_readable_data(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    init_tracing("warn");

    let data: Vec<u8> = load_data(&args.input)?;

    if data.is_empty() {
        Err(eyre::eyre!("empty input data: {:?}", args.input))?;
    }

    let mut runner =
        create_runner(args.mode.into(), &args.air_interpreter_path, <_>::default()).await?;
    let out = { runner.to_human_readable(data).await? };
    println!("{out}");

    Ok(())
}

fn init_tracing(tracing_params: &str) {
    let builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_params)
        .with_writer(std::io::stderr);
    builder.init();
}

async fn create_runner(
    mode: Option<Mode>,
    _air_interpreter_wasm_path: &Path,
    _test_init_parameters: TestInitParameters,
) -> eyre::Result<Box<dyn DataToHumanReadable>> {
    #[cfg(not(feature = "wasm"))]
    let default_mode = Mode::Native;
    #[cfg(feature = "wasm")]
    let default_mode = Mode::Wasm;

    let mode = mode.unwrap_or(default_mode);
    let runner = match mode {
        Mode::Native => crate::trace::run::native::create_native_avm_runner(_test_init_parameters)
            .context("Failed to instantiate a native AVM")? as _,
        #[cfg(feature = "wasm")]
        Mode::Wasm => crate::trace::run::wasm::create_wasm_avm_runner(
            _air_interpreter_wasm_path,
            None,
            <_>::default(),
        )
        .await
        .context("Failed to instantiate WASM AVM")? as _,
    };

    Ok(runner)
}
