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

use super::lexer::LambdaASTLexer;
use super::LambdaParserResult;
use crate::parser::errors::IncorrectLambdaError;
use crate::parser::va_lambda::RawLambdaASTParser;
use crate::Functor;
use crate::LambdaAST;
use crate::ValueAccessor;

// Caching parser to cache internal regexes, which are expensive to instantiate
// See also https://github.com/lalrpop/lalrpop/issues/269
thread_local!(static PARSER: RawLambdaASTParser = RawLambdaASTParser::new());

/// Parse AIR lambda ast to `LambdaAST`
pub fn parse(lambda: &str) -> LambdaParserResult<'_, LambdaAST> {
    PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = LambdaASTLexer::new(lambda);
        let result = parser.parse(lambda, &mut errors, lexer);

        match result {
            Ok(lambda_ast) if errors.is_empty() => lambda_ast.try_into().map_err(Into::into),
            Ok(_) => Err(errors.into()),
            Err(e) => Err(e.into()),
        }
    })
}

impl<'input> TryFrom<RawLambdaAST<'input>> for LambdaAST<'input> {
    type Error = IncorrectLambdaError;

    fn try_from(raw_lambda_ast: RawLambdaAST<'input>) -> Result<Self, Self::Error> {
        match raw_lambda_ast {
            RawLambdaAST::ValuePath(accessors) => {
                LambdaAST::try_from_accessors(accessors).or(Err(IncorrectLambdaError::EmptyLambda))
            }
            RawLambdaAST::Functor(functor) => Ok(LambdaAST::from_functor(functor)),
            RawLambdaAST::Error => Err(IncorrectLambdaError::InternalError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum RawLambdaAST<'input> {
    Functor(Functor),
    ValuePath(Vec<ValueAccessor<'input>>),
    // needed to allow parser catch all errors from a lambda expression without stopping on the very first one.
    Error,
}
