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

#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

use air_beautifier::Beautifier;
use anyhow::{Context, Result};
use clap::Parser;

use std::{io, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = air_beautifier::DEFAULT_INDENT_SIZE)]
    indent_size: usize,
    #[clap(short, long)]
    output: Option<PathBuf>,
    input: Option<PathBuf>,
}

fn read_script(args: &Args) -> Result<String> {
    use std::io::Read;

    let air_script = match &args.input {
        Some(in_path) => std::fs::read_to_string(in_path)?,
        None => {
            let mut buffer = String::new();
            let mut stdin = io::stdin().lock();

            stdin.read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(air_script)
}

fn build_output(args: &Args) -> Result<Box<dyn io::Write>> {
    let output: Box<dyn io::Write> = match &args.output {
        Some(out_path) => {
            let file = std::fs::File::create(out_path)?;
            Box::new(file)
        }
        None => {
            let stdout = io::stdout().lock();
            Box::new(stdout)
        }
    };
    Ok(output)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let air_script = read_script(&args).context("failed to read the input")?;
    let output = build_output(&args).context("failed to open the output")?;

    Beautifier::new_with_indent(output, args.indent_size).beautify(&air_script)?;
    Ok(())
}
