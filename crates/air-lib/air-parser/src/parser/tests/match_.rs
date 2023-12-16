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
fn parse_match() {
    let source_code = r#"
        (match v1 v2
            (null)
        )
        "#;
    let instruction = parse(source_code);
    let expected = match_(
        ImmutableValue::Variable(ImmutableVariable::scalar("v1", 16.into())),
        ImmutableValue::Variable(ImmutableVariable::scalar("v2", 19.into())),
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match_with_canon_stream() {
    let canon_stream = "#canon_stream";
    let canon_stream_lambda = ".$.[0]";
    let source_code = format!(
        r#"
        (match {canon_stream}{canon_stream_lambda} v2
            (null)
        )
        "#
    );

    let instruction = parse(&source_code);
    let expected = match_(
        ImmutableValue::VariableWithLambda(ImmutableVariableWithLambda::canon_stream(
            canon_stream,
            LambdaAST::try_from_accessors(vec![ValueAccessor::ArrayAccess { idx: 0 }]).unwrap(),
            16.into(),
        )),
        ImmutableValue::Variable(ImmutableVariable::scalar("v2", 36.into())),
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
    let instruction = parse(source_code);
    let expected = match_(
        ImmutableValue::Variable(ImmutableVariable::scalar("v1", 16.into())),
        ImmutableValue::InitPeerId,
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match_with_timestamp() {
    let source_code = r#"
        (match %timestamp% v1
            (null)
        )
        "#;
    let instruction = parse(source_code);
    let expected = match_(
        ImmutableValue::Timestamp,
        ImmutableValue::Variable(ImmutableVariable::scalar("v1", 28.into())),
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match_with_ttl() {
    let source_code = r#"
        (match %ttl% v1
            (null)
        )
        "#;
    let instruction = parse(source_code);
    let expected = match_(
        ImmutableValue::TTL,
        ImmutableValue::Variable(ImmutableVariable::scalar("v1", 22.into())),
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
    let instruction = parse(source_code);
    let expected = mismatch(
        ImmutableValue::Variable(ImmutableVariable::scalar("v1", 19.into())),
        ImmutableValue::Variable(ImmutableVariable::scalar("v2", 22.into())),
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

    let left_value = ImmutableValue::Variable(ImmutableVariable::scalar("isOnline", 17.into()));
    let right_value = ImmutableValue::Boolean(true);
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

    let left_value = ImmutableValue::Boolean(true);
    let right_value = ImmutableValue::Variable(ImmutableVariable::scalar("isOnline", 25.into()));
    let null = null();
    let expected = mismatch(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn match_with_empty_array() {
    let source_code = r#"
         (match variable []
            (null)
         )
        "#;

    let left_value = ImmutableValue::Variable(ImmutableVariable::scalar("variable", 17.into()));
    let right_value = ImmutableValue::EmptyArray;
    let instr = null();
    let expected = match_(left_value, right_value, instr);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);

    let source_code = r#"
         (match [] variable
            (null)
         )
        "#;

    let left_value = ImmutableValue::EmptyArray;
    let right_value = ImmutableValue::Variable(ImmutableVariable::scalar("variable", 20.into()));
    let instr = null();
    let expected = match_(left_value, right_value, instr);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn mismatch_with_empty_array() {
    let source_code = r#"
         (mismatch variable []
            (null)
         )
        "#;

    let left_value = ImmutableValue::Variable(ImmutableVariable::scalar("variable", 20.into()));
    let right_value = ImmutableValue::EmptyArray;
    let instr = null();
    let expected = mismatch(left_value, right_value, instr);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);

    let source_code = r#"
         (mismatch [] variable
            (null)
         )
        "#;

    let left_value = ImmutableValue::EmptyArray;
    let right_value = ImmutableValue::Variable(ImmutableVariable::scalar("variable", 23.into()));
    let instr = null();
    let expected = mismatch(left_value, right_value, instr);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn match_with_canon_stream_wl() {
    let source_code = r#"
         (match #%canon.$.left_key #%canon.$.right_key
            (null)
         )
        "#;

    let left_lambda = LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
        field_name: "left_key",
    }])
    .unwrap();
    let left = ImmutableValue::VariableWithLambda(ImmutableVariableWithLambda::canon_stream_map(
        "#%canon",
        left_lambda,
        17.into(),
    ));

    let right_lambda = LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
        field_name: "right_key",
    }])
    .unwrap();
    let right = ImmutableValue::VariableWithLambda(ImmutableVariableWithLambda::canon_stream_map(
        "#%canon",
        right_lambda,
        36.into(),
    ));

    let instr = null();
    let expected = match_(left, right, instr);

    let instruction = parse(source_code);
    assert_eq!(
        instruction, expected,
        "actual:\n{:#?}\n expected {:#?}",
        instruction, expected
    );
}

#[test]
fn mismatch_with_canon_stream_wl() {
    let source_code = r#"
         (mismatch #%canon.$.left_key #%canon.$.right_key
            (null)
         )
        "#;

    let left_lambda = LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
        field_name: "left_key",
    }])
    .unwrap();
    let left = ImmutableValue::VariableWithLambda(ImmutableVariableWithLambda::canon_stream_map(
        "#%canon",
        left_lambda,
        20.into(),
    ));

    let right_lambda = LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
        field_name: "right_key",
    }])
    .unwrap();
    let right = ImmutableValue::VariableWithLambda(ImmutableVariableWithLambda::canon_stream_map(
        "#%canon",
        right_lambda,
        39.into(),
    ));

    let instr = null();
    let expected = mismatch(left, right, instr);

    let instruction = parse(source_code);
    assert_eq!(
        instruction, expected,
        "actual:\n{:#?}\n expected {:#?}",
        instruction, expected
    );
}
