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
use crate::ast::*;
use crate::parser::ParserError;

use air_lambda_ast::ValueAccessor;
use lalrpop_util::ParseError;

#[test]
fn fold_with_undefined_iterable() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable i
                (seq
                    (call "" ("" "") ["hello" ""] $void)
                    (next j)
                )
            )
        )
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
        ParserError::UndefinedIterable { .. }
    ));
}

#[test]
fn fold_with_undefined_variable() {
    let source_code = r#"
        (seq
            (null)
            (fold iterable i
                (seq
                    (call "" ("" "") ["hello" ""] $void)
                    (next i)
                )
            )
        )
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
        ParserError::UndefinedVariable { .. }
    ));
}

#[test]
fn scalar_fold_with_multiple_nexts_inside() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable i
                (seq
                    (next i)
                    (next i)
                )
            )
        )
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
        ParserError::MultipleNextInFold { .. }
    ));
}

#[test]
fn multiple_scalar_folds_with_same_iterator() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (seq
                (fold iterable i
                    (seq
                        (null)
                        (next i)
                    )
                )
                (fold iterable i
                    (seq
                        (null)
                        (next i)
                    )
                )
            )
        )
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    let errors = validator.finalize();

    assert!(errors.is_empty());
}

#[test]
fn stream_fold_with_multiple_nexts_inside() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] $stream)
            (fold $stream i
                (seq
                    (next i)
                    (next i)
                )
            )
        )
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
        ParserError::MultipleNextInFold { .. }
    ));
}

#[test]
fn parse_fold_with_multiple_iterator() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] iterable_1)
                (call "" ("" "") [] iterable_2)
            )
            (fold iterable_1 i
                (seq
                    (fold iterable_2 i
                        (seq
                            (call "" ("" "") ["hello" ""] $void)
                            (next i)
                        )
                    )
                    (next i)
                )
            )
        )
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
        ParserError::MultipleIterableValues { .. }
    ));
}

#[test]
fn parse_fold() {
    let source_code = r#"
        (fold iterable i
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = fold_scalar_variable(
        ScalarWithLambda::new("iterable", None, 15),
        Scalar::new("i", 24),
        null(),
        Span::new(9, 54),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_json_path() {
    let source_code = r#"
        ; comment
        (fold members.$.[123321] m (null)) ;;; comment
        ;;; comment
    "#;

    let instruction = parse(source_code);
    let expected = fold_scalar_variable(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![ValueAccessor::ArrayAccess { idx: 123321 }],
            33,
        ),
        Scalar::new("m", 52),
        null(),
        Span::new(27, 61),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_empty_array_iterable() {
    let source_code = r#"
        (fold [] m
            (null)
        )
    "#;

    let instruction = parse(source_code);
    let expected = fold_scalar_empty_array(Scalar::new("m", 18), null(), Span::new(9, 48));
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream() {
    let source_code = r#"
        (fold $stream iterator (null))
    "#;

    let instruction = parse(source_code);
    let expected = fold_stream(
        Stream::new("$stream", 15),
        Scalar::new("iterator", 23),
        null(),
        Span::new(9, 39),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn comments() {
    let source_code = r#"
        ; comment
        (fold members.$.field[1] m (null)) ;;; comment ;;?()()
        ;;; comme;?!.$.  nt[][][][()()()null;$::!
    "#;
    let instruction = parse(source_code);
    let expected = fold_scalar_variable(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![
                ValueAccessor::FieldAccessByName {
                    field_name: "field",
                },
                ValueAccessor::ArrayAccess { idx: 1 },
            ],
            33,
        ),
        Scalar::new("m", 52),
        null(),
        Span::new(27, 61),
    );
    assert_eq!(instruction, expected);
}

fn source_fold_with(name: &str) -> String {
    f!(r#"(fold iterable i
            ({name} (null) (null))
        )"#)
}
#[test]
fn parse_fold_with_xor_par_seq() {
    for name in &["xor", "par", "seq"] {
        let source_code = source_fold_with(name);
        let instruction = parse(&source_code);
        let instr = binary_instruction(*name);
        let expected = fold_scalar_variable(
            ScalarWithLambda::new("iterable", None, 6),
            Scalar::new("i", 15),
            instr(null(), null()),
            Span::new(0, 58),
        );
        assert_eq!(instruction, expected);
    }
}
