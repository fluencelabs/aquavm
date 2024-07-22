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

#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
// risc0 feature flag is commented out, but the code remains for future experiments
#![allow(unexpected_cfgs)]

mod beautify;
mod data;
mod trace;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Subcommand {
    #[clap(alias = "b")]
    Beautify(self::beautify::Args),
    #[clap(alias = "d")]
    Data(self::data::Args),
    #[clap(alias = "r")]
    Run(self::trace::run::Args),
    #[clap(alias = "s")]
    Stats(self::trace::stats::Args),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.subcommand {
        Subcommand::Beautify(args) => self::beautify::beautify(args)?,
        Subcommand::Data(args) => self::data::to_human_readable_data(args).await?,
        Subcommand::Run(args) => self::trace::run::run(args).await?,
        Subcommand::Stats(args) => self::trace::stats::stats(args)?,
    }
    Ok(())
}
