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

pub mod air_parser;
mod air_utils;
pub(crate) mod lexer;
mod span;

// air is auto-generated, so exclude it from `cargo fmt -- --check` and `cargo clippy`
#[rustfmt::skip]
#[allow(clippy::all)]
mod air;

mod errors;
mod validator;

#[cfg(test)]
pub mod tests;

pub use self::air_parser::parse;
pub use air::AIRParser;
pub use lexer::AIRLexer;
pub(crate) use lexer::ERROR;
pub(crate) use lexer::LAST_ERROR;
pub use span::Span;
pub use validator::VariableValidator;

use errors::ParserError;
