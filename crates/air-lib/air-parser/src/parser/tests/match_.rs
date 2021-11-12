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
fn parse_match() {
    let source_code = r#"
        (match v1 v2
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = match_(
        Value::Variable(VariableWithLambda::scalar("v1")),
        Value::Variable(VariableWithLambda::scalar("v2")),
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match_with_init_peer_id() {
    let source_code = r#"
        (match v1 %init_peer_id%
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = match_(
        Value::Variable(VariableWithLambda::scalar("v1")),
        Value::InitPeerId,
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_mismatch() {
    let source_code = r#"
        (mismatch v1 v2
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = mismatch(
        Value::Variable(VariableWithLambda::scalar("v1")),
        Value::Variable(VariableWithLambda::scalar("v2")),
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn match_with_bool() {
    let source_code = r#"
         (match isOnline true
            (null)
         )
        "#;

    let left_value = Value::Variable(VariableWithLambda::scalar("isOnline"));
    let right_value = Value::Boolean(true);
    let null = null();
    let expected = match_(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn mismatch_with_bool() {
    let source_code = r#"
         (mismatch true isOnline
            (null)
         )
        "#;

    let left_value = Value::Boolean(true);
    let right_value = Value::Variable(VariableWithLambda::scalar("isOnline"));
    let null = null();
    let expected = mismatch(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}
