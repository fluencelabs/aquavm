/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::ast;
use crate::parser::lexer::LastErrorPath;
use crate::parser::AIRParser;
use crate::parser::ParserError;
use ast::AstVariable::Scalar;
use ast::AstVariable::Stream;
use ast::Call;
use ast::CallInstrArgValue;
use ast::CallInstrValue;
use ast::Instruction;

use air_lambda_parser::ValueAccessor;

use fstrings::f;
use lalrpop_util::ParseError;
use std::rc::Rc;

thread_local!(static TEST_PARSER: AIRParser = AIRParser::new());

fn parse(source_code: &str) -> Instruction {
    *TEST_PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = crate::parser::AIRLexer::new(source_code);
        let mut validator = crate::parser::VariableValidator::new();
        parser
            .parse(source_code, &mut errors, &mut validator, lexer)
            .expect("parsing should be successful")
    })
}

#[test]
fn parse_seq() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (call peerid function [[] []] output)
            (call "id" "f" ["hello" [] name])
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Variable(Scalar("peerid"))),
            function_part: FuncName(CallInstrValue::Variable(Scalar("function"))),
            args: Rc::new(vec![
                CallInstrArgValue::EmptyArray,
                CallInstrArgValue::EmptyArray,
            ]),
            output: Variable(Scalar("output")),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal("id")),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![
                CallInstrArgValue::Literal("hello"),
                CallInstrArgValue::EmptyArray,
                CallInstrArgValue::Variable(Scalar("name")),
            ]),
            output: None,
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_seq_seq() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (seq
                (call peerid function [])
                (call (peerid serviceA) ("serviceB" function) [])
            )
            (call "id" "f" ["hello" name] $output)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        seq(
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Variable(Scalar("peerid"))),
                function_part: FuncName(CallInstrValue::Variable(Scalar("function"))),
                args: Rc::new(vec![]),
                output: None,
            }),
            Instruction::Call(Call {
                peer_part: PeerPkWithServiceId(
                    CallInstrValue::Variable(Scalar("peerid")),
                    CallInstrValue::Variable(Scalar("serviceA")),
                ),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("serviceB"),
                    CallInstrValue::Variable(Scalar("function")),
                ),
                args: Rc::new(vec![]),
                output: None,
            }),
        ),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal("id")),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![
                CallInstrArgValue::Literal("hello"),
                CallInstrArgValue::Variable(Scalar("name")),
            ]),
            output: Variable(Stream("$output")),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call id.$.a! "f" ["hello" name] $void)
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::VariableWithLambda(
            ast::VariableWithLambda::from_raw_algebras(
                Scalar("id"),
                vec![ValueAccessor::FieldAccess { field_name: "a" }],
            ),
        )),
        function_part: FuncName(CallInstrValue::Literal("f")),
        args: Rc::new(vec![
            CallInstrArgValue::Literal("hello"),
            CallInstrArgValue::Variable(Scalar("name")),
        ]),
        output: Variable(Stream("$void")),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn parse_empty_array() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call id "f" ["" [] arg])
        "#;
    let actual = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::Variable(ast::AstVariable::Scalar("id"))),
        function_part: FuncName(CallInstrValue::Literal("f")),
        args: Rc::new(vec![
            CallInstrArgValue::Literal(""),
            CallInstrArgValue::EmptyArray,
            CallInstrArgValue::Variable(Scalar("arg")),
        ]),
        output: None,
    });

    assert_eq!(actual, expected);
}

#[test]
fn parse_empty_array_2() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call id "f" [k [] []])
        "#;
    let actual = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::Variable(ast::AstVariable::Scalar("id"))),
        function_part: FuncName(CallInstrValue::Literal("f")),
        args: Rc::new(vec![
            CallInstrArgValue::Variable(ast::AstVariable::Scalar("k")),
            CallInstrArgValue::EmptyArray,
            CallInstrArgValue::EmptyArray,
        ]),
        output: None,
    });

    assert_eq!(actual, expected);
}

