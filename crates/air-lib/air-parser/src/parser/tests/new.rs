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

use lalrpop_util::ParseError;

#[test]
fn parse_new_with_scalar() {
    let source_code = r#"(new scalar
            (null)
        )
        "#;

    let instruction = parse(source_code);
    let expected = new(
        NewArgument::Scalar(Scalar::new("scalar", 5.into())),
        null(),
        Span::new(0.into(), 40.into()),
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
        NewArgument::Stream(Stream::new("$stream", 5.into())),
        null(),
        Span::new(0.into(), 41.into()),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_new_with_canon_stream() {
    let source_code = r#"(new #canon_stream
            (null)
        )
        "#;

    let instruction = parse(source_code);
    let expected = new(
        NewArgument::CanonStream(CanonStream::new("#canon_stream", 5.into())),
        null(),
        Span::new(0.into(), 47.into()),
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
fn parse_new_with_stream_map() {
    let source_code = r#"(new %stream
            (null)
        )
        "#;

    let instruction = parse(source_code);
    let expected = new(
        NewArgument::StreamMap(StreamMap::new("%stream", 5.into())),
        null(),
        Span::new(0.into(), 41.into()),
    );
    assert_eq!(instruction, expected);
}
