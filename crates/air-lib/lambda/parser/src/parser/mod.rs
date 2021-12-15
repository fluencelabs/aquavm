/*
 * Copyright 2021 Fluence Labs Limited
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

pub mod lambda_parser;
mod lexer;

// air is auto-generated, so exclude it from `cargo fmt -- --check` and `cargo clippy`
#[rustfmt::skip]
#[allow(clippy::all)]
mod va_lambda;

mod errors;

#[cfg(test)]
pub mod tests;

pub type LambdaParserResult<'input, T> = std::result::Result<T, LambdaParserError<'input>>;

pub use errors::LambdaParserError;
pub use lambda_parser::parse;
pub use lexer::AccessorsLexer;
pub use lexer::LexerError;
pub use va_lambda::LambdaParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}
