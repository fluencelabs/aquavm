/*
 * Copyright 2023 Fluence Labs Limited
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

mod beautify;
mod trace;
#[cfg(feature = "near")]
mod near;

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
    Beautify(crate::beautify::Args),
    #[cfg(feature = "near")]
    #[clap(alias = "n")]
    Near(crate::near::Args),
    #[clap(alias = "r")]
    Run(crate::trace::run::Args),
    #[clap(alias = "s")]
    Stats(crate::trace::stats::Args),
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.subcommand {
        Subcommand::Beautify(args) => crate::beautify::beautify(args),
        #[cfg(feature = "near")]
        Subcommand::Near(args) => crate::near::near(args),
        Subcommand::Run(args) => crate::trace::run::run(args),
        Subcommand::Stats(args) => crate::trace::stats::stats(args),
    }
}
