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

use super::super::super::utils::unix_timestamp_now;
use super::ExecutionData;
use crate::trace::run::runner::TestInitParameters;

use avm_interface::ParticleParameters;

use eyre::Context;

use std::path::{Path, PathBuf};

const DEFAULT_DATA: &[u8] = b"";

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
    #[clap(long = "current-data")]
    current_data_path: Option<PathBuf>,

    #[clap(long = "particle-id")]
    particle_id: Option<String>,

    #[clap(long = "air-size-limit")]
    air_size_limit: Option<u64>,

    #[clap(long = "particle-size-limit")]
    particle_size_limit: Option<u64>,

    #[clap(long = "call-result-size-limit")]
    call_result_size_limit: Option<u64>,

    #[clap(long = "hard-limit-enabled", default_value = "false")]
    hard_limit_enabled: bool,
}

pub(crate) fn load(args: &PlainDataArgs) -> eyre::Result<ExecutionData<'_>> {
    use super::super::load_data_or_default;

    let air_script = read_air_with_prompt(args.air_script_path.as_deref())
        .context("failed to read AIR script")?;
    let prev_data = load_data_or_default(args.prev_data_path.as_ref(), DEFAULT_DATA)
        .context("failed to read prev_data")?;
    let current_data = load_data_or_default(args.current_data_path.as_ref(), DEFAULT_DATA)
        .context("failed to read data")?;

    let timestamp = args.timestamp.unwrap_or_else(unix_timestamp_now);
    let ttl = args.ttl.unwrap_or(u32::MAX);
    let init_peer_id = &args.init_peer_id;
    let current_peer_id = &args.current_peer_id;

    let particle = ParticleParameters::new(
        init_peer_id.into(),
        args.particle_id.as_deref().unwrap_or_default().into(),
        timestamp,
        ttl,
        current_peer_id.into(),
    );

    let test_init_parameters = TestInitParameters::new(
        args.air_size_limit,
        args.particle_size_limit,
        args.call_result_size_limit,
        args.hard_limit_enabled,
    );

    Ok(ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
        test_init_parameters,
    })
}

fn read_air_with_prompt(air_input: Option<&Path>) -> eyre::Result<String> {
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
