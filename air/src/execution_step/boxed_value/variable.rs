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
use air_parser::ast;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Variable<'i> {
    Scalar(&'i str),
    Stream { name: &'i str, generation: Generation },
}

impl<'i> Variable<'i> {
    pub(crate) fn scalar(name: &'i str) -> Self {
        Self::Scalar(name)
    }

    #[allow(dead_code)]
    pub(crate) fn from_ast_with_generation(ast_variable: &ast::Variable<'i>, generation: Generation) -> Self {
        use ast::Variable::*;

        match ast_variable {
            Scalar(scalar) => Variable::Scalar(scalar.name),
            Stream(stream) => Variable::Stream {
                name: stream.name,
                generation,
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_stream(name: &'i str, generation: Generation) -> Self {
        Self::Stream { name, generation }
    }
}

impl<'i> From<&ast::Variable<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::Variable<'i>) -> Self {
        use ast::Variable::*;

        match ast_variable {
            Scalar(scalar) => Self::Scalar(scalar.name),
            Stream(stream) => Self::Stream {
                name: stream.name,
                generation: Generation::Last,
            },
        }
    }
}

impl<'i> From<&ast::VariableWithLambda<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::VariableWithLambda<'i>) -> Self {
        use ast::VariableWithLambda::*;

        match ast_variable {
            Scalar(scalar) => Self::Scalar(scalar.name),
            Stream(stream) => Self::Stream {
                name: stream.name,
                generation: Generation::Last,
            },
        }
    }
}
