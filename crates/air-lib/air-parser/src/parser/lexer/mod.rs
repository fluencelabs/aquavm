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

mod air_lexer;
mod call_variable_parser;
mod errors;
mod token;
mod utils;

#[cfg(test)]
mod tests;
pub mod text_pos;

pub use air_lexer::AIRLexer;
pub(crate) use air_lexer::ERROR;
pub(crate) use air_lexer::LAST_ERROR;
pub use errors::LexerError;
pub use text_pos::AirPos;
pub use token::Token;

pub(super) type LexerResult<T> = std::result::Result<T, LexerError>;

use utils::is_air_alphanumeric;
use utils::is_lens_allowed_char;