#[test]
fn parse_undefined_variable() {
    let source_code = r#"
        (call id.$.a "f" ["hello" name] $void)
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = super::VariableValidator::new();
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
fn parse_undefined_iterable() {
    let source_code = r#"
        (seq
            (call "" ("" "") [] iterable)
            (fold iterable i
                (seq
                    (call "" ("" "") ["hello" ""] $void)
                    (next j)
                )
            )
        )
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = super::VariableValidator::new();
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

    assert!(matches!(parser_error, ParserError::UndefinedIterable(..)));
}

#[test]
fn parse_undefined_stream_without_json_path() {
    let source_code = r#"
        (call "" "" [$stream])
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = super::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shouldn't fail");

    let errors = validator.finalize();

    assert!(errors.is_empty());
}

#[test]
fn parse_undefined_stream_with_json_path() {
    let source_code = r#"
        (call "" "" [$stream.$.json_path])
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = super::VariableValidator::new();
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
fn parse_json_path_complex() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (call m.$.[1]! "f" [] void)
            (call m.$.abc[0].cde[1][0].cde[1]! "f" [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::VariableWithLambda(
                ast::VariableWithLambda::from_raw_algebras(
                    Scalar("m"),
                    vec![ValueAccessor::ArrayAccess { idx: 1 }],
                ),
            )),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![]),
            output: Variable(Scalar("void")),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::VariableWithLambda(
                ast::VariableWithLambda::from_raw_algebras(
                    Scalar("m"),
                    vec![
                        ValueAccessor::FieldAccess { field_name: "abc" },
                        ValueAccessor::ArrayAccess { idx: 0 },
                        ValueAccessor::FieldAccess { field_name: "cde" },
                        ValueAccessor::ArrayAccess { idx: 1 },
                        ValueAccessor::ArrayAccess { idx: 0 },
                        ValueAccessor::FieldAccess { field_name: "cde" },
                        ValueAccessor::ArrayAccess { idx: 1 },
                    ],
                ),
            )),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![]),
            output: Variable(Scalar("void")),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn json_path_square_braces() {
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call u.$.peer_id! ("return" "") [u.$[1].cde[0][0].abc u.$.name] $void)
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::VariableWithLambda(
            ast::VariableWithLambda::from_raw_algebras(
                Scalar("u"),
                vec![ValueAccessor::FieldAccess {
                    field_name: "peer_id",
                }],
            ),
        )),
        function_part: ServiceIdWithFuncName(
            CallInstrValue::Literal("return"),
            CallInstrValue::Literal(""),
        ),
        args: Rc::new(vec![
            CallInstrArgValue::VariableWithLambda(ast::VariableWithLambda::from_raw_algebras(
                Scalar("u"),
                vec![
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::FieldAccess { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccess { field_name: "abc" },
                ],
            )),
            CallInstrArgValue::VariableWithLambda(ast::VariableWithLambda::from_raw_algebras(
                Scalar("u"),
                vec![ValueAccessor::FieldAccess { field_name: "name" }],
            )),
        ]),
        output: Variable(Stream("$void")),
    });

    assert_eq!(instruction, expected);
}

