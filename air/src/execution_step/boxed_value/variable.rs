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

use super::Generation;
use air_parser::{ast, AirPos};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Variable<'i> {
    Scalar {
        name: &'i str,
    },
    #[allow(dead_code)] // it will be used in BoxedValues
    Stream {
        name: &'i str,
        generation: Generation,
        position: AirPos,
    },
    CanonStream {
        name: &'i str,
    },
}

impl<'i> Variable<'i> {
    pub(crate) fn scalar(name: &'i str) -> Self {
        Self::Scalar { name }
    }

    pub(crate) fn canon_stream(name: &'i str) -> Self {
        Self::CanonStream { name }
    }
}

impl<'i> From<&ast::ImmutableVariable<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::ImmutableVariable<'i>) -> Self {
        use ast::ImmutableVariable::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name),
            CanonStream(canon_stream) => Self::canon_stream(canon_stream.name),
        }
    }
}

impl<'i> From<&ast::ImmutableVariableWithLambda<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::ImmutableVariableWithLambda<'i>) -> Self {
        use ast::ImmutableVariableWithLambda::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name),
            CanonStream(canon_stream) => Self::canon_stream(canon_stream.name),
        }
    }
}
