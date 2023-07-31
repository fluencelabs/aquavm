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

use super::*;

use std::fmt;

impl fmt::Display for ApResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ApResult::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            Stream(stream) => write!(f, "{stream}"),
        }
    }
}

impl fmt::Display for ImmutableValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImmutableValue::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            LastError(error_accessor) => display_last_error(f, error_accessor),
            Literal(literal) => write!(f, r#""{literal}""#),
            Timestamp => write!(f, "%timestamp%"),
            TTL => write!(f, "%ttl%"),
            Number(number) => write!(f, "{number}"),
            Boolean(bool) => write!(f, "{bool}"),
            EmptyArray => write!(f, "[]"),
            Variable(variable) => write!(f, "{variable}"),
            VariableWithLambda(variable) => write!(f, "{variable}"),
        }
    }
}

impl fmt::Display for ResolvableToPeerIdVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResolvableToPeerIdVariable::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            Literal(literal) => write!(f, r#""{literal}""#),
            Scalar(scalar) => write!(f, "{scalar}"),
            ScalarWithLambda(scalar) => write!(f, "{scalar}"),
            CanonStreamWithLambda(canon_stream) => write!(f, "{canon_stream}"),
        }
    }
}

impl fmt::Display for ResolvableToStringVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResolvableToStringVariable::*;

        match self {
            Literal(literal) => write!(f, r#""{literal}""#),
            Scalar(scalar) => write!(f, "{scalar}"),
            ScalarWithLambda(scalar) => write!(f, "{scalar}"),
            CanonStreamWithLambda(canon_stream) => write!(f, "{canon_stream}"),
        }
    }
}

impl fmt::Display for CallOutputValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallOutputValue::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            Stream(stream) => write!(f, "{stream}"),
            None => Ok(()),
        }
    }
}

impl fmt::Display for ApArgument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ApArgument::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            LastError(error_accessor) => display_last_error(f, error_accessor),
            Literal(str) => write!(f, r#""{str}""#),
            Timestamp => write!(f, "%timestamp%"),
            TTL => write!(f, "%ttl%"),
            Number(number) => write!(f, "{number}"),
            Boolean(bool) => write!(f, "{bool}"),
            EmptyArray => write!(f, "[]"),
            Scalar(scalar) => write!(f, "{scalar}"),
            ScalarWithLambda(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
            CanonStreamWithLambda(canon_stream) => write!(f, "{canon_stream}"),
            CanonStreamMap(canon_stream_map) => write!(f, "{canon_stream_map}"),
            CanonStreamMapWithLambda(canon_stream_map) => write!(f, "{canon_stream_map}"),
        }
    }
}

impl fmt::Display for StreamMapKeyClause<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StreamMapKeyClause::*;

        match self {
            Literal(str) => write!(f, r#""{str}""#),
            Int(int) => write!(f, "{int}"),
            Scalar(scalar) => write!(f, "{scalar}"),
            ScalarWithLambda(scalar) => write!(f, "{scalar}"),
            CanonStreamWithLambda(canon_stream) => write!(f, "{canon_stream}"),
        }
    }
}

impl fmt::Display for Triplet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} {})",
            self.peer_id, self.service_id, self.function_name
        )
    }
}

impl fmt::Display for NewArgument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(scalar) => write!(f, "{scalar}"),
            Self::Stream(stream) => write!(f, "{stream}"),
            Self::CanonStream(canon_stream) => write!(f, "{canon_stream}"),
            Self::StreamMap(stream_map) => write!(f, "{stream_map}"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Number::*;

        match self {
            Int(number) => write!(f, "{number}"),
            Float(number) => write!(f, "{number}"),
        }
    }
}

impl fmt::Display for FoldScalarIterable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FoldScalarIterable::*;

        match self {
            Scalar(scalar) => write!(f, "{scalar}"),
            ScalarWithLambda(scalar) => write!(f, "{scalar}"),
            CanonStream(canon_stream) => write!(f, "{canon_stream}"),
            CanonStreamMap(canon_stream_map) => write!(f, "{canon_stream_map}"),
            EmptyArray => write!(f, "[]"),
        }
    }
}

impl From<Number> for serde_json::Value {
    fn from(number: Number) -> Self {
        (&number).into()
    }
}

impl From<&Number> for serde_json::Value {
    fn from(number: &Number) -> Self {
        match number {
            Number::Int(value) => (*value).into(),
            Number::Float(value) => (*value).into(),
        }
    }
}

fn display_last_error(f: &mut fmt::Formatter, lambda_ast: &Option<LambdaAST>) -> fmt::Result {
    match lambda_ast {
        Some(lambda_ast) => write!(f, "%last_error%{lambda_ast}"),
        None => write!(f, "%last_error%"),
    }
}
