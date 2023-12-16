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

use crate::parser::lexer::LexerError;
use crate::parser::Span;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    #[error(transparent)]
    LexerError(#[from] LexerError),

    #[error("lambda can't be applied to streams in this position")]
    LambdaAppliedToStream(Span),

    #[error("variable '{variable_name}' wasn't defined")]
    UndefinedVariable { span: Span, variable_name: String },

    #[error("iterable '{variable_name}' wasn't defined")]
    UndefinedIterable { span: Span, variable_name: String },

    #[error("last error with non-empty path is ambiguous, please use just %last_error%")]
    AmbiguousFailLastError(Span),

    #[error("new can't be applied to a '{iterator_name}' because it's an iterator")]
    IteratorRestrictionNotAllowed { span: Span, iterator_name: String },

    #[error("multiple iterable values found for iterator name '{iterator_name}'")]
    MultipleIterableValuesForOneIterator { span: Span, iterator_name: String },

    #[error(
        "multiple next instructions for iterator '{iterator_name}' found for one fold, that is prohibited"
    )]
    MultipleNextInFold { span: Span, iterator_name: String },

    #[error("unsupported variable key type in (ap {ap_key_type} value {ap_result_name})")]
    UnsupportedMapKeyType {
        span: Span,
        ap_key_type: String,
        ap_result_name: String,
    },

    #[error("error code 0 with fail is unsupported")]
    UnsupportedLiteralErrCodes { span: Span },
}

impl ParserError {
    pub fn span(&self) -> Span {
        match self {
            Self::LexerError(lexer_error) => lexer_error.span(),
            Self::LambdaAppliedToStream(span) => *span,
            Self::UndefinedVariable { span, .. } => *span,
            Self::UndefinedIterable { span, .. } => *span,
            Self::AmbiguousFailLastError(span) => *span,
            Self::IteratorRestrictionNotAllowed { span, .. } => *span,
            Self::MultipleIterableValuesForOneIterator { span, .. } => *span,
            Self::MultipleNextInFold { span, .. } => *span,
            Self::UnsupportedMapKeyType { span, .. } => *span,
            Self::UnsupportedLiteralErrCodes { span } => *span,
        }
    }

    pub fn undefined_variable(span: Span, variable_name: impl Into<String>) -> Self {
        Self::UndefinedVariable {
            span,
            variable_name: variable_name.into(),
        }
    }

    pub fn undefined_iterable(span: Span, variable_name: impl Into<String>) -> Self {
        Self::UndefinedIterable {
            span,
            variable_name: variable_name.into(),
        }
    }

    pub fn invalid_iterator_restriction(span: Span, iterator_name: impl Into<String>) -> Self {
        Self::IteratorRestrictionNotAllowed {
            span,
            iterator_name: iterator_name.into(),
        }
    }

    pub fn multiple_iterables(span: Span, iterator_name: impl Into<String>) -> Self {
        Self::MultipleIterableValuesForOneIterator {
            span,
            iterator_name: iterator_name.into(),
        }
    }

    pub fn multiple_next_in_fold(span: Span, iterator_name: impl Into<String>) -> Self {
        Self::MultipleNextInFold {
            span,
            iterator_name: iterator_name.into(),
        }
    }

    pub fn unsupported_map_key_type(
        span: Span,
        ap_key_type: impl Into<String>,
        ap_result_name: impl Into<String>,
    ) -> Self {
        Self::UnsupportedMapKeyType {
            span,
            ap_key_type: ap_key_type.into(),
            ap_result_name: ap_result_name.into(),
        }
    }

    pub fn unsupported_literal_errcodes(span: Span) -> Self {
        Self::UnsupportedLiteralErrCodes { span }
    }
}

impl From<std::convert::Infallible> for ParserError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
