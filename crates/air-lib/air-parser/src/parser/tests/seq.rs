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
            CallInstrValue::Variable(VariableWithLambda::scalar("peer_id", 32)),
            CallInstrValue::Variable(VariableWithLambda::scalar("service_id", 41)),
            CallInstrValue::Variable(VariableWithLambda::scalar("function_name", 52)),
            Rc::new(vec![Value::EmptyArray, Value::EmptyArray]),
            CallOutputValue::Scalar(Scalar::new("output", 75)),
        ),
        call(
            CallInstrValue::Literal("peer_id"),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![
                Value::Literal("hello"),
                Value::EmptyArray,
                Value::Variable(VariableWithLambda::scalar("name", 154)),
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
                CallInstrValue::Variable(VariableWithLambda::scalar("peer_id", 53)),
                CallInstrValue::Variable(VariableWithLambda::scalar("service_id", 62)),
                CallInstrValue::Variable(VariableWithLambda::scalar("function_name", 73)),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
            call(
                CallInstrValue::Variable(VariableWithLambda::scalar("peer_id", 114)),
                CallInstrValue::Literal("service_B"),
                CallInstrValue::Variable(VariableWithLambda::scalar("function_name", 135)),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
        ),
        call(
            CallInstrValue::Literal("peer_id"),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![
                Value::Literal("hello"),
                Value::Variable(VariableWithLambda::scalar("name", 236)),
            ]),
            CallOutputValue::Stream(Stream::new("$output", 242)),
        ),
    );
    assert_eq!(instruction, expected);
}

fn source_seq_with(name: &'static str) -> String {
    f!(r#"
        (seq
            ({name}
                (seq (null) (null))
                (null)
            )
            ({name}   (null) (seq (null) (null))   )
        )
        "#)
}

#[test]
fn parse_seq_par_xor_seq() {
    for name in &["xor", "par", "seq"] {
        let source_code = source_seq_with(name);
        let instruction = parse(&source_code);
        let instr = binary_instruction(*name);
        let expected = seq(instr(seqnn(), null()), instr(null(), seqnn()));
        assert_eq!(instruction, expected);
    }
}
