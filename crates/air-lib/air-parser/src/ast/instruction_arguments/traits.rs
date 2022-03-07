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

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            LastError(error_accessor) => display_last_error(f, error_accessor),
            Literal(literal) => write!(f, r#""{}""#, literal),
            Number(number) => write!(f, "{}", number),
            Boolean(bool) => write!(f, "{}", bool),
            EmptyArray => write!(f, "[]"),
            Variable(variable) => write!(f, "{}", variable),
        }
    }
}

impl fmt::Display for CallInstrValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallInstrValue::*;

        match self {
            InitPeerId => write!(f, "%init_peer_id%"),
            Literal(literal) => write!(f, r#""{}""#, literal),
            Variable(variable) => write!(f, "{}", variable),
        }
    }
}

impl fmt::Display for CallOutputValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CallOutputValue::*;

        match self {
            Variable(variable) => write!(f, "{}", variable),
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
            Literal(str) => write!(f, r#""{}""#, str),
            Number(number) => write!(f, "{}", number),
            Boolean(bool) => write!(f, "{}", bool),
            EmptyArray => write!(f, "[]"),
            Scalar(scalar) => write!(f, "{}", scalar),
        }
    }
}

impl fmt::Display for PeerPart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PeerPart::*;

        match self {
            PeerPk(peer_pk) => write!(f, "{}", peer_pk),
            PeerPkWithServiceId(peer_pk, service_id) => write!(f, "({} {})", peer_pk, service_id),
        }
    }
}

impl fmt::Display for FunctionPart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FunctionPart::*;

        match self {
            FuncName(func_name) => write!(f, "{}", func_name),
            ServiceIdWithFuncName(service_id, func_name) => {
                write!(f, "({} {})", service_id, func_name)
            }
        }
    }
}

impl fmt::Display for Triplet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} {})",
            self.peer_pk, self.service_id, self.function_name
        )
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Number::*;

        match self {
            Int(number) => write!(f, "{}", number),
            Float(number) => write!(f, "{}", number),
        }
    }
}

impl fmt::Display for FoldScalarIterable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FoldScalarIterable::*;

        match self {
            Scalar(variable) => write!(f, "{}", variable),
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

fn display_last_error(
    f: &mut fmt::Formatter,
    last_error_accessor: &Option<LambdaAST>,
) -> fmt::Result {
    match last_error_accessor {
        Some(accessor) => write!(f, "%last_error%.$.{}", air_lambda_ast::format_ast(accessor)),
        None => write!(f, "%last_error%"),
    }
}
