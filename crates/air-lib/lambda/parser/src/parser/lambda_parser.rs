/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
