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

#[test]
fn ap_with_literal() {
    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Literal("some_string"),
        Variable::stream("$stream"),
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
        Variable::stream("$stream"),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_bool() {
    let source_code = r#"
        (ap true $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(ApArgument::Boolean(true), Variable::stream("$stream"));

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_last_error() {
    let source_code = r#"
        (ap %last_error%.$.msg! $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::LastError(LastErrorPath::Message),
        Variable::stream("$stream"),
    );

    assert_eq!(actual, expected);
}
