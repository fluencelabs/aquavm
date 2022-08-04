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

use super::ExecutionData;
use crate::utils::unix_timestamp_now;
use air_test_utils::ParticleParameters;

use anyhow::Context;

use std::path::{Path, PathBuf};

const DEFAULT_DATA: &str = "";

#[derive(clap::Args, Debug)]
pub(crate) struct PlainDataArgs {
    #[clap(long)]
    init_peer_id: Option<String>,
    #[clap(long, help = "default: current time")]
    timestamp: Option<u64>,
    #[clap(long, help = "default: max possible ttl")]
    ttl: Option<u32>,

    #[clap(long = "script", help = "read from stdin by default")]
    air_script_path: Option<PathBuf>,
    #[clap(long = "prev-data")]
    prev_data_path: Option<PathBuf>,
    #[clap(long = "data")]
    data_path: PathBuf,
}

pub(crate) fn load(args: &PlainDataArgs) -> anyhow::Result<ExecutionData> {
    use crate::run::load_data;

    let air_script =
        read_air_script(args.air_script_path.as_deref()).context("failed to read AIR script")?;
    let prev_data = match &args.prev_data_path {
        None => DEFAULT_DATA.to_owned(),
        Some(prev_data_path) => load_data(prev_data_path).context("failed to read prev_data")?,
    };
    let current_data = load_data(&args.data_path).context("failed to read data")?;

    let timestamp = args.timestamp.unwrap_or_else(unix_timestamp_now);
    let ttl = args.ttl.unwrap_or(u32::MAX);
    let init_peer_id = args.init_peer_id.as_deref().unwrap_or("some_id");

    let particle = ParticleParameters::new(init_peer_id.into(), "".into(), timestamp, ttl);
    Ok(ExecutionData {
        air_script,
        prev_data,
        current_data,
        particle,
    })
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