#[test]
fn parse_null() {
    let source_code = r#"
        (seq
            (null)
            
            ( null     )
        )
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Seq(ast::Seq(Box::new(null()), Box::new(null())));
    assert_eq!(instruction, expected)
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

#[test]
fn parse_fold() {
    let source_code = r#"
        (fold iterable i
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = fold_scalar(
        ast::IterableScalarValue::ScalarVariable("iterable"),
        "i",
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match() {
    use ast::MatchableValue::Variable;

    let source_code = r#"
        (match v1 v2
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = match_(Variable(Scalar("v1")), Variable(Scalar("v2")), null());
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match_with_init_peer_id() {
    use ast::MatchableValue::InitPeerId;
    use ast::MatchableValue::Variable;

    let source_code = r#"
        (match v1 %init_peer_id%
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = match_(Variable(Scalar("v1")), InitPeerId, null());
    assert_eq!(instruction, expected);
}

#[test]
fn parse_mismatch() {
    use ast::MatchableValue::Variable;

    let source_code = r#"
        (mismatch v1 v2
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = mismatch(Variable(Scalar("v1")), Variable(Scalar("v2")), null());
    assert_eq!(instruction, expected);
}

fn source_fold_with(name: &str) -> String {
    f!(r#"(fold iterable i
            ({name} (null) (null))
        )"#)
}
#[test]
fn parse_fold_with_xor_par_seq() {
    for name in &["xor", "par", "seq"] {
        let source_code = source_fold_with(name);
        let instruction = parse(&source_code);
        let instr = binary_instruction(*name);
        let expected = fold_scalar(
            ast::IterableScalarValue::ScalarVariable("iterable"),
            "i",
            instr(null(), null()),
        );
        assert_eq!(instruction, expected);
    }
}

#[test]
fn parse_init_peer_id() {
    use ast::Call;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let peer_id = String::from("some_peer_id");
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
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
            function_part: ServiceIdWithFuncName(
                CallInstrValue::Literal("local_service_id"),
                CallInstrValue::Literal("local_fn_name"),
            ),
            args: Rc::new(vec![]),
            output: None,
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::InitPeerId),
            function_part: ServiceIdWithFuncName(
                CallInstrValue::Literal("service_id"),
                CallInstrValue::Literal("fn_name"),
            ),
            args: Rc::new(vec![]),
            output: None,
        }),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn parse_last_error() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = format!(
        r#"
        (seq
            (call %init_peer_id% ("service_id" "fn_name") [%last_error%])
            (null)
        )"#,
    );

    let instruction = parse(&source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::InitPeerId),
            function_part: ServiceIdWithFuncName(
                CallInstrValue::Literal("service_id"),
                CallInstrValue::Literal("fn_name"),
            ),
            args: Rc::new(vec![CallInstrArgValue::LastError(LastErrorPath::None)]),
            output: None,
        }),
        Instruction::Null(ast::Null),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_par_call() {
    use ast::Call;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let peer_id = String::from("some_peer_id");
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
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("local_service_id"),
                    CallInstrValue::Literal("local_fn_name"),
                ),
                args: Rc::new(vec![]),
                output: Variable(Scalar("result_1")),
            }),
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("service_id"),
                    CallInstrValue::Literal("fn_name"),
                ),
                args: Rc::new(vec![]),
                output: Variable(Scalar("g")),
            }),
        ),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
            function_part: ServiceIdWithFuncName(
                CallInstrValue::Literal("local_service_id"),
                CallInstrValue::Literal("local_fn_name"),
            ),
            args: Rc::new(vec![]),
            output: Variable(Scalar("result_2")),
        }),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_with_empty_and_dash() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

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
                Instruction::Call(Call {
                    peer_part: PeerPk(CallInstrValue::Literal("set_variables")),
                    function_part: ServiceIdWithFuncName(
                        CallInstrValue::Literal(""),
                        CallInstrValue::Literal(""),
                    ),
                    args: Rc::new(vec![CallInstrArgValue::Literal("module-bytes")]),
                    output: Variable(Scalar("module-bytes")),
                }),
                Instruction::Call(Call {
                    peer_part: PeerPk(CallInstrValue::Literal("set_variables")),
                    function_part: ServiceIdWithFuncName(
                        CallInstrValue::Literal(""),
                        CallInstrValue::Literal(""),
                    ),
                    args: Rc::new(vec![CallInstrArgValue::Literal("module_config")]),
                    output: Variable(Scalar("module_config")),
                }),
            ),
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal("set_variables")),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal(""),
                    CallInstrValue::Literal(""),
                ),
                args: Rc::new(vec![CallInstrArgValue::Literal("blueprint")]),
                output: Variable(Scalar("blueprint")),
            }),
        ),
        seq(
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal("A")),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("add_module"),
                    CallInstrValue::Literal(""),
                ),
                args: Rc::new(vec![
                    CallInstrArgValue::Variable(Scalar("module-bytes")),
                    CallInstrArgValue::Variable(Scalar("module_config")),
                ]),
                output: Variable(Scalar("module")),
            }),
            seq(
                Instruction::Call(Call {
                    peer_part: PeerPk(CallInstrValue::Literal("A")),
                    function_part: ServiceIdWithFuncName(
                        CallInstrValue::Literal("add_blueprint"),
                        CallInstrValue::Literal(""),
                    ),
                    args: Rc::new(vec![CallInstrArgValue::Variable(Scalar("blueprint"))]),
                    output: Variable(Scalar("blueprint_id")),
                }),
                seq(
                    Instruction::Call(Call {
                        peer_part: PeerPk(CallInstrValue::Literal("A")),
                        function_part: ServiceIdWithFuncName(
                            CallInstrValue::Literal("create"),
                            CallInstrValue::Literal(""),
                        ),
                        args: Rc::new(vec![CallInstrArgValue::Variable(Scalar("blueprint_id"))]),
                        output: Variable(Scalar("service_id")),
                    }),
                    Instruction::Call(Call {
                        peer_part: PeerPk(CallInstrValue::Literal("remote_peer_id")),
                        function_part: ServiceIdWithFuncName(
                            CallInstrValue::Literal(""),
                            CallInstrValue::Literal(""),
                        ),
                        args: Rc::new(vec![CallInstrArgValue::Variable(Scalar("service_id"))]),
                        output: Variable(Scalar("client_result")),
                    }),
                ),
            ),
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn match_with_bool() {
    use ast::MatchableValue::*;

    let source_code = r#"
         (match isOnline true
            (null)
         )
        "#;

    let left_value = Variable(Scalar("isOnline"));
    let right_value = Boolean(true);
    let null = null();
    let expected = match_(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn mismatch_with_bool() {
    use ast::MatchableValue::*;

    let source_code = r#"
         (mismatch true isOnline
            (null)
         )
        "#;

    let left_value = Boolean(true);
    let right_value = Variable(Scalar("isOnline"));
    let null = null();
    let expected = mismatch(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn no_output() {
    use ast::Call;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call peer (service fname) [])
    "#;
    let actual = parse(source_code);

    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::Variable(Scalar("peer"))),
        function_part: ServiceIdWithFuncName(
            CallInstrValue::Variable(Scalar("service")),
            CallInstrValue::Variable(Scalar("fname")),
        ),
        args: Rc::new(vec![]),
        output: None,
    });
    assert_eq!(actual, expected);
}

