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

use air_beautifier::Beautifier;
use clap::Parser;
use eyre::{Context, Result};

use std::{io, path::PathBuf};

#[derive(Parser)]
#[clap(about = "Pretty-print an AIR script to Python-like representation")]
pub(crate) struct Args {
    #[clap(short, long, default_value_t = air_beautifier::DEFAULT_INDENT_STEP)]
    indent_step: usize,
    #[clap(short, long, help = "Recognize virtual instruction patterns")]
    patterns: bool,
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

pub(crate) fn beautify(args: Args) -> Result<()> {
    let air_script = read_script(&args).context("failed to read the input")?;
    let output = build_output(&args).context("failed to open the output")?;

    let mut beautifier = Beautifier::new_with_indent(output, args.indent_step);

    if args.patterns {
        beautifier = beautifier.enable_all_patterns();
    }

    beautifier.beautify(&air_script)?;
    Ok(())
}
