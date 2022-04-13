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
    LambdaAppliedToStream(usize, usize),

    #[error("variable '{2}' wasn't defined")]
    UndefinedVariable(usize, usize, String),

    #[error("iterable '{2}' wasn't defined")]
    UndefinedIterable(usize, usize, String),

    #[error("last error with non-empty path is ambiguous, please use just %last_error%")]
    AmbiguousFailLastError(usize, usize),

    /// Semantic errors in a call instructions.
    #[error("call should have service id specified by peer part or function part")]
    InvalidCallTriplet(usize, usize),

    #[error("new can't be applied to a '{2}' because it's an iterator")]
    IteratorRestrictionNotAllowed(usize, usize, String),

    #[error("multiple iterable values found for iterator name '{2}'")]
    MultipleIterableValues(usize, usize, String),

    #[error(
        "multiple next instructions for iterator '{2}' found for one fold, that is prohibited"
    )]
    MultipleNextInFold(usize, usize, String),
}

impl ParserError {
    pub fn undefined_iterable(span: Span, variable_name: impl Into<String>) -> Self {
        Self::UndefinedIterable(span.left, span.right, variable_name.into())
    }

    pub fn invalid_iterator_restriction(span: Span, variable_name: impl Into<String>) -> Self {
        Self::IteratorRestrictionNotAllowed(span.left, span.right, variable_name.into())
    }

    pub fn multiple_iterables(span: Span, variable_name: impl Into<String>) -> Self {
        Self::MultipleIterableValues(span.left, span.right, variable_name.into())
    }

    pub fn undefined_variable(span: Span, variable_name: impl Into<String>) -> Self {
        Self::UndefinedVariable(span.left, span.right, variable_name.into())
    }

    pub fn multiple_next_in_fold(span: Span, variable_name: impl Into<String>) -> Self {
        Self::MultipleNextInFold(span.left, span.right, variable_name.into())
    }
}

impl From<std::convert::Infallible> for ParserError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
