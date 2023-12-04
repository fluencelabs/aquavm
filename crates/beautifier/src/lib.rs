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

pub use crate::beautifier::{Beautifier, BeautifyError, DEFAULT_INDENT_STEP};

use std::io;

/// Beautify the `air_script` with default settings to the `output`.
pub fn beautify(air_script: &str, output: &mut impl io::Write) -> Result<(), BeautifyError> {
    let mut beautifier = Beautifier::new(output);
    beautifier.beautify(air_script)
}

/// Beautify the `air_script` to a string with default settings.
/// Return error on parsing error.
pub fn beautify_to_string(air_script: &str) -> Result<String, String> {
    let arena = air_parser::Arena::new();
    let ast = air_parser::parse(air_script, &arena)?;
    let mut buffer = vec![];
    let mut beautifier = Beautifier::new(&mut buffer);

    beautifier.beautify_ast(ast).unwrap();
    // Safety: safe because Beautifier produces valid utf8 strings
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
}

#[cfg(test)]
mod tests;
