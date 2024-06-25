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

mod beautifier;
mod r#virtual;

pub use crate::beautifier::{Beautifier, BeautifyError, DEFAULT_INDENT_STEP};

use std::io;

/// Beautify the `air_script` with default settings to the `output`.
pub fn beautify(
    air_script: &str,
    output: &mut impl io::Write,
    enable_patterns: bool,
) -> Result<(), BeautifyError> {
    let mut beautifier = Beautifier::new(output);
    if enable_patterns {
        beautifier = beautifier.enable_all_patterns();
    }
    beautifier.beautify(air_script)
}

/// Beautify the `air_script` to a string with default settings.
/// Return error on parsing error.
pub fn beautify_to_string(air_script: &str) -> Result<String, String> {
    let ast = air_parser::parse(air_script)?;
    let mut buffer = vec![];
    let mut beautifier = Beautifier::new(&mut buffer);

    beautifier.beautify_ast(&ast).unwrap();
    // Safety: safe because Beautifier produces valid utf8 strings
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
}

#[cfg(test)]
mod tests;
