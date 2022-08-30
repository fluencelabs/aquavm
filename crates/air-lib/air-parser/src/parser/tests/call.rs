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

use air_lambda_ast::{LambdaAST, ValueAccessor};
use fstrings::f;
use fstrings::format_args_f;
use lalrpop_util::ParseError;

use std::rc::Rc;

#[test]
fn parse_json_path() {
    let source_code = r#"
        (call peer_id.$.a! ("service_id" "function_name") ["hello" name] $void)
        "#;

    let instruction = parse(source_code);
    let expected = call(
        ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
            "peer_id",
            vec![ValueAccessor::FieldAccessByName { field_name: "a" }],
            15,
        )),
        ResolvableToStringVariable::Literal("service_id"),
        ResolvableToStringVariable::Literal("function_name"),
        Rc::new(vec![
            ImmutableValue::Literal("hello"),
            ImmutableValue::Variable(ImmutableVariable::scalar("name", 68)),
        ]),
        CallOutputValue::Stream(Stream::new("$void", 74)),
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
        ResolvableToStringVariable::Scalar(Scalar::new("peer_id", 15)),
        ResolvableToStringVariable::Scalar(Scalar::new("service_id", 24)),
        ResolvableToStringVariable::Literal("function_name"),
        Rc::new(vec![
            ImmutableValue::Literal(""),
            ImmutableValue::EmptyArray,
            ImmutableValue::Variable(ImmutableVariable::scalar("arg", 59)),
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
        ResolvableToStringVariable::Scalar(Scalar::new("peer_id", 15)),
        ResolvableToStringVariable::Literal("service_id"),
        ResolvableToStringVariable::Literal("function_name"),
        Rc::new(vec![
            ImmutableValue::Variable(ImmutableVariable::scalar("k", 55)),
            ImmutableValue::EmptyArray,
            ImmutableValue::EmptyArray,
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

        assert!(matches!(
            parser_error,
            ParserError::UndefinedVariable { .. }
        ));
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
fn parse_lambda_complex() {
    let source_code = r#"
        (seq
            (call m.$.[1]! ("service_id" "function_name") [] void)
            (call m.$.abc[0].cde[1][0].cde[1]! ("service_id" "function_name") [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        call(
            ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
                "m",
                vec![ValueAccessor::ArrayAccess { idx: 1 }],
                32,
            )),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Scalar(Scalar::new("void", 75)),
        ),
        call(
            ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
                "m",
                vec![
                    ValueAccessor::FieldAccessByName { field_name: "abc" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccessByName { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccessByName { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                ],
                99,
            )),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Scalar(Scalar::new("void", 162)),
        ),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_lambda_with_scalars_complex() {
    let source_code = r#"
        (seq
            (call m.$.[1].[scalar_1].[scalar_2]! ("service_id" "function_name") [] void)
            (call m.$.abc[0].[scalar_2].cde[1][0][scalar_3].cde[1]! ("service_id" "function_name") [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        call(
            ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
                "m",
                vec![
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::FieldAccessByScalar {
                        scalar_name: "scalar_1",
                    },
                    ValueAccessor::FieldAccessByScalar {
                        scalar_name: "scalar_2",
                    },
                ],
                32,
            )),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Scalar(Scalar::new("void", 97)),
        ),
        call(
            ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
                "m",
                vec![
                    ValueAccessor::FieldAccessByName { field_name: "abc" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccessByScalar {
                        scalar_name: "scalar_2",
                    },
                    ValueAccessor::FieldAccessByName { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccessByScalar {
                        scalar_name: "scalar_3",
                    },
                    ValueAccessor::FieldAccessByName { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 1 },
                ],
                121,
            )),
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Scalar(Scalar::new("void", 205)),
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
        ResolvableToStringVariable::ScalarWithLambda(ScalarWithLambda::from_raw_lambda(
            "u",
            vec![ValueAccessor::FieldAccessByName {
                field_name: "peer_id",
            }],
            15,
        )),
        ResolvableToStringVariable::Literal("return"),
        ResolvableToStringVariable::Literal(""),
        Rc::new(vec![
            ImmutableValue::VariableWithLambda(
                ImmutableVariableWithLambda::from_raw_lambda_scalar(
                    "u",
                    vec![
                        ValueAccessor::ArrayAccess { idx: 1 },
                        ValueAccessor::FieldAccessByName { field_name: "cde" },
                        ValueAccessor::ArrayAccess { idx: 0 },
                        ValueAccessor::ArrayAccess { idx: 0 },
                        ValueAccessor::FieldAccessByName { field_name: "abc" },
                    ],
                    43,
                ),
            ),
            ImmutableValue::VariableWithLambda(
                ImmutableVariableWithLambda::from_raw_lambda_scalar(
                    "u",
                    vec![ValueAccessor::FieldAccessByName { field_name: "name" }],
                    64,
                ),
            ),
        ]),
        CallOutputValue::Stream(Stream::new("$void", 74)),
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
            ResolvableToStringVariable::Literal(peer_id),
            ResolvableToStringVariable::Literal("local_service_id"),
            ResolvableToStringVariable::Literal("local_fn_name"),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
        call(
            ResolvableToStringVariable::InitPeerId,
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("fn_name"),
            Rc::new(vec![]),
            CallOutputValue::None,
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn parse_timestamp() {
    let source_code = r#"
        (call "peer_id" ("service_id" "fn_name") [%timestamp%])
        "#;

    let instruction = parse(source_code);
    let expected = call(
        ResolvableToStringVariable::Literal("peer_id"),
        ResolvableToStringVariable::Literal("service_id"),
        ResolvableToStringVariable::Literal("fn_name"),
        Rc::new(vec![ImmutableValue::Timestamp]),
        CallOutputValue::None,
    );

    assert_eq!(instruction, expected);
}

#[test]
fn parse_ttl() {
    let source_code = r#"
        (call "peer_id" ("service_id" "fn_name") [%ttl%])
        "#;

    let instruction = parse(source_code);
    let expected = call(
        ResolvableToStringVariable::Literal("peer_id"),
        ResolvableToStringVariable::Literal("service_id"),
        ResolvableToStringVariable::Literal("fn_name"),
        Rc::new(vec![ImmutableValue::TTL]),
        CallOutputValue::None,
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
            ResolvableToStringVariable::InitPeerId,
            ResolvableToStringVariable::Literal("service_id"),
            ResolvableToStringVariable::Literal("fn_name"),
            Rc::new(vec![ImmutableValue::LastError(None)]),
            CallOutputValue::None,
        ),
        null(),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn canon_stream_in_args() {
    let service_id = "service_id";
    let function_name = "function_name";
    let canon_stream = "#canon_stream";
    let source_code = f!(r#"
            (call %init_peer_id% ("{service_id}" "{function_name}") [{canon_stream}])
        "#);

    let instruction = parse(&source_code);
    let expected = call(
        ResolvableToStringVariable::InitPeerId,
        ResolvableToStringVariable::Literal(service_id),
        ResolvableToStringVariable::Literal(function_name),
        Rc::new(vec![ImmutableValue::Variable(
            ImmutableVariable::canon_stream(canon_stream, 66),
        )]),
        CallOutputValue::None,
    );

    assert_eq!(instruction, expected);
}

#[test]
fn canon_stream_in_triplet() {
    let service_id = "service_id";
    let function_name = "function_name";
    let canon_stream = "#canon_stream";
    let canon_stream_lambda = ".$.[0].path!";
    let source_code = f!(r#"
            (call {canon_stream}{canon_stream_lambda} ("{service_id}" "{function_name}") [])
        "#);

    let instruction = parse(&source_code);
    let expected = call(
        ResolvableToStringVariable::CanonStreamWithLambda(CanonStreamWithLambda::new(
            canon_stream,
            unsafe {
                LambdaAST::new_unchecked(vec![
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccessByName { field_name: "path" },
                ])
            },
            19,
        )),
        ResolvableToStringVariable::Literal(service_id),
        ResolvableToStringVariable::Literal(function_name),
        Rc::new(vec![]),
        CallOutputValue::None,
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_par_call() {
    let peer_id = "some_peer_id";
    let source_code = f!(r#"
        (seq
            (par
                (call "{peer_id}" ("local_service_id" "local_fn_name") [] result_1)
                (call "{peer_id}" ("service_id" "fn_name") [] g)
            )
            (call "{peer_id}" ("local_service_id" "local_fn_name") [] result_2)
        )"#);

    let instruction = parse(&source_code);
    let expected = seq(
        par(
            call(
                ResolvableToStringVariable::Literal(peer_id),
                ResolvableToStringVariable::Literal("local_service_id"),
                ResolvableToStringVariable::Literal("local_fn_name"),
                Rc::new(vec![]),
                CallOutputValue::Scalar(Scalar::new("result_1", 108)),
            ),
            call(
                ResolvableToStringVariable::Literal(peer_id),
                ResolvableToStringVariable::Literal("service_id"),
                ResolvableToStringVariable::Literal("fn_name"),
                Rc::new(vec![]),
                CallOutputValue::Scalar(Scalar::new("g", 183)),
            ),
        ),
        call(
            ResolvableToStringVariable::Literal(peer_id),
            ResolvableToStringVariable::Literal("local_service_id"),
            ResolvableToStringVariable::Literal("local_fn_name"),
            Rc::new(vec![]),
            CallOutputValue::Scalar(Scalar::new("result_2", 273)),
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
                    ResolvableToStringVariable::Literal("set_variables"),
                    ResolvableToStringVariable::Literal(""),
                    ResolvableToStringVariable::Literal(""),
                    Rc::new(vec![ImmutableValue::Literal("module-bytes")]),
                    CallOutputValue::Scalar(Scalar::new("module-bytes", 119)),
                ),
                call(
                    ResolvableToStringVariable::Literal("set_variables"),
                    ResolvableToStringVariable::Literal(""),
                    ResolvableToStringVariable::Literal(""),
                    Rc::new(vec![ImmutableValue::Literal("module_config")]),
                    CallOutputValue::Scalar(Scalar::new("module_config", 201)),
                ),
            ),
            call(
                ResolvableToStringVariable::Literal("set_variables"),
                ResolvableToStringVariable::Literal(""),
                ResolvableToStringVariable::Literal(""),
                Rc::new(vec![ImmutableValue::Literal("blueprint")]),
                CallOutputValue::Scalar(Scalar::new("blueprint", 294)),
            ),
        ),
        seq(
            call(
                ResolvableToStringVariable::Literal("A"),
                ResolvableToStringVariable::Literal("add_module"),
                ResolvableToStringVariable::Literal(""),
                Rc::new(vec![
                    ImmutableValue::Variable(ImmutableVariable::scalar("module-bytes", 381)),
                    ImmutableValue::Variable(ImmutableVariable::scalar("module_config", 394)),
                ]),
                CallOutputValue::Scalar(Scalar::new("module", 409)),
            ),
            seq(
                Instruction::Call(Call {
                    triplet: Triplet {
                        peer_pk: ResolvableToStringVariable::Literal("A"),
                        service_id: ResolvableToStringVariable::Literal("add_blueprint"),
                        function_name: ResolvableToStringVariable::Literal(""),
                    },
                    args: Rc::new(vec![ImmutableValue::Variable(ImmutableVariable::scalar(
                        "blueprint",
                        490,
                    ))]),
                    output: CallOutputValue::Scalar(Scalar::new("blueprint_id", 501)),
                }),
                seq(
                    call(
                        ResolvableToStringVariable::Literal("A"),
                        ResolvableToStringVariable::Literal("create"),
                        ResolvableToStringVariable::Literal(""),
                        Rc::new(vec![ImmutableValue::Variable(ImmutableVariable::scalar(
                            "blueprint_id",
                            589,
                        ))]),
                        CallOutputValue::Scalar(Scalar::new("service_id", 603)),
                    ),
                    call(
                        ResolvableToStringVariable::Literal("remote_peer_id"),
                        ResolvableToStringVariable::Literal(""),
                        ResolvableToStringVariable::Literal(""),
                        Rc::new(vec![ImmutableValue::Variable(ImmutableVariable::scalar(
                            "service_id",
                            671,
                        ))]),
                        CallOutputValue::Scalar(Scalar::new("client_result", 683)),
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
        ResolvableToStringVariable::Scalar(Scalar::new("peer", 15)),
        ResolvableToStringVariable::Scalar(Scalar::new("service", 21)),
        ResolvableToStringVariable::Scalar(Scalar::new("fname", 29)),
        Rc::new(vec![]),
        CallOutputValue::None,
    );
    assert_eq!(actual, expected);
}

#[test]
fn not_defined_scalar_in_lambda() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] value)
            (call "" ("" "") [value.$.[not_defined_scalar]])
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
        ParserError::UndefinedVariable { .. }
    ));
}
