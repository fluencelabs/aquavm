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

use air_lambda_ast::{LambdaAST, ValueAccessor};

#[test]
fn ap_with_literal() {
    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Literal("some_string".into()),
        ApResult::Stream(Stream::new("$stream", 27.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_number() {
    let source_code = r#"
        (ap -100 $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Number(Number::Int(-100)),
        ApResult::Stream(Stream::new("$stream", 18.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_bool() {
    let source_code = r#"
        (ap true $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Boolean(true),
        ApResult::Stream(Stream::new("$stream", 18.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_last_error() {
    let source_code = r#"
        (ap %last_error%.$.message! $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::LastError(Some(
            LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
                field_name: "message",
            }])
            .unwrap(),
        )),
        ApResult::Stream(Stream::new("$stream", 37.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_empty_array() {
    let source_code = r#"
        (ap [] $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::EmptyArray,
        ApResult::Stream(Stream::new("$stream", 16.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_init_peer_id() {
    let source_code = r#"
        (ap %init_peer_id% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::InitPeerId,
        ApResult::Stream(Stream::new("$stream", 28.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_timestamp() {
    let source_code = r#"
        (ap %timestamp% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Timestamp,
        ApResult::Stream(Stream::new("$stream", 25.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_ttl() {
    let source_code = r#"
        (ap %ttl% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::TTL,
        ApResult::Stream(Stream::new("$stream", 19.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream() {
    let canon_stream = "#canon_stream";
    let scalar = "scalar";
    let source_code = format!(
        r#"
        (ap {canon_stream} {scalar})
    "#
    );

    let actual = parse(&source_code);
    let expected = ap(
        ApArgument::CanonStream(CanonStream::new(canon_stream, 13.into())),
        ApResult::Scalar(Scalar::new(scalar, 27.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream_with_lambda() {
    let canon_stream = "#canon_stream";
    let scalar = "scalar";
    let source_code = format!(
        r#"
        (ap {canon_stream}.$.[0] {scalar})
    "#
    );

    let actual = parse(&source_code);
    let expected = ap(
        ApArgument::CanonStreamWithLambda(CanonStreamWithLambda::new(
            canon_stream,
            LambdaAST::try_from_accessors(vec![ValueAccessor::ArrayAccess { idx: 0 }]).unwrap(),
            13.into(),
        )),
        ApResult::Scalar(Scalar::new(scalar, 33.into())),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_stream_map() {
    // 4 variants
    let var_name = "%stream_map";
    let key_name = "keyo";
    let value = "some_string";
    let source_code = format!(
        r#"
        (ap ("{key_name}" "{value}") %stream_map)
    "#
    );
    let actual = parse(source_code.as_str());
    let expected = ap_with_map(
        StreamMapKeyClause::Literal(key_name.into()),
        ApArgument::Literal(value.into()),
        StreamMap::new(var_name, source_code.find(var_name).unwrap().into()),
    );
    assert_eq!(actual, expected);

    // It is possible to use Scalar as a key in the context of a parser
    // but populate_context will raise an error
    let source_code = format!(
        r#"
        (ap ({key_name} "{value}") %stream_map)
    "#
    );
    let actual = parse(source_code.as_str());
    let expected = ap_with_map(
        StreamMapKeyClause::Scalar(Scalar::new(
            key_name,
            source_code.find(key_name).unwrap().into(),
        )),
        ApArgument::Literal(value.into()),
        StreamMap::new(var_name, source_code.find(var_name).unwrap().into()),
    );
    assert_eq!(actual, expected);

    let source_code = format!(
        r#"
        (ap ("{key_name}" {value}) %stream_map)
    "#
    );
    let actual = parse(source_code.as_str());
    let expected = ap_with_map(
        StreamMapKeyClause::Literal(key_name.into()),
        ApArgument::Scalar(Scalar::new(value, source_code.find(value).unwrap().into())),
        StreamMap::new(var_name, source_code.find(var_name).unwrap().into()),
    );
    assert_eq!(actual, expected);

    // It is possible to use Scalar as a key in the context of a parser
    // but populate_context will raise an error
    let source_code = format!(
        r#"
        (ap ({key_name} {value}) %stream_map)
    "#
    );
    let actual = parse(source_code.as_str());
    let expected = ap_with_map(
        StreamMapKeyClause::Scalar(Scalar::new(
            key_name,
            source_code.find(key_name).unwrap().into(),
        )),
        ApArgument::Scalar(Scalar::new(value, source_code.find(value).unwrap().into())),
        StreamMap::new(var_name, source_code.find(var_name).unwrap().into()),
    );
    assert_eq!(actual, expected);

    let key_name = 123;
    let source_code = format!(
        r#"
        (ap ({key_name} {value}) %stream_map)
    "#
    );
    let actual = parse(source_code.as_str());
    let expected = ap_with_map(
        StreamMapKeyClause::Int(key_name),
        ApArgument::Scalar(Scalar::new(value, source_code.find(value).unwrap().into())),
        StreamMap::new(var_name, source_code.find(var_name).unwrap().into()),
    );
    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream_map_lambda_literal_key() {
    let source_code = format!(
        r#"
        (ap #%canon.$.key scalar)
    "#
    );
    let actual = parse(source_code.as_str());
    let canon_stream_map: CanonStreamMapWithLambda<'_> = CanonStreamMapWithLambda {
        name: "#%canon",
        lambda: LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "key",
        }])
        .unwrap(),
        position: 13.into(),
    };
    let expected = ap(
        ApArgument::CanonStreamMapWithLambda(canon_stream_map),
        ApResult::Scalar(Scalar::new("scalar", 27.into())),
    );
    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream_map_lambda_numeric_key() {
    let source_code = format!(
        r#"
        (ap #%canon.$.[42] scalar)
    "#
    );
    let actual = parse(source_code.as_str());
    let canon_stream_map: CanonStreamMapWithLambda<'_> = CanonStreamMapWithLambda {
        name: "#%canon",
        lambda: LambdaAST::try_from_accessors(vec![ValueAccessor::ArrayAccess { idx: 42 }])
            .unwrap(),
        position: 13.into(),
    };
    let expected = ap(
        ApArgument::CanonStreamMapWithLambda(canon_stream_map),
        ApResult::Scalar(Scalar::new("scalar", 28.into())),
    );
    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream_map_lambda_scalar_key() {
    let source_code = format!(
        r#"
        (ap #%canon.$.[key_scalar] scalar)
    "#
    );
    let actual = parse(source_code.as_str());

    let canon_stream_map: CanonStreamMapWithLambda<'_> = CanonStreamMapWithLambda {
        name: "#%canon",
        lambda: LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByScalar {
            scalar_name: "key_scalar",
        }])
        .unwrap(),
        position: 13.into(),
    };
    let expected = ap(
        ApArgument::CanonStreamMapWithLambda(canon_stream_map),
        ApResult::Scalar(Scalar::new("scalar", 36.into())),
    );
    assert_eq!(actual, expected);
}
