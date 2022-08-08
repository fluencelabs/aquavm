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

use lalrpop_util::ParseError;

#[test]
fn parse_new_with_scalar() {
    let source_code = r#"(new scalar
            (null)
        )
        "#;

    let instruction = parse(source_code);
    let expected = new(
        NewArgument::Scalar(Scalar::new("scalar", 5)),
        null(),
        Span::new(0, 40),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_new_with_stream() {
    let source_code = r#"(new $stream
            (null)
        )
        "#;

    let instruction = parse(source_code);
    let expected = new(
        NewArgument::Stream(Stream::new("$stream", 5)),
        null(),
        Span::new(0, 41),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn iterators_cant_be_restricted() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable iterator
                (new iterator
                    (call "" ("" "") [iterator])
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
        ParserError::IteratorRestrictionNotAllowed { .. }
    ));
}

#[test]
fn canon_streams_cant_be_restricted() {
    let source_code = r#"
        (seq
            (seq
                (call "" ("" "") [] $stream)
                (canon "" $stream #canon_stream)
            )
            (new #canon_stream
                (null)
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

    assert!(!errors.is_empty());
}
