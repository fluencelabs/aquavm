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

use super::lexer::AlgebraLexer;
use super::va_lambda;
use super::LambdaParserError;
use super::LambdaParserResult;
use crate::LambdaAST;
use crate::ValueAlgebra;

use va_lambda::LambdaParser;

// Caching parser to cache internal regexes, which are expensive to instantiate
// See also https://github.com/lalrpop/lalrpop/issues/269
thread_local!(static PARSER: LambdaParser = LambdaParser::new());

/// Parse AIR `source_code` to `Box<Instruction>`
pub fn parse(lambda: &str) -> LambdaParserResult<'_, LambdaAST> {
    PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = AlgebraLexer::new(lambda);
        let result = parser.parse(lambda, &mut errors, lexer);

        match result {
            Ok(algebras) if errors.is_empty() => try_to_lambda(algebras),
            Ok(_) => Err(errors.into()),
            Err(e) => Err(e.into()),
        }
    })
}

fn try_to_lambda(algebras: Vec<ValueAlgebra>) -> LambdaParserResult<'_, LambdaAST> {
    if algebras.is_empty() {
        return Err(LambdaParserError::EmptyLambda);
    }

    let ast = unsafe { LambdaAST::new_unchecked(algebras) };
    Ok(ast)
}
