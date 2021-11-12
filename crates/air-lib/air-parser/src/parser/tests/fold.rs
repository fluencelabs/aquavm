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
use fstrings::f;
use lalrpop_util::ParseError;

#[test]
fn parse_undefined_iterable() {
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

    assert!(matches!(parser_error, ParserError::UndefinedIterable(..)));
}

#[test]
fn parse_fold() {
    let source_code = r#"
        (fold iterable i
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = fold_scalar(ScalarWithLambda::new("iterable", None), "i", null());
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
    let expected = fold_scalar(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![ValueAccessor::ArrayAccess { idx: 123321 }],
        ),
        "m",
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream() {
    let source_code = r#"
        (fold $stream iterator (null))
    "#;

    let instruction = parse(source_code);
    let expected = fold_stream("$stream", "iterator", null());
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
    let expected = fold_scalar(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![
                ValueAccessor::FieldAccess {
                    field_name: "field",
                },
                ValueAccessor::ArrayAccess { idx: 1 },
            ],
        ),
        "m",
        null(),
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
        let expected = fold_scalar(
            ScalarWithLambda::new("iterable", None),
            "i",
            instr(null(), null()),
        );
        assert_eq!(instruction, expected);
    }
}
