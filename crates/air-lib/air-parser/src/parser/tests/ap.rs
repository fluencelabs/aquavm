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

use air_lambda_ast::{LambdaAST, ValueAccessor};
use fstrings::f;
use fstrings::format_args_f;

#[test]
fn ap_with_literal() {
    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Literal("some_string"),
        ApResult::Stream(Stream::new("$stream", 27)),
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
        ApResult::Stream(Stream::new("$stream", 18)),
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
        ApResult::Stream(Stream::new("$stream", 18)),
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
        ApArgument::LastError(Some(unsafe {
            LambdaAST::new_unchecked(vec![ValueAccessor::FieldAccessByName {
                field_name: "message",
            }])
        })),
        ApResult::Stream(Stream::new("$stream", 37)),
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
        ApResult::Stream(Stream::new("$stream", 16)),
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
        ApResult::Stream(Stream::new("$stream", 28)),
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
        ApResult::Stream(Stream::new("$stream", 25)),
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
        ApResult::Stream(Stream::new("$stream", 19)),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream() {
    let canon_stream = "#canon_stream";
    let scalar = "scalar";
    let source_code = f!(r#"
        (ap {canon_stream} {scalar})
    "#);

    let actual = parse(&source_code);
    let expected = ap(
        ApArgument::CanonStream(CanonStream::new(canon_stream, 13)),
        ApResult::Scalar(Scalar::new(scalar, 27)),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_canon_stream_with_lambda() {
    let canon_stream = "#canon_stream";
    let scalar = "scalar";
    let source_code = f!(r#"
        (ap {canon_stream}.$.[0] {scalar})
    "#);

    let actual = parse(&source_code);
    let expected = ap(
        ApArgument::CanonStreamWithLambda(CanonStreamWithLambda::new(
            canon_stream,
            unsafe { LambdaAST::new_unchecked(vec![ValueAccessor::ArrayAccess { idx: 0 }]) },
            13,
        )),
        ApResult::Scalar(Scalar::new(scalar, 33)),
    );

    assert_eq!(actual, expected);
}
