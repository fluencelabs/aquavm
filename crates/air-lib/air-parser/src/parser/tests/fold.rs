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
        ParserError::MultipleIterableValuesForOneIterator { .. }
    ));
}

#[test]
fn parse_fold() {
    let source_code = r#"
        (fold iterable i
            (null)
        )
        "#;
    let instruction = parse(source_code);
    let expected = fold_scalar_variable(
        Scalar::new("iterable", 15.into()),
        Scalar::new("i", 24.into()),
        null(),
        None,
        Span::new(9.into(), 54.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_with_scalar_and_last_instruction() {
    let source_code = r#"
        (fold iterable i
            (null)
            (null)
        )
        "#;
    let instruction = parse(source_code);
    let expected = fold_scalar_variable(
        Scalar::new("iterable", 15.into()),
        Scalar::new("i", 24.into()),
        null(),
        Some(null()),
        Span::new(9.into(), 73.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_lambda() {
    let source_code = r#"
        ; comment
        (fold members.$.[123321] m (null)) ;;; comment
        ;;; comment
    "#;

    let instruction = parse(source_code);
    let expected = fold_scalar_variable_wl(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![ValueAccessor::ArrayAccess { idx: 123321 }],
            33.into(),
        ),
        Scalar::new("m", 52.into()),
        null(),
        None,
        Span::new(27.into(), 61.into()),
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
    let expected = fold_scalar_empty_array(
        Scalar::new("m", 18.into()),
        null(),
        None,
        Span::new(9.into(), 48.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream() {
    let source_code = r#"
        (fold $stream iterator (null))
    "#;

    let instruction = parse(source_code);
    let expected = fold_stream(
        Stream::new("$stream", 15.into()),
        Scalar::new("iterator", 23.into()),
        null(),
        None,
        Span::new(9.into(), 39.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream_with_last_null() {
    let source_code = r#"
        (fold $stream iterator
            (null)
            (null)
        )
    "#;

    let instruction = parse(source_code);
    let expected = fold_stream(
        Stream::new("$stream", 15.into()),
        Scalar::new("iterator", 23.into()),
        null(),
        Some(null()),
        Span::new(9.into(), 79.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_canon_stream_obsolete_syntax() {
    let canon_stream = "#canon_stream";
    let iterator = "iterator";
    let source_code = format!(
        r#"
        (fold {canon_stream} {iterator} (null))
    "#
    );

    let instruction = parse(&source_code);
    let expected = fold_scalar_canon_stream(
        CanonStream::new(canon_stream, 15.into()),
        Scalar::new(iterator, 29.into()),
        null(),
        None,
        Span::new(9.into(), 45.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_canon_stream() {
    let canon_stream = "#$canon_stream";
    let iterator = "iterator";
    let source_code = format!(
        r#"
        (fold {canon_stream} {iterator} (null))
    "#
    );

    let instruction = parse(&source_code);
    let expected = fold_scalar_canon_stream(
        CanonStream::new(canon_stream, 15.into()),
        Scalar::new(iterator, 30.into()),
        null(),
        None,
        Span::new(9.into(), 46.into()),
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
    let expected = fold_scalar_variable_wl(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![
                ValueAccessor::FieldAccessByName {
                    field_name: "field",
                },
                ValueAccessor::ArrayAccess { idx: 1 },
            ],
            33.into(),
        ),
        Scalar::new("m", 52.into()),
        null(),
        None,
        Span::new(27.into(), 61.into()),
    );
    assert_eq!(instruction, expected);
}

fn source_fold_with(name: &str) -> String {
    format!(
        r#"(fold iterable i
            ({name} (null) (null))
        )"#
    )
}
#[test]
fn parse_fold_with_xor_par_seq() {
    for name in &["xor", "par", "seq"] {
        let source_code = source_fold_with(name);
        let instruction = parse(&source_code);
        let instr = binary_instruction(name);
        let expected = fold_scalar_variable(
            Scalar::new("iterable", 6.into()),
            Scalar::new("i", 15.into()),
            instr(null(), null()),
            None,
            Span::new(0.into(), 58.into()),
        );
        assert_eq!(instruction, expected);
    }
}

#[test]
fn fold_on_canon_stream_map() {
    let canon_map = "#%canon_map";
    let iterator = "iterator";
    let source_code = format!(
        r#"
        (fold {canon_map} {iterator} (null))
    "#
    );

    let instruction = parse(&source_code);
    let expected = fold_scalar_canon_stream_map(
        CanonStreamMap::new("#%canon_map", 15.into()),
        Scalar::new(iterator, 27.into()),
        null(),
        None,
        Span::new(9.into(), 43.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_scalar_with_subtree_and_next() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable i
                (seq
                    (seq
                        (call "" ("" "") ["hello" ""] $void)
                        (call "" ("" "") ["hello" ""] $voida)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next_in_a_fold() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] iterable1)
                (call "" ("" "") [] iterable2)
            )
            (fold iterable1 i
                (seq
                    (fold iterable2 it
                        (call "" ("" "") ["hello" ""] $void)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] iterable)
        (fold iterable i
            (seq
                (next i)
                (call "" ("" "") ["hello" ""] $void)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next_in_a_fold1() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] iterable1)
                (call "" ("" "") [] iterable2)
            )
            (fold iterable1 i
                (seq
                    (fold iterable2 it
                        (seq
                            (next i)
                            (call "" ("" "") ["hello" ""] $void)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next_in_a_fold2() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] iterable1)
                (call "" ("" "") [] iterable2)
            )
            (fold iterable1 i
                (seq
                    (fold iterable2 it
                        (seq
                            (call "" ("" "") ["hello" ""] $void)
                            (next i)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next_in_a_branch1() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable i
                (seq
                    (seq
                        (call "" ("" "") ["hello" ""] $void)
                        (next i)
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_scalar_with_next_in_a_branch2() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] $iterable)
            (fold $iterable i
                (seq
                    (call "" ("" "") ["hello" ""] $voida)
                    (seq
                        (next i)
                        (call "" ("" "") ["hello" ""] $void)
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
    dbg!(&errors);
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_subtree_and_next() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] $iterable)
            (fold $iterable i
                (seq
                    (seq
                        (call "" ("" "") ["hello" ""] $void)
                        (call "" ("" "") ["hello" ""] $voida)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_with_next_in_a_fold() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $iterable1)
                (call "" ("" "") [] $iterable2)
            )
            (fold $iterable1 i
                (seq
                    (fold $iterable2 it
                        (call "" ("" "") ["hello" ""] $void)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_with_next_neg() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (seq
                (next i)
                (call "" ("" "") ["hello" ""] $void)
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
    dbg!(&errors);
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_next_in_a_fold1() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $iterable1)
                (call "" ("" "") [] $iterable2)
            )
            (fold $iterable1 i
                (seq
                    (fold $iterable2 it
                        (seq
                            (next i)
                            (call "" ("" "") ["hello" ""] $void)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    dbg!(&errors);
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_next_in_a_fold2() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $iterable1)
                (call "" ("" "") [] $iterable2)
            )
            (fold $iterable1 i
                (seq
                    (fold $iterable2 it
                        (seq
                            (call "" ("" "") ["hello" ""] $void)
                            (next i)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    dbg!(&errors); // WIP remove all
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_next_in_a_branch1_neg() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] $iterable)
            (fold $iterable i
                (seq
                    (seq
                        (call "" ("" "") ["hello" ""] $void)
                        (next i)
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
    dbg!(&errors);
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_next_in_a_branch2_neg() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] $iterable)
            (fold $iterable i
                (seq
                    (call "" ("" "") ["hello" ""] $voida)
                    (seq
                        (next i)
                        (call "" ("" "") ["hello" ""] $void)
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
    dbg!(&errors);
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_xor() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (xor
                (next i)
                (call "" ("" "") ["hello" ""] $void)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_with_xor_and_par() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (xor
                (par
                    (ap 42 some)
                    (next i)
                )
                (par
                    (call "" ("" "") ["hello" ""] $void)
                    (ap 42 some)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_multiple_folds_same_iter_names() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (seq
            (fold $iterable i
                (seq
                    (ap 42 scalar)
                    (next i)
                )
            )
            (fold $iterable i
                (seq
                    (ap 42 scalar)
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_with_next_in_a_fold1_neg() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $iterable1)
                (call "" ("" "") [] $iterable2)
            )
            (fold $iterable1 i
                (seq
                    (fold $iterable2 it
                        (seq
                            (next i)
                            (call "" ("" "") ["hello" ""] $void)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_next_in_a_fold2_neg() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $iterable1)
                (call "" ("" "") [] $iterable2)
            )
            (fold $iterable1 i
                (seq
                    (fold $iterable2 it
                        (seq
                            (call "" ("" "") ["hello" ""] $void)
                            (next i)
                        )
                    )
                    (call "" ("" "") ["hello" ""] $voida)
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
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_multiple_folds_same_iter_names_neg() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (seq
            (fold $iterable i
                (seq
                    (next i)
                    (ap 42 scalar)
                )
            )
            (fold $iterable i
                (seq
                    (next i)
                    (ap 42 scalar)
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
    assert_eq!(errors.len(), 2);

    errors.iter().map(|e| &e.error).for_each(|error| {
        let parser_error = match error {
            ParseError::User { error } => error,
            _ => panic!("unexpected error type"),
        };

        assert!(matches!(
            parser_error,
            ParserError::FoldHasInstructionAfterNext { .. }
        ));
    })
}

#[test]
fn fold_on_stream_with_xor_and_par_neg() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (xor
                (par
                    (next i)
                    (ap 42 some)
                )
                (par
                    (call "" ("" "") ["hello" ""] $void)
                    (ap 42 some)
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
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_xor_and_nested_fold_neg_1() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (xor
                (seq
                    (fold $iterable it
                        (seq
                            (next it)
                            (ap 42 some)
                        )
                    )
                    (next i)
                )
                (seq
                    (call "" ("" "") ["hello" ""] $void)
                    (ap 42 some)
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
    assert_eq!(errors.len(), 1);

    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(
        parser_error,
        ParserError::FoldHasInstructionAfterNext { .. }
    ));
}

#[test]
fn fold_on_stream_with_xor_and_nested_fold_2() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (seq
                (ap 42 some)
                (xor
                    (next i)
                    (fold $iterable it
                        (seq
                            (ap 42 some)
                            (next it)
                        )
                    )
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
    assert_eq!(errors.len(), 0);
}

#[test]
fn fold_on_stream_with_xor_and_nested_fold_neg_2() {
    let source_code = r#"
    (seq
        (call "" ("" "") [] $iterable)
        (fold $iterable i
            (seq
                (xor
                    (next i)
                    (fold $iterable it
                        (seq
                            (next it)
                            (ap 42 some)
                        )
                    )

                )
                (ap 42 some)
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
    assert_eq!(errors.len(), 2);

    errors.iter().map(|e| &e.error).for_each(|error| {
        let parser_error = match error {
            ParseError::User { error } => error,
            _ => panic!("unexpected error type"),
        };

        assert!(matches!(
            parser_error,
            ParserError::FoldHasInstructionAfterNext { .. }
        ));
    })
}

#[test]
fn fold_on_stream_with_multiple_folds() {
    let source_code = r#"
        (new $inner
            (seq
                (par
                    (fold $inner ns
                        (seq
                            (ap ns $result)
                            (next ns)
                        )
                    )
                    (null)
                )
                (par
                    (fold $inner ns
                        (next ns)
                    )
                    (null)
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
    dbg!(errors.clone());
    assert_eq!(errors.len(), 0);
}
