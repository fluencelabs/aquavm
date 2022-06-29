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

mod logs;
mod run;
mod stats;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Subcomm,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Subcomm {
    Run(crate::run::Args),
    Stats(crate::stats::Args),
}

fn main() -> anyhow::Result<()> {
    let command = Cli::parse();
    match command.command {
        Subcomm::Run(args) => crate::run::run(args),
        Subcomm::Stats(args) => crate::stats::stats(args),
    }
}
