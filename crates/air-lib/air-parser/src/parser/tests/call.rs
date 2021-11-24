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

use air_lambda_ast::ValueAccessor;
use lalrpop_util::ParseError;

use std::rc::Rc;

#[test]
fn parse_json_path() {
    let source_code = r#"
        (call peer_id.$.a! ("service_id" "function_name") ["hello" name] $void)
        "#;

    let instruction = parse(source_code);
    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
            "peer_id",
            vec![ValueAccessor::FieldAccess { field_name: "a" }],
            15,
        )),
        CallInstrValue::Literal("service_id"),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            Value::Literal("hello"),
            Value::Variable(VariableWithLambda::scalar("name", 68)),
        ]),
        CallOutputValue::Variable(Variable::stream("$void", 74)),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_empty_array() {
    let source_code = r#"
        (call peer_id (service_id "function_name") ["" [] arg])
    "#;

    let actual = parse(source_code);
    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::scalar("peer_id", 15)),
        CallInstrValue::Variable(VariableWithLambda::scalar("service_id", 24)),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            Value::Literal(""),
            Value::EmptyArray,
            Value::Variable(VariableWithLambda::scalar("arg", 59)),
        ]),
        CallOutputValue::None,
    );

    assert_eq!(actual, expected);
}

#[test]
fn parse_empty_array_2() {
    let source_code = r#"
        (call peer_id ("service_id" "function_name") [k [] []])
        "#;

    let actual = parse(source_code);
    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::scalar("peer_id", 15)),
        CallInstrValue::Literal("service_id"),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            Value::Variable(VariableWithLambda::scalar("k", 55)),
            Value::EmptyArray,
            Value::EmptyArray,
        ]),
        CallOutputValue::None,
    );

    assert_eq!(actual, expected);
}

#[test]
fn parse_undefined_variable() {
    let source_code = r#"
        (call id.$.a ("" "f") ["hello" name] $void)
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    let errors = validator.finalize();

    assert_eq!(errors.len(), 2);
    for i in 0..2 {
        let error = &errors[i].error;
        let parser_error = match error {
            ParseError::User { error } => error,
            _ => panic!("unexpected error type"),
        };

        assert!(matches!(parser_error, ParserError::UndefinedVariable(..)));
    }
}

#[test]
fn parse_undefined_stream_without_json_path() {
    let source_code = r#"
        (call "" ("" "") [$stream])
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    let errors = validator.finalize();

    assert!(errors.is_empty());
}

#[test]
fn parse_undefined_stream_with_lambda() {
    let source_code = r#"
        (call "" ("" "") [$stream.$.json_path])
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

    assert!(matches!(parser_error, ParserError::UndefinedVariable(..)));
}

#[test]
fn parse_call_with_invalid_triplet() {
    let source_code = r#"
        (call "" "" [$stream.$.json_path])
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = crate::parser::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    assert_eq!(errors.len(), 1);
    let error = &errors[0].error;
    let parser_error = match error {
        ParseError::User { error } => error,
        _ => panic!("unexpected error type"),
    };

    assert!(matches!(parser_error, ParserError::InvalidCallTriplet(..)));
}

