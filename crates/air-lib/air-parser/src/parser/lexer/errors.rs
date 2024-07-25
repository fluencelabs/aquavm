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

use super::AirPos;
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

    #[error("a tagged name should be non empty")]
    EmptyTaggedName(Span),

    #[error("a canon name should be non empty")]
    EmptyCanonName(Span),

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

    #[error("this embedded program has not terminating sequence")]
    UnclosedEmbedded(Span),
}

impl LexerError {
    pub fn span(&self) -> Span {
        let span = match self {
            Self::UnclosedQuote(span) => span,
            Self::EmptyString(span) => span,
            Self::IsNotAlphanumeric(span) => span,
            Self::EmptyTaggedName(span) => span,
            Self::EmptyCanonName(span) => span,
            Self::EmptyVariableOrConst(span) => span,
            Self::InvalidLambda(span) => span,
            Self::UnallowedCharInNumber(span) => span,
            Self::ParseIntError(span, _) => span,
            Self::ParseFloatError(span, _) => span,
            Self::LambdaParserError { span, .. } => span,
            Self::LastErrorPathError { span, .. } => span,
            Self::TooBigFloat(span) => span,
            Self::LeadingDot(span) => span,
            Self::UnclosedEmbedded(span) => span,
        };

        *span
    }

    pub fn unclosed_quote(range: Range<AirPos>) -> Self {
        Self::UnclosedQuote(range.into())
    }

    pub fn empty_string(range: Range<AirPos>) -> Self {
        Self::EmptyString(range.into())
    }

    pub fn is_not_alphanumeric(range: Range<AirPos>) -> Self {
        Self::IsNotAlphanumeric(range.into())
    }

    pub fn empty_tagged_name(range: Range<AirPos>) -> Self {
        Self::EmptyTaggedName(range.into())
    }

    pub fn empty_canon_name(range: Range<AirPos>) -> Self {
        Self::EmptyCanonName(range.into())
    }

    pub fn empty_variable_or_const(range: Range<AirPos>) -> Self {
        Self::EmptyVariableOrConst(range.into())
    }

    pub fn invalid_lambda(range: Range<AirPos>) -> Self {
        Self::InvalidLambda(range.into())
    }

    pub fn unallowed_char_in_number(range: Range<AirPos>) -> Self {
        Self::UnallowedCharInNumber(range.into())
    }

    pub fn parse_int_error(range: Range<AirPos>, parse_int_error: ParseIntError) -> Self {
        Self::ParseIntError(range.into(), parse_int_error)
    }

    pub fn parse_float_error(range: Range<AirPos>, parse_float_error: ParseFloatError) -> Self {
        Self::ParseFloatError(range.into(), parse_float_error)
    }

    pub fn lambda_parser_error(
        range: Range<AirPos>,
        se_lambda_parser_error: impl Into<String>,
    ) -> Self {
        Self::LambdaParserError {
            span: range.into(),
            se_lambda_parser_error: se_lambda_parser_error.into(),
        }
    }

    pub fn last_error_path_error(range: Range<AirPos>, error_path: String) -> Self {
        Self::LastErrorPathError {
            span: range.into(),
            error_path,
        }
    }

    pub fn too_big_float(range: Range<AirPos>) -> Self {
        Self::TooBigFloat(range.into())
    }

    pub fn leading_dot(range: Range<AirPos>) -> Self {
        Self::LeadingDot(range.into())
    }

    pub fn unclosed_embedded(range: Range<AirPos>) -> Self {
        Self::UnclosedEmbedded(range.into())
    }
}

use super::Token;
use crate::parser::air::__ToTriple;
use crate::parser::ParserError;

impl<'err, 'input, 'i> __ToTriple<'err, 'input, 'i>
    for Result<(AirPos, Token<'input>, AirPos), LexerError>
{
    #[allow(clippy::wrong_self_convention)]
    fn to_triple(
        value: Self,
    ) -> Result<
        (AirPos, Token<'input>, AirPos),
        lalrpop_util::ParseError<AirPos, Token<'input>, ParserError>,
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
