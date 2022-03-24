/*
 * Copyright 2022 Fluence Labs Limited
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

use crate::stream::Generation;
use air_parser::ast;

#[derive(Clone, Copy, Debug)]
pub enum Variable<'i> {
    #[allow(dead_code)]
    // position will be needed to implement new for operators
    Scalar { name: &'i str, position: usize },
    Stream {
        name: &'i str,
        generation: Generation,
        position: usize,
    },
}

impl<'i> Variable<'i> {
    pub fn scalar(name: &'i str, position: usize) -> Self {
        Self::Scalar { name, position }
    }

    pub fn stream(name: &'i str, generation: Generation, position: usize) -> Self {
        Self::Stream {
            name,
            generation,
            position,
        }
    }
}

impl<'i> From<&ast::Variable<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::Variable<'i>) -> Self {
        use ast::Variable::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name, scalar.position),
            Stream(stream) => Self::stream(stream.name, Generation::Last, stream.position),
        }
    }
}

impl<'i> From<&ast::VariableWithLambda<'i>> for Variable<'i> {
    fn from(ast_variable: &ast::VariableWithLambda<'i>) -> Self {
        use ast::VariableWithLambda::*;

        match ast_variable {
            Scalar(scalar) => Self::scalar(scalar.name, scalar.position),
            Stream(stream) => Self::stream(stream.name, Generation::Last, stream.position),
        }
    }
}
