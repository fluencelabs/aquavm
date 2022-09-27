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

mod ap;
mod call;
mod canon;
mod dsl;
mod fail;
mod fold;
mod match_;
mod never;
mod new;
mod null;
mod par;
mod seq;

use crate::ast::Instruction;
use crate::parser::AIRParser;

thread_local!(static TEST_PARSER: AIRParser = AIRParser::new());

fn parse(source_code: &str) -> Instruction {
    *TEST_PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = crate::parser::AIRLexer::new(source_code);
        let mut validator = crate::parser::VariableValidator::new();

        parser
            .parse(source_code, &mut errors, &mut validator, lexer)
            .expect("parsing should be successful")
    })
}
