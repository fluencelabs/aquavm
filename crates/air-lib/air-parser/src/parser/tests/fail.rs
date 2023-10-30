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
    let arena = typed_arena::Arena::new();
    let instruction = parse(source_code, &arena);
    let expected = fail_last_error();
    assert_eq!(instruction, &expected);
}

#[test]
fn parse_fail_literals() {
    let source_code = r#"
           (fail 1 "error message")
        "#;
    let arena = typed_arena::Arena::new();
    let instruction = parse(source_code, &arena);
    let expected = fail_literals(1, "error message");
    assert_eq!(instruction, &expected);
}

#[test]
fn parse_fail_scalars() {
    let source_code = r#"
           (fail scalar)
        "#;
    let arena = typed_arena::Arena::new();
    let instruction = parse(source_code, &arena);
    let expected = fail_scalar(Scalar::new("scalar", 18.into()));
    assert_eq!(instruction, &expected);
}

#[test]
fn parse_fail_scalar_with_lambda() {
    let source_code = r#"
           (fail scalar.$.field_accessor)
        "#;
    let arena = typed_arena::Arena::new();
    let instruction = parse(source_code, &arena);
    let expected = fail_scalar_wl(ScalarWithLambda::new(
        "scalar",
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "field_accessor",
        }])
        .unwrap(),
        18.into(),
    ));
    assert_eq!(instruction, &expected);
}

#[test]
fn parse_fail_scalar_with_error() {
    let source_code = r#"
           (fail :error:)
        "#;
    let arena = typed_arena::Arena::new();
    let instruction = parse(source_code, &arena);
    let expected = fail_error();
    assert_eq!(instruction, &expected);
}

#[test]
fn parse_fail_literal_0() {
    use crate::parser::errors::ParserError;
    use lalrpop_util::ParseError;

    let source_code = r#"
           (fail 0 "some error")
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let arena = typed_arena::Arena::new();
    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, &arena, lexer)
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
