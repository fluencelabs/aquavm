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

use super::dsl::*;
use super::parse;
use crate::ast::Scalar;
use crate::ast::ScalarWithLambda;

use air_lambda_ast::LambdaAST;
use air_lambda_ast::ValueAccessor;

#[test]
fn parse_fail_last_error() {
    let source_code = r#"
           (fail %last_error%)
        "#;
    let instruction = parse(source_code);
    let expected = fail_last_error();
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_literals() {
    let source_code = r#"
           (fail 1 "error message")
        "#;
    let instruction = parse(source_code);
    let expected = fail_literals(1, "error message");
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_scalars() {
    let source_code = r#"
           (fail scalar)
        "#;
    let instruction = parse(source_code);
    let expected = fail_scalar(Scalar::new("scalar", 18.into()));
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_scalar_with_lambda() {
    let source_code = r#"
           (fail scalar.$.field_accessor)
        "#;
    let instruction = parse(source_code);
    let expected = fail_scalar_wl(ScalarWithLambda::new(
        "scalar",
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "field_accessor",
        }])
        .unwrap(),
        18.into(),
    ));
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_scalar_with_error() {
    let source_code = r#"
           (fail :error:)
        "#;
    let instruction = parse(source_code);
    let expected = fail_error();
    assert_eq!(instruction, expected)
}

#[test]
fn parse_fail_literal_0() {
    use crate::parser::errors::ParserError;
    use lalrpop_util::ParseError;

    let source_code = r#"
           (fail 0 "some error")
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    let errors = validator.finalize();

    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::UnsupportedLiteralErrCodes { .. }
    ));
}
