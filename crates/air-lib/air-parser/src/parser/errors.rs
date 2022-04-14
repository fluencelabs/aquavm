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

// TODO: replace usize pair with Span
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

    /// Semantic errors in a call instructions.
    #[error("call should have service id specified by peer part or function part")]
    InvalidCallTriplet(Span),

    #[error("new can't be applied to a '{iterator_name}' because it's an iterator")]
    IteratorRestrictionNotAllowed { span: Span, iterator_name: String },

    #[error("multiple iterable values found for iterator name '{iterator_name}'")]
    MultipleIterableValues { span: Span, iterator_name: String },

    #[error(
        "multiple next instructions for iterator '{iterator_name}' found for one fold, that is prohibited"
    )]
    MultipleNextInFold { span: Span, iterator_name: String },
}

impl ParserError {
    pub fn span(&self) -> Span {
        match self {
            Self::LexerError(lexer_error) => lexer_error.span(),
            Self::LambdaAppliedToStream(span) => *span,
            Self::UndefinedVariable { span, .. } => *span,
            Self::UndefinedIterable { span, .. } => *span,
            Self::AmbiguousFailLastError(span) => *span,
            Self::InvalidCallTriplet(span) => *span,
            Self::IteratorRestrictionNotAllowed { span, .. } => *span,
            Self::MultipleIterableValues { span, .. } => *span,
            Self::MultipleNextInFold { span, .. } => *span,
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
        Self::MultipleIterableValues {
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
}

impl From<std::convert::Infallible> for ParserError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
