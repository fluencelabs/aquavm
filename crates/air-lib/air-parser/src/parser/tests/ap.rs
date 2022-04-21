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

#[test]
fn ap_with_literal() {
    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Literal("some_string"),
        Variable::stream("$stream", 27),
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
        Variable::stream("$stream", 18),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_bool() {
    let source_code = r#"
        (ap true $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::Boolean(true), Variable::stream("$stream", 18));

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
        Variable::stream("$stream", 37),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_empty_array() {
    let source_code = r#"
        (ap [] $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::EmptyArray, Variable::stream("$stream", 16));

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_init_peer_id() {
    let source_code = r#"
        (ap %init_peer_id% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::InitPeerId, Variable::stream("$stream", 28));

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_timestamp() {
    let source_code = r#"
        (ap %timestamp% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::Timestamp, Variable::stream("$stream", 25));

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_ttl() {
    let source_code = r#"
        (ap %ttl% $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::TTL, Variable::stream("$stream", 19));

    assert_eq!(actual, expected);
}
