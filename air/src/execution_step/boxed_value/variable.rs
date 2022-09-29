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

    pub(crate) fn stream(name: &'i str, generation: Generation, position: AirPos) -> Self {
        Self::Stream {
            name,
            generation,
            position,
        }
    }

    pub(crate) fn canon_stream(name: &'i str) -> Self {
        Self::CanonStream { name }
    }
}

impl<'i> From<&ast::Variable<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::Variable<'i>) -> Self {
        use ast::Variable::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name),
            Stream(stream) => Self::stream(stream.name, Generation::Last, stream.position),
            CanonStream(canon_stream) => Self::canon_stream(canon_stream.name),
        }
    }
}

impl<'i> From<&ast::VariableWithLambda<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::VariableWithLambda<'i>) -> Self {
        use ast::VariableWithLambda::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name),
            Stream(stream) => Self::stream(stream.name, Generation::Last, stream.position),
            CanonStream(canon_stream) => Self::canon_stream(canon_stream.name),
        }
    }
}
