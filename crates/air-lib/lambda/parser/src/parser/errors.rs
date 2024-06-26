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

use crate::parser::lexer::LexerError;
use crate::parser::lexer::Token;

use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum LambdaParserError<'input> {
    #[error(transparent)]
    LexerError(#[from] LexerError),

    #[error(transparent)]
    LambdaError(#[from] IncorrectLambdaError),

    #[error("{0:?}")]
    ParseError(ParseError<usize, Token<'input>, LexerError>),

    #[error("{0:?}")]
    RecoveryErrors(Vec<ErrorRecovery<usize, Token<'input>, LexerError>>),
}

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum IncorrectLambdaError {
    #[error("provided lambda expression doesn't contain any accessor")]
    EmptyLambda,

    #[error(
        "normally, this error shouldn't occur, it's an internal error of a parser implementation"
    )]
    InternalError,
}

impl<'input> From<ParseError<usize, Token<'input>, LexerError>> for LambdaParserError<'input> {
    fn from(e: ParseError<usize, Token<'input>, LexerError>) -> Self {
        Self::ParseError(e)
    }
}

impl<'input> From<Vec<ErrorRecovery<usize, Token<'input>, LexerError>>>
    for LambdaParserError<'input>
{
    fn from(errors: Vec<ErrorRecovery<usize, Token<'input>, LexerError>>) -> Self {
        Self::RecoveryErrors(errors)
    }
}
