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
pub use lexer::LambdaASTLexer;
pub use lexer::LexerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub left: usize,
    pub right: usize,
}
