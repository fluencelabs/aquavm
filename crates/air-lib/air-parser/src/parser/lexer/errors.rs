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

use crate::parser::Span;
use thiserror::Error as ThisError;

use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::ops::Range;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    #[error("this string literal has unclosed quote")]
    UnclosedQuote(Span),

    #[error("empty string aren't allowed in this position")]
    EmptyString(Span),

    #[error("only alphanumeric, '_', and '-' characters are allowed in this position")]
    IsNotAlphanumeric(Span),

    #[error("a stream name should be non empty")]
    EmptyStreamName(Span),

    #[error("this variable or constant shouldn't have empty name")]
    EmptyVariableOrConst(Span),

    #[error("invalid character in lambda")]
    InvalidLambda(Span),

    #[error("a digit could contain only digits or one dot")]
    UnallowedCharInNumber(Span),

    #[error("{1}")]
    ParseIntError(Span, #[source] ParseIntError),

    #[error("{1}")]
    ParseFloatError(Span, #[source] ParseFloatError),

    // TODO: use LambdaParserError directly here (it'll require introducing a lifetime)
    #[error("{se_lambda_parser_error}")]
    LambdaParserError {
        span: Span,
        se_lambda_parser_error: String,
    },

    #[error("{error_path} is an incorrect path for %last_error%, only .$.instruction, .$.msg, and .$.peer_id are allowed")]
    LastErrorPathError { span: Span, error_path: String },

    #[error("this float is too big, a float could contain less than 12 digits")]
    TooBigFloat(Span),

    #[error("leading dot without any symbols before - please write 0 if it's float or variable name if it's a lambda")]
    LeadingDot(Span),
}

impl LexerError {
    pub fn span(&self) -> Span {
        let span = match self {
            Self::UnclosedQuote(span) => span,
            Self::EmptyString(span) => span,
            Self::IsNotAlphanumeric(span) => span,
            Self::EmptyStreamName(span) => span,
            Self::EmptyVariableOrConst(span) => span,
            Self::InvalidLambda(span) => span,
            Self::UnallowedCharInNumber(span) => span,
            Self::ParseIntError(span, _) => span,
            Self::ParseFloatError(span, _) => span,
            Self::LambdaParserError { span, .. } => span,
            Self::LastErrorPathError { span, .. } => span,
            Self::TooBigFloat(span) => span,
            Self::LeadingDot(span) => span,
        };

        *span
    }

    pub fn unclosed_quote(range: Range<usize>) -> Self {
        Self::UnclosedQuote(range.into())
    }

    pub fn empty_string(range: Range<usize>) -> Self {
        Self::EmptyString(range.into())
    }

    pub fn is_not_alphanumeric(range: Range<usize>) -> Self {
        Self::IsNotAlphanumeric(range.into())
    }

    pub fn empty_stream_name(range: Range<usize>) -> Self {
        Self::EmptyStreamName(range.into())
    }

    pub fn empty_variable_or_const(range: Range<usize>) -> Self {
        Self::EmptyVariableOrConst(range.into())
    }

    pub fn invalid_lambda(range: Range<usize>) -> Self {
        Self::InvalidLambda(range.into())
    }

    pub fn unallowed_char_in_number(range: Range<usize>) -> Self {
        Self::UnallowedCharInNumber(range.into())
    }

    pub fn parse_int_error(range: Range<usize>, parse_int_error: ParseIntError) -> Self {
        Self::ParseIntError(range.into(), parse_int_error)
    }

    pub fn parse_float_error(range: Range<usize>, parse_float_error: ParseFloatError) -> Self {
        Self::ParseFloatError(range.into(), parse_float_error)
    }

    pub fn lambda_parser_error(
        range: Range<usize>,
        se_lambda_parser_error: impl Into<String>,
    ) -> Self {
        Self::LambdaParserError {
            span: range.into(),
            se_lambda_parser_error: se_lambda_parser_error.into(),
        }
    }

    pub fn last_error_path_error(range: Range<usize>, error_path: String) -> Self {
        Self::LastErrorPathError {
            span: range.into(),
            error_path,
        }
    }

    pub fn too_big_float(range: Range<usize>) -> Self {
        Self::TooBigFloat(range.into())
    }

    pub fn leading_dot(range: Range<usize>) -> Self {
        Self::LeadingDot(range.into())
    }
}

use super::Token;
use crate::parser::air::__ToTriple;
use crate::parser::ParserError;

impl<'err, 'input, 'i> __ToTriple<'err, 'input, 'i>
    for Result<(usize, Token<'input>, usize), LexerError>
{
    #[allow(clippy::wrong_self_convention)]
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
