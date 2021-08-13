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
use air_parser::ast::AstVariable;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Variable<'i> {
    Scalar(&'i str),
    Stream { name: &'i str, generation: Generation },
}

impl<'i> Variable<'i> {
    pub(crate) fn from_ast(ast_variable: &AstVariable<'i>) -> Self {
        match ast_variable {
            AstVariable::Scalar(name) => Variable::Scalar(name),
            AstVariable::Stream(name) => Variable::Stream {
                name,
                generation: Generation::Last,
            },
        }
    }

    pub(crate) fn from_ast_with_generation(ast_variable: &AstVariable<'i>, generation: Generation) -> Self {
        match ast_variable {
            AstVariable::Scalar(name) => Variable::Scalar(name),
            AstVariable::Stream(name) => Variable::Stream { name, generation },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_stream(name: &'i str, generation: Generation) -> Self {
        Self::Stream { name, generation }
    }
}
