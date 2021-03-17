/*
 * Copyright 2020 Fluence Labs Limited
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

pub mod air_parser;
mod lexer;

// air is auto-generated, so exclude it from `cargo fmt -- --check` and `cargo clippy`
#[rustfmt::skip]
#[allow(clippy::all)]
mod air;

pub mod ast;
mod errors;
mod validator;

#[cfg(test)]
pub mod tests;

pub use self::air_parser::parse;
pub use air::AIRParser;
pub use lexer::AIRLexer;

use errors::ParserError;
use validator::VariableValidator;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}