#[test]
fn ap_with_literal() {
    use ast::Ap;

    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = Instruction::Ap(Ap {
        argument: ast::ApArgument::Literal("some_string"),
        result: ast::AstVariable::Stream("$stream"),
    });

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_number() {
    use ast::Ap;
    use ast::Number;

    let source_code = r#"
        (ap -100 $stream)
    "#;

    let actual = parse(source_code);
    let expected = Instruction::Ap(Ap {
        argument: ast::ApArgument::Number(Number::Int(-100)),
        result: ast::AstVariable::Stream("$stream"),
    });

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_bool() {
    use ast::Ap;

    let source_code = r#"
        (ap true $stream)
    "#;

    let actual = parse(source_code);
    let expected = Instruction::Ap(Ap {
        argument: ast::ApArgument::Boolean(true),
        result: ast::AstVariable::Stream("$stream"),
    });

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_last_error() {
    use ast::Ap;
    use ast::LastErrorPath;

    let source_code = r#"
        (ap %last_error%.$.msg! $stream)
    "#;

    let actual = parse(source_code);
    let expected = Instruction::Ap(Ap {
        argument: ast::ApArgument::LastError(LastErrorPath::Message),
        result: ast::AstVariable::Stream("$stream"),
    });

    assert_eq!(actual, expected);
}

#[test]
fn fold_json_path() {
    use ast::FoldScalar;
    use ast::IterableScalarValue;

    let source_code = r#"
    ; comment
    (fold members.$.[123321] m (null)) ;;; comment
    ;;; comment
    "#;
    let instruction = parse(source_code);
    let expected = Instruction::FoldScalar(FoldScalar {
        iterable: IterableScalarValue::new_vl(
            "members",
            vec![ValueAccessor::ArrayAccess { idx: 123321 }],
        ),
        iterator: "m",
        instruction: Rc::new(null()),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream() {
    use ast::FoldStream;

    let source_code = r#"
        (fold $stream iterator (null))
    "#;
    let instruction = parse(source_code);
    let expected = Instruction::FoldStream(FoldStream {
        stream_name: "$stream",
        iterator: "iterator",
        instruction: Rc::new(null()),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn comments() {
    use ast::FoldScalar;
    use ast::IterableScalarValue;

    let source_code = r#"
    ; comment
    (fold members.$.field[1] m (null)) ;;; comment ;;?()()
    ;;; comme;?!.$.  nt[][][][()()()null;$::!
    "#;
    let instruction = parse(source_code);
    let expected = Instruction::FoldScalar(FoldScalar {
        iterable: IterableScalarValue::new_vl(
            "members",
            vec![
                ValueAccessor::FieldAccess {
                    field_name: "field",
                },
                ValueAccessor::ArrayAccess { idx: 1 },
            ],
        ),
        iterator: "m",
        instruction: Rc::new(null()),
    });
    assert_eq!(instruction, expected);
}

// Test DSL

fn seq<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Seq(ast::Seq(Box::new(l), Box::new(r)))
}

fn par<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Par(ast::Par(Box::new(l), Box::new(r)))
}

fn xor<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Xor(ast::Xor(Box::new(l), Box::new(r)))
}

fn seqnn() -> Instruction<'static> {
    seq(null(), null())
}

fn null() -> Instruction<'static> {
    Instruction::Null(ast::Null)
}

fn fold_scalar<'a>(
    iterable: ast::IterableScalarValue<'a>,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::FoldScalar(ast::FoldScalar {
        iterable,
        iterator,
        instruction: std::rc::Rc::new(instruction),
    })
}

fn match_<'a>(
    left_value: ast::MatchableValue<'a>,
    right_value: ast::MatchableValue<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::Match(ast::Match {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

fn mismatch<'a>(
    left_value: ast::MatchableValue<'a>,
    right_value: ast::MatchableValue<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::MisMatch(ast::MisMatch {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

fn binary_instruction<'a, 'b>(
    name: &'a str,
) -> impl Fn(Instruction<'b>, Instruction<'b>) -> Instruction<'b> {
    match name {
        "xor" => |l, r| xor(l, r),
        "par" => |l, r| par(l, r),
        "seq" => |l, r| seq(l, r),
        _ => unreachable!(),
    }
}
