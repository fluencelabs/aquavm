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

use thiserror::Error as ThisError;

use std::num::ParseFloatError;
use std::num::ParseIntError;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    #[error("this string literal has unclosed quote")]
    UnclosedQuote(usize, usize),

    #[error("empty string aren't allowed in this position")]
    EmptyString(usize, usize),

    #[error("only alphanumeric, '_', and '-' characters are allowed in this position")]
    IsNotAlphanumeric(usize, usize),

    #[error("an stream name should be non empty")]
    EmptyStreamName(usize, usize),

    #[error("this variable or constant shouldn't have empty name")]
    EmptyVariableOrConst(usize, usize),

    #[error("invalid character in json path")]
    InvalidJsonPath(usize, usize),

    #[error("a digit could contain only digits or one dot")]
    UnallowedCharInNumber(usize, usize),

    #[error("{2}")]
    ParseIntError(usize, usize, #[source] ParseIntError),

    #[error("{2}")]
    ParseFloatError(usize, usize, #[source] ParseFloatError),

    #[error("this float is too big, a float could contain less than 12 digits")]
    TooBigFloat(usize, usize),

    #[error("leading dot without any symbols before - please write 0 if it's float or variable name if it's json path")]
    LeadingDot(usize, usize),
}

use super::Token;
use crate::parser::air::__ToTriple;
use crate::parser::ParserError;

impl<'err, 'input, 'i> __ToTriple<'err, 'input, 'i>
    for Result<(usize, Token<'input>, usize), LexerError>
{
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Token<'input>, usize),
        lalrpop_util::ParseError<usize, Token<'input>, ParserError>,
    > {
        match value {
            Ok(v) => Ok(v),
            Err(error) => {
                let error = ParserError::LexerError(error);
                Err(lalrpop_util::ParseError::User { error })
            }
        }
    }
}

impl From<std::convert::Infallible> for LexerError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
