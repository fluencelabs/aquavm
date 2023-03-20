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

use super::super::super::utils::unix_timestamp_now;
use super::ExecutionData;
use avm_interface::ParticleParameters;

use anyhow::Context;

use std::path::{Path, PathBuf};

const DEFAULT_DATA: &str = "";

#[derive(clap::Args, Debug)]
pub(crate) struct PlainDataArgs {
    #[clap(long, default_value = "some_id")]
    init_peer_id: String,
    #[clap(long, help = "default: current time")]
    timestamp: Option<u64>,
    #[clap(long, help = "default: max possible ttl")]
    ttl: Option<u32>,
    #[clap(long, default_value = "some_id")]
    current_peer_id: String,

    #[clap(long = "script", help = "read from stdin by default")]
    air_script_path: Option<PathBuf>,
    #[clap(long = "prev-data")]
    prev_data_path: Option<PathBuf>,
    #[clap(long = "data")]
    data_path: PathBuf,
}

pub(crate) fn load(args: &PlainDataArgs) -> anyhow::Result<ExecutionData<'_>> {
    use super::super::load_data;

    let air_script = read_air_with_prompt(args.air_script_path.as_deref())
        .context("failed to read AIR script")?;
    let prev_data = match &args.prev_data_path {
        None => DEFAULT_DATA.to_owned(),
        Some(prev_data_path) => load_data(prev_data_path).context("failed to read prev_data")?,
    };
    let current_data = load_data(&args.data_path).context("failed to read data")?;

    let timestamp = args.timestamp.unwrap_or_else(unix_timestamp_now);
    let ttl = args.ttl.unwrap_or(u32::MAX);
    let init_peer_id = &args.init_peer_id;
    let current_peer_id = &args.current_peer_id;

    let particle = ParticleParameters::new(
        init_peer_id.into(),
        "".into(),
        timestamp,
        ttl,
        current_peer_id.into(),
    );

    Ok(ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
    })
}

fn read_air_with_prompt(air_input: Option<&Path>) -> anyhow::Result<String> {
    use std::io::Read;

    let air_script = match air_input {
        Some(in_path) => std::fs::read_to_string(in_path)?,
        None => {
            let mut buffer = String::new();
            let mut stdin = std::io::stdin().lock();

            // unfortunately, it seems to always return false in WASM mode
            if atty::is(atty::Stream::Stdin) {
                print_air_prompt();
            }

            stdin.read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(air_script)
}

fn print_air_prompt() {
    use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor as _};

    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let mut bold = ColorSpec::new();
    bold.set_bold(true);

    let _ = stderr.set_color(&bold);
    eprintln!("Reading AIR script from stdin...");
    let _ = stderr.set_color(&ColorSpec::new());
}