#[test]
fn parse_json_path_complex() {
    let source_code = r#"
        (seq
            (call m.$.[1]! ("service_id" "function_name") [] void)
            (call m.$.abc[0].cde[1][0].cde[1]! ("service_id" "function_name") [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        call(
            CallInstrValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "m",
                vec![ValueAccessor::ArrayAccess { idx: 1 }],
                32,
            )),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("void", 75)),
        ),
        call(
            CallInstrValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "m",
                vec![
                    ValueAccessor::FieldAccess { field_name: "abc" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccess { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccess { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                ],
                99,
            )),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("void", 162)),
        ),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn json_path_square_braces() {
    let source_code = r#"
        (call u.$.peer_id! ("return" "") [u.$[1].cde[0][0].abc u.$.name] $void)
        "#;
    let instruction = parse(source_code);
    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
            "u",
            vec![ValueAccessor::FieldAccess {
                field_name: "peer_id",
            }],
            15,
        )),
        CallInstrValue::Literal("return"),
        CallInstrValue::Literal(""),
        Rc::new(vec![
            Value::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "u",
                vec![
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::FieldAccess { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccess { field_name: "abc" },
                ],
                43,
            )),
            Value::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "u",
                vec![ValueAccessor::FieldAccess { field_name: "name" }],
                64,
            )),
        ]),
        CallOutputValue::Variable(Variable::stream("$void", 74)),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn parse_init_peer_id() {
    let peer_id = "some_peer_id";
    let source_code = format!(
        r#"
        (seq
            (call "{}" ("local_service_id" "local_fn_name") [])
            (call %init_peer_id% ("service_id" "fn_name") [])
        )"#,
        peer_id
    );

    let instruction = parse(&source_code);
    let expected = seq(
        call(
            CallInstrValue::Literal(peer_id),
            CallInstrValue::Literal("local_service_id"),
            CallInstrValue::Literal("local_fn_name"),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
        call(
            CallInstrValue::InitPeerId,
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("fn_name"),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn parse_last_error() {
    let source_code = format!(
        r#"
        (seq
            (call %init_peer_id% ("service_id" "fn_name") [%last_error%])
            (null)
        )"#,
    );

    let instruction = parse(&source_code);
    let expected = seq(
        call(
            CallInstrValue::InitPeerId,
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("fn_name"),
            Rc::new(vec![Value::LastError(LastErrorPath::None)]),
            CallOutputValue::None,
        ),
        null(),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_par_call() {
    let peer_id = "some_peer_id";
    let source_code = format!(
        r#"
        (seq
            (par
                (call "{0}" ("local_service_id" "local_fn_name") [] result_1)
                (call "{0}" ("service_id" "fn_name") [] g)
            )
            (call "{0}" ("local_service_id" "local_fn_name") [] result_2)
        )"#,
        peer_id,
    );

    let instruction = parse(&source_code);
    let expected = seq(
        par(
            call(
                CallInstrValue::Literal(peer_id),
                CallInstrValue::Literal("local_service_id"),
                CallInstrValue::Literal("local_fn_name"),
                Rc::new(vec![]),
                CallOutputValue::Variable(Variable::scalar("result_1", 108)),
            ),
            call(
                CallInstrValue::Literal(peer_id),
                CallInstrValue::Literal("service_id"),
                CallInstrValue::Literal("fn_name"),
                Rc::new(vec![]),
                CallOutputValue::Variable(Variable::scalar("g", 183)),
            ),
        ),
        call(
            CallInstrValue::Literal(peer_id),
            CallInstrValue::Literal("local_service_id"),
            CallInstrValue::Literal("local_fn_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("result_2", 273)),
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_with_empty_and_dash() {
    let source_code = r#"
        (seq
            (seq
                (seq
                    (call "set_variables" ("" "") ["module-bytes"] module-bytes)
                    (call "set_variables" ("" "") ["module_config"] module_config)
                )
                (call "set_variables" ("" "") ["blueprint"] blueprint)
            )
            (seq
                (call "A" ("add_module" "") [module-bytes module_config] module)
                (seq
                    (call "A" ("add_blueprint" "") [blueprint] blueprint_id)
                    (seq
                        (call "A" ("create" "") [blueprint_id] service_id)
                        (call "remote_peer_id" ("" "") [service_id] client_result)
                    )
                )
            )
        )
        "#;

    let instruction = parse(source_code);
    let expected = seq(
        seq(
            seq(
                call(
                    CallInstrValue::Literal("set_variables"),
                    CallInstrValue::Literal(""),
                    CallInstrValue::Literal(""),
                    Rc::new(vec![Value::Literal("module-bytes")]),
                    CallOutputValue::Variable(Variable::scalar("module-bytes", 119)),
                ),
                call(
                    CallInstrValue::Literal("set_variables"),
                    CallInstrValue::Literal(""),
                    CallInstrValue::Literal(""),
                    Rc::new(vec![Value::Literal("module_config")]),
                    CallOutputValue::Variable(Variable::scalar("module_config", 201)),
                ),
            ),
            call(
                CallInstrValue::Literal("set_variables"),
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                Rc::new(vec![Value::Literal("blueprint")]),
                CallOutputValue::Variable(Variable::scalar("blueprint", 294)),
            ),
        ),
        seq(
            call(
                CallInstrValue::Literal("A"),
                CallInstrValue::Literal("add_module"),
                CallInstrValue::Literal(""),
                Rc::new(vec![
                    Value::Variable(VariableWithLambda::scalar("module-bytes", 381)),
                    Value::Variable(VariableWithLambda::scalar("module_config", 394)),
                ]),
                CallOutputValue::Variable(Variable::scalar("module", 409)),
            ),
            seq(
                Instruction::Call(Call {
                    triplet: Triplet {
                        peer_pk: CallInstrValue::Literal("A"),
                        service_id: CallInstrValue::Literal("add_blueprint"),
                        function_name: CallInstrValue::Literal(""),
                    },
                    args: Rc::new(vec![Value::Variable(VariableWithLambda::scalar(
                        "blueprint",
                        490,
                    ))]),
                    output: CallOutputValue::Variable(Variable::scalar("blueprint_id", 501)),
                }),
                seq(
                    call(
                        CallInstrValue::Literal("A"),
                        CallInstrValue::Literal("create"),
                        CallInstrValue::Literal(""),
                        Rc::new(vec![Value::Variable(VariableWithLambda::scalar(
                            "blueprint_id",
                            589,
                        ))]),
                        CallOutputValue::Variable(Variable::scalar("service_id", 603)),
                    ),
                    call(
                        CallInstrValue::Literal("remote_peer_id"),
                        CallInstrValue::Literal(""),
                        CallInstrValue::Literal(""),
                        Rc::new(vec![Value::Variable(VariableWithLambda::scalar(
                            "service_id",
                            671,
                        ))]),
                        CallOutputValue::Variable(Variable::scalar("client_result", 683)),
                    ),
                ),
            ),
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn no_output() {
    let source_code = r#"
        (call peer (service fname) [])
    "#;

    let actual = parse(source_code);

    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::scalar("peer", 15)),
        CallInstrValue::Variable(VariableWithLambda::scalar("service", 21)),
        CallInstrValue::Variable(VariableWithLambda::scalar("fname", 29)),
        Rc::new(vec![]),
        CallOutputValue::None,
    );
    assert_eq!(actual, expected);
}
