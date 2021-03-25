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
use crate::parser::AIRParser;
use crate::parser::ParserError;
use ast::Instruction;

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
            .expect("parsing should be successfull")
    })
}

#[test]
fn parse_seq() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (call peerid function [] output)
            (call "id" "f" ["hello" name])
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Variable("peerid")),
            function_part: FuncName(CallInstrValue::Variable("function")),
            args: Rc::new(vec![]),
            output: Scalar("output"),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal("id")),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![
                CallInstrArgValue::Literal("hello"),
                CallInstrArgValue::Variable("name"),
            ]),
            output: None,
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_seq_seq() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (seq
                (call peerid function [])
                (call (peerid serviceA) ("serviceB" function) [])
            )
            (call "id" "f" ["hello" name] output[])
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        seq(
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Variable("peerid")),
                function_part: FuncName(CallInstrValue::Variable("function")),
                args: Rc::new(vec![]),
                output: None,
            }),
            Instruction::Call(Call {
                peer_part: PeerPkWithServiceId(
                    CallInstrValue::Variable("peerid"),
                    CallInstrValue::Variable("serviceA"),
                ),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("serviceB"),
                    CallInstrValue::Variable("function"),
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
                CallInstrArgValue::Variable("name"),
            ]),
            output: Accumulator("output"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call id.$.a! "f" ["hello" name] void[])
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::JsonPath {
            variable: "id",
            path: "$.a",
            should_flatten: true,
        }),
        function_part: FuncName(CallInstrValue::Literal("f")),
        args: Rc::new(vec![
            CallInstrArgValue::Literal("hello"),
            CallInstrArgValue::Variable("name"),
        ]),
        output: Accumulator("void"),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn parse_undefined_variable() {
    let source_code = r#"
        (call id.$.a "f" ["hello" name] void[])
        "#;

    let lexer = crate::AIRLexer::new(source_code);

    let parser = crate::AIRParser::new();
    let mut errors = Vec::new();
    let mut validator = super::VariableValidator::new();
    parser
        .parse(source_code, &mut errors, &mut validator, lexer)
        .expect("parser shoudn't fail");

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
                    (call "" ("" "") ["hello" ""] void[])
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
        .expect("parser shoudn't fail");

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
fn parse_json_path_complex() {
    use ast::Call;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (call m.$.[1]! "f" [] void)
            (call m.$.abc["c"].cde[a][0].cde["bcd"]! "f" [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::JsonPath {
                variable: "m",
                path: "$.[1]",
                should_flatten: true,
            }),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![]),
            output: Scalar("void"),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::JsonPath {
                variable: "m",
                path: r#"$.abc["c"].cde[a][0].cde["bcd"]"#,
                should_flatten: true,
            }),
            function_part: FuncName(CallInstrValue::Literal("f")),
            args: Rc::new(vec![]),
            output: Scalar("void"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn json_path_square_braces() {
    use ast::Call;
    use ast::CallInstrArgValue;
    use ast::CallInstrValue;
    use ast::CallOutputValue::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call u.$["peer_id"]! ("return" "") [u.$["peer_id"].cde[0]["abc"].abc u.$["name"]] void[])
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::JsonPath {
            variable: "u",
            path: r#"$["peer_id"]"#,
            should_flatten: true,
        }),
        function_part: ServiceIdWithFuncName(
            CallInstrValue::Literal("return"),
            CallInstrValue::Literal(""),
        ),
        args: Rc::new(vec![
            CallInstrArgValue::JsonPath {
                variable: "u",
                path: r#"$["peer_id"].cde[0]["abc"].abc"#,
                should_flatten: false,
            },
            CallInstrArgValue::JsonPath {
                variable: "u",
                path: r#"$["name"]"#,
                should_flatten: false,
            },
        ]),
        output: Accumulator("void"),
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
    let instruction = parse(source_code);
    let expected = fold(ast::IterableValue::Variable("iterable"), "i", null());
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
    let instruction = parse(source_code);
    let expected = match_(Variable("v1"), Variable("v2"), null());
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
    let instruction = parse(source_code);
    let expected = mismatch(Variable("v1"), Variable("v2"), null());
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
        let expected = fold(
            ast::IterableValue::Variable("iterable"),
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
            args: Rc::new(vec![CallInstrArgValue::LastError]),
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
                output: Scalar("result_1"),
            }),
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal("service_id"),
                    CallInstrValue::Literal("fn_name"),
                ),
                args: Rc::new(vec![]),
                output: Scalar("g"),
            }),
        ),
        Instruction::Call(Call {
            peer_part: PeerPk(CallInstrValue::Literal(&peer_id)),
            function_part: ServiceIdWithFuncName(
                CallInstrValue::Literal("local_service_id"),
                CallInstrValue::Literal("local_fn_name"),
            ),
            args: Rc::new(vec![]),
            output: Scalar("result_2"),
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
                    output: Scalar("module-bytes"),
                }),
                Instruction::Call(Call {
                    peer_part: PeerPk(CallInstrValue::Literal("set_variables")),
                    function_part: ServiceIdWithFuncName(
                        CallInstrValue::Literal(""),
                        CallInstrValue::Literal(""),
                    ),
                    args: Rc::new(vec![CallInstrArgValue::Literal("module_config")]),
                    output: Scalar("module_config"),
                }),
            ),
            Instruction::Call(Call {
                peer_part: PeerPk(CallInstrValue::Literal("set_variables")),
                function_part: ServiceIdWithFuncName(
                    CallInstrValue::Literal(""),
                    CallInstrValue::Literal(""),
                ),
                args: Rc::new(vec![CallInstrArgValue::Literal("blueprint")]),
                output: Scalar("blueprint"),
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
                    CallInstrArgValue::Variable("module-bytes"),
                    CallInstrArgValue::Variable("module_config"),
                ]),
                output: Scalar("module"),
            }),
            seq(
                Instruction::Call(Call {
                    peer_part: PeerPk(CallInstrValue::Literal("A")),
                    function_part: ServiceIdWithFuncName(
                        CallInstrValue::Literal("add_blueprint"),
                        CallInstrValue::Literal(""),
                    ),
                    args: Rc::new(vec![CallInstrArgValue::Variable("blueprint")]),
                    output: Scalar("blueprint_id"),
                }),
                seq(
                    Instruction::Call(Call {
                        peer_part: PeerPk(CallInstrValue::Literal("A")),
                        function_part: ServiceIdWithFuncName(
                            CallInstrValue::Literal("create"),
                            CallInstrValue::Literal(""),
                        ),
                        args: Rc::new(vec![CallInstrArgValue::Variable("blueprint_id")]),
                        output: Scalar("service_id"),
                    }),
                    Instruction::Call(Call {
                        peer_part: PeerPk(CallInstrValue::Literal("remote_peer_id")),
                        function_part: ServiceIdWithFuncName(
                            CallInstrValue::Literal(""),
                            CallInstrValue::Literal(""),
                        ),
                        args: Rc::new(vec![CallInstrArgValue::Variable("service_id")]),
                        output: Scalar("client_result"),
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

    let left_value = Variable("isOnline");
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
    let right_value = Variable("isOnline");
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
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(CallInstrValue::Variable("peer")),
        function_part: ServiceIdWithFuncName(
            CallInstrValue::Variable("service"),
            CallInstrValue::Variable("fname"),
        ),
        args: Rc::new(vec![]),
        output: None,
    });
    assert_eq!(instruction, expected);
}

#[test]
fn fold_json_path() {
    use ast::Fold;
    use ast::IterableValue::*;

    let source_code = r#"
    ; comment
    (fold members.$.["users"] m (null)) ;;; comment
    ;;; comment
    "#;
    let instruction = parse(source_code);
    let expected = Instruction::Fold(Fold {
        iterable: JsonPath {
            variable: "members",
            path: "$.[\"users\"]",
            should_flatten: false,
        },
        iterator: "m",
        instruction: Rc::new(null()),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn comments() {
    use ast::Fold;
    use ast::IterableValue::*;

    let source_code = r#"
    ; comment
    (fold members.$.["users"] m (null)) ;;; comment ;;?()()
    ;;; comme;?!.$.  nt[][][][()()()null;$::!
    "#;
    let instruction = parse(source_code);
    let expected = Instruction::Fold(Fold {
        iterable: JsonPath {
            variable: "members",
            path: "$.[\"users\"]",
            should_flatten: false,
        },
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

fn fold<'a>(
    iterable: ast::IterableValue<'a>,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::Fold(ast::Fold {
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
