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

use crate::parser::lexer::LexerError;
use crate::parser::lexer::Token;

use lalrpop_util::ErrorRecovery;
use lalrpop_util::ParseError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum LambdaParserError<'input> {
    #[error(transparent)]
    LexerError(#[from] LexerError),

    #[error("provided lambda expression doesn't contain any algebras")]
    EmptyLambda,

    #[error("{0:?}")]
    ParseError(ParseError<usize, Token<'input>, LexerError>),

    #[error("{0:?}")]
    RecoveryErrors(Vec<ErrorRecovery<usize, Token<'input>, LexerError>>),
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
