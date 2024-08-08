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

mod ap;
mod call;
mod canon;
mod dsl;
mod embed;
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
    TEST_PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = crate::parser::AIRLexer::new(source_code);
        let mut validator = crate::parser::VariableValidator::new();

        parser
            .parse(source_code, &mut errors, &mut validator, lexer)
            .expect("parsing should be successful")
    })
}
