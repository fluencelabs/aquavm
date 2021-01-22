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
use ast::Instruction;

use fstrings::f;
use std::rc::Rc;

fn parse(source_code: &str) -> Instruction {
    *crate::parse(source_code).expect("parsing failed")
}

#[test]
fn parse_seq() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
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
            peer_part: PeerPk(Variable("peerid")),
            function_part: FuncName(Variable("function")),
            args: Rc::new(vec![]),
            output: Scalar("output"),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(Literal("id")),
            function_part: FuncName(Literal("f")),
            args: Rc::new(vec![Literal("hello"), Variable("name")]),
            output: None,
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_seq_seq() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
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
                peer_part: PeerPk(Variable("peerid")),
                function_part: FuncName(Variable("function")),
                args: Rc::new(vec![]),
                output: None,
            }),
            Instruction::Call(Call {
                peer_part: PeerPkWithServiceId(Variable("peerid"), Variable("serviceA")),
                function_part: ServiceIdWithFuncName(Literal("serviceB"), Variable("function")),
                args: Rc::new(vec![]),
                output: None,
            }),
        ),
        Instruction::Call(Call {
            peer_part: PeerPk(Literal("id")),
            function_part: FuncName(Literal("f")),
            args: Rc::new(vec![Literal("hello"), Variable("name")]),
            output: Accumulator("output"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call id.$.a "f" ["hello" name] void[])
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(JsonPath {
            variable: "id",
            path: "$.a",
        }),
        function_part: FuncName(Literal("f")),
        args: Rc::new(vec![Literal("hello"), Variable("name")]),
        output: Accumulator("void"),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path_complex() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (seq
            (call m.$.[1] "f" [] void)
            (call m.$.abc["c"].cde[a][0].cde["bcd"] "f" [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(JsonPath {
                variable: "m",
                path: "$.[1]",
            }),
            function_part: FuncName(Literal("f")),
            args: Rc::new(vec![]),
            output: Scalar("void"),
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(JsonPath {
                variable: "m",
                path: r#"$.abc["c"].cde[a][0].cde["bcd"]"#,
            }),
            function_part: FuncName(Literal("f")),
            args: Rc::new(vec![]),
            output: Scalar("void"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn json_path_square_braces() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
        (call u.$["peer_id"] ("return" "") [u.$["peer_id"].cde[0]["abc"].abc u.$["name"]] void[])
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(JsonPath {
            variable: "u",
            path: r#"$["peer_id"]"#,
        }),
        function_part: ServiceIdWithFuncName(Literal("return"), Literal("")),
        args: Rc::new(vec![
            JsonPath {
                variable: "u",
                path: r#"$["peer_id"].cde[0]["abc"].abc"#,
            },
            JsonPath {
                variable: "u",
                path: r#"$["name"]"#,
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
        let instruction = parse(&source_code.as_ref());
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
    let instruction = parse(&source_code.as_ref());
    let expected = fold(ast::IterableValue::Variable("iterable"), "i", null());
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
        let instruction = parse(&source_code.as_ref());
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
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
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

    let instruction = parse(&source_code.as_ref());
    let expected = seq(
        Instruction::Call(Call {
            peer_part: PeerPk(Literal(&peer_id)),
            function_part: ServiceIdWithFuncName(
                Literal("local_service_id"),
                Literal("local_fn_name"),
            ),
            args: Rc::new(vec![]),
            output: None,
        }),
        Instruction::Call(Call {
            peer_part: PeerPk(InitPeerId),
            function_part: ServiceIdWithFuncName(Literal("service_id"), Literal("fn_name")),
            args: Rc::new(vec![]),
            output: None,
        }),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn seq_par_call() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
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

    let instruction = parse(&source_code.as_ref());
    let expected = seq(
        par(
            Instruction::Call(Call {
                peer_part: PeerPk(Literal(&peer_id)),
                function_part: ServiceIdWithFuncName(
                    Literal("local_service_id"),
                    Literal("local_fn_name"),
                ),
                args: Rc::new(vec![]),
                output: Scalar("result_1"),
            }),
            Instruction::Call(Call {
                peer_part: PeerPk(Literal(&peer_id)),
                function_part: ServiceIdWithFuncName(Literal("service_id"), Literal("fn_name")),
                args: Rc::new(vec![]),
                output: Scalar("g"),
            }),
        ),
        Instruction::Call(Call {
            peer_part: PeerPk(Literal(&peer_id)),
            function_part: ServiceIdWithFuncName(
                Literal("local_service_id"),
                Literal("local_fn_name"),
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
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
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
    let instruction = parse(&source_code.as_ref());
    let expected = seq(
        seq(
            seq(
                Instruction::Call(Call {
                    peer_part: PeerPk(Literal("set_variables")),
                    function_part: ServiceIdWithFuncName(Literal(""), Literal("")),
                    args: Rc::new(vec![Literal("module-bytes")]),
                    output: Scalar("module-bytes"),
                }),
                Instruction::Call(Call {
                    peer_part: PeerPk(Literal("set_variables")),
                    function_part: ServiceIdWithFuncName(Literal(""), Literal("")),
                    args: Rc::new(vec![Literal("module_config")]),
                    output: Scalar("module_config"),
                }),
            ),
            Instruction::Call(Call {
                peer_part: PeerPk(Literal("set_variables")),
                function_part: ServiceIdWithFuncName(Literal(""), Literal("")),
                args: Rc::new(vec![Literal("blueprint")]),
                output: Scalar("blueprint"),
            }),
        ),
        seq(
            Instruction::Call(Call {
                peer_part: PeerPk(Literal("A")),
                function_part: ServiceIdWithFuncName(Literal("add_module"), Literal("")),
                args: Rc::new(vec![Variable("module-bytes"), Variable("module_config")]),
                output: Scalar("module"),
            }),
            seq(
                Instruction::Call(Call {
                    peer_part: PeerPk(Literal("A")),
                    function_part: ServiceIdWithFuncName(Literal("add_blueprint"), Literal("")),
                    args: Rc::new(vec![Variable("blueprint")]),
                    output: Scalar("blueprint_id"),
                }),
                seq(
                    Instruction::Call(Call {
                        peer_part: PeerPk(Literal("A")),
                        function_part: ServiceIdWithFuncName(Literal("create"), Literal("")),
                        args: Rc::new(vec![Variable("blueprint_id")]),
                        output: Scalar("service_id"),
                    }),
                    Instruction::Call(Call {
                        peer_part: PeerPk(Literal("remote_peer_id")),
                        function_part: ServiceIdWithFuncName(Literal(""), Literal("")),
                        args: Rc::new(vec![Variable("service_id")]),
                        output: Scalar("client_result"),
                    }),
                ),
            ),
        ),
    );

    assert_eq!(instruction, expected);
}

#[test]
fn no_output() {
    use ast::Call;
    use ast::InstructionArg::*;
    use ast::CallOutput::*;
    use ast::FunctionPart::*;
    use ast::PeerPart::*;

    let source_code = r#"
    (call peer (service fname) [])
    "#;
    let instruction = parse(&source_code.as_ref());
    let expected = Instruction::Call(Call {
        peer_part: PeerPk(Variable("peer")),
        function_part: ServiceIdWithFuncName(Variable("service"), Variable("fname")),
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
    let instruction = parse(&source_code.as_ref());
    let expected = Instruction::Fold(Fold {
        iterable: JsonPath {
            variable: "members",
            path: "$.[\"users\"]",
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
    let instruction = parse(&source_code.as_ref());
    let expected = Instruction::Fold(Fold {
        iterable: JsonPath {
            variable: "members",
            path: "$.[\"users\"]",
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
