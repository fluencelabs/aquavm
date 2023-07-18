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

use std::rc::Rc;

#[test]
fn parse_seq() {
    let source_code = r#"
        (seq
            (call peer_id (service_id function_name) [[] []] output)
            (call "peer_id" ("service_id" "function_name") ["hello" [] name])
        )
        "#;

    let instruction = parse(source_code);
    let expected = seq(
        call(
            ResolvableToPeerIdVariable::Scalar(Scalar::new("peer_id", 32.into())),
            ResolvableToStringVariable::Scalar(Scalar::new("service_id", 41.into())),
            ResolvableToStringVariable::Scalar(Scalar::new("function_name", 52.into())),
            Rc::new(vec![ImmutableValue::EmptyArray, ImmutableValue::EmptyArray]),
            CallOutputValue::Scalar(Scalar::new("output", 75.into())),
        ),
        call(
            ResolvableToPeerIdVariable::Literal("peer_id"),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![
                ImmutableValue::Literal("hello"),
                ImmutableValue::EmptyArray,
                ImmutableValue::Variable(ImmutableVariable::scalar("name", 154.into())),
            ]),
            CallOutputValue::None,
        ),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_seq_seq() {
    let source_code = r#"
        (seq
            (seq
                (call peer_id (service_id function_name) [])
                (call peer_id ("service_B" function_name) [])
            )
            (call "peer_id" ("service_id" "function_name") ["hello" name] $output)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        seq(
            call(
                ResolvableToPeerIdVariable::Scalar(Scalar::new("peer_id", 53.into())),
                ResolvableToStringVariable::Scalar(Scalar::new("service_id", 62.into())),
                ResolvableToStringVariable::Scalar(Scalar::new("function_name", 73.into())),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
            call(
                ResolvableToPeerIdVariable::Scalar(Scalar::new("peer_id", 114.into())),
                ResolvableToStringVariable::Literal("service_B"),
                ResolvableToStringVariable::Scalar(Scalar::new("function_name", 135.into())),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
        ),
        call(
            ResolvableToPeerIdVariable::Literal("peer_id"),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![
                ImmutableValue::Literal("hello"),
                ImmutableValue::Variable(ImmutableVariable::scalar("name", 236.into())),
            ]),
            CallOutputValue::Stream(Stream::new("$output", 242.into())),
        ),
    );
    assert_eq!(instruction, expected);
}

fn source_seq_with(name: &'static str) -> String {
    format!(
        r#"
        (seq
            ({name}
                (seq (null) (null))
                (null)
            )
            ({name}   (null) (seq (null) (null))   )
        )
        "#
    )
}

#[test]
fn parse_seq_par_xor_seq() {
    for name in &["xor", "par", "seq"] {
        let source_code = source_seq_with(name);
        let instruction = parse(&source_code);
        let instr = binary_instruction(name);
        let expected = seq(instr(seqnn(), null()), instr(null(), seqnn()));
        assert_eq!(instruction, expected);
    }
}
