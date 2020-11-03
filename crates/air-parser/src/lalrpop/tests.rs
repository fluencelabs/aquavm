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

use crate::ast::*;
use CallOutput::*;
use FunctionPart::*;
use PeerPart::*;
use Value::*;

use fstrings::f;

fn parse(source_code: &str) -> Instruction {
    *super::parse(source_code).expect("parsing failed")
}

#[test]
fn parse_seq() {
    let source_code = r#"
        (seq
            (call peerid function [] void)
            (call "id" "f" ["hello" name] void[])
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer: PeerPk(Variable("peerid")),
            f: FuncName(Variable("function")),
            args: vec![],
            output: Scalar("void"),
        }),
        Instruction::Call(Call {
            peer: PeerPk(Literal("id")),
            f: FuncName(Literal("f")),
            args: vec![Literal("hello"), Variable("name")],
            output: Accumulator("void"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_seq_seq() {
    // TODO: make output one of _ () "" and absence

    let source_code = r#"
        (seq
            (seq
                (call peerid function [] void)
                (call (peerid serviceA) ("serviceB" function) [] void)
            )
            (call "id" "f" ["hello" name] void[])
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        seq(
            Instruction::Call(Call {
                peer: PeerPk(Variable("peerid")),
                f: FuncName(Variable("function")),
                args: vec![],
                output: Scalar("void"),
            }),
            Instruction::Call(Call {
                peer: PeerPkWithServiceId(Variable("peerid"), Variable("serviceA")),
                f: ServiceIdWithFuncName(Literal("serviceB"), Variable("function")),
                args: vec![],
                output: Scalar("void"),
            }),
        ),
        Instruction::Call(Call {
            peer: PeerPk(Literal("id")),
            f: FuncName(Literal("f")),
            args: vec![Literal("hello"), Variable("name")],
            output: Accumulator("void"),
        }),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path() {
    let source_code = r#"
        (call id.$.a "f" ["hello" name] void[])
        "#;
    let instruction = parse(source_code);
    let expected = Instruction::Call(Call {
        peer: PeerPk(JsonPath {
            variable: "id",
            path: "$.a",
        }),
        f: FuncName(Literal("f")),
        args: vec![Literal("hello"), Variable("name")],
        output: Accumulator("void"),
    });
    assert_eq!(instruction, expected);
}

#[test]
fn parse_json_path_complex() {
    let source_code = r#"
        (seq
            (call m.$.[1] "f" [] void)
            (call m.$.abc.cde[a][0].cde "f" [] void)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        Instruction::Call(Call {
            peer: PeerPk(JsonPath {
                variable: "m",
                path: "$.[1]",
            }),
            f: FuncName(Literal("f")),
            args: vec![],
            output: Scalar("void"),
        }),
        Instruction::Call(Call {
            peer: PeerPk(JsonPath {
                variable: "m",
                path: "$.abc.cde[a][0].cde",
            }),
            f: FuncName(Literal("f")),
            args: vec![],
            output: Scalar("void"),
        }),
    );
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
    let expected = Instruction::Seq(Seq(Box::new(null()), Box::new(null())));
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
    let expected = fold("iterable", "i", null());
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
        let expected = fold("iterable", "i", instr(null(), null()));
        assert_eq!(instruction, expected);
    }
}

#[test]
fn seq_par_call() {
    let source_code = r#"
        (seq 
            (par 
                (call %current_peer_id% ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call %current_peer_id% ("local_service_id" "local_fn_name") [] result_2)
        )"#;
    let instruction = parse(&source_code.as_ref());
    let expected = seq(
        par(
            Instruction::Call(Call {
                peer: PeerPk(CurrentPeerId),
                f: ServiceIdWithFuncName(Literal("local_service_id"), Literal("local_fn_name")),
                args: vec![],
                output: Scalar("result_1"),
            }),
            Instruction::Call(Call {
                peer: PeerPk(Literal("remote_peer_id")),
                f: ServiceIdWithFuncName(Literal("service_id"), Literal("fn_name")),
                args: vec![],
                output: Scalar("g"),
            }),
        ),
        Instruction::Call(Call {
            peer: PeerPk(CurrentPeerId),
            f: ServiceIdWithFuncName(Literal("local_service_id"), Literal("local_fn_name")),
            args: vec![],
            output: Scalar("result_2"),
        }),
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
    let instruction = parse(&source_code.as_ref());
    let expected = seq(
        seq(
            seq(
                Instruction::Call(Call {
                    peer: PeerPk(Literal("set_variables")),
                    f: ServiceIdWithFuncName(Literal(""), Literal("")),
                    args: vec![Literal("module-bytes")],
                    output: Scalar("module-bytes"),
                }),
                Instruction::Call(Call {
                    peer: PeerPk(Literal("set_variables")),
                    f: ServiceIdWithFuncName(Literal(""), Literal("")),
                    args: vec![Literal("module_config")],
                    output: Scalar("module_config"),
                }),
            ),
            Instruction::Call(Call {
                peer: PeerPk(Literal("set_variables")),
                f: ServiceIdWithFuncName(Literal(""), Literal("")),
                args: vec![Literal("blueprint")],
                output: Scalar("blueprint"),
            }),
        ),
        seq(
            Instruction::Call(Call {
                peer: PeerPk(Literal("A")),
                f: ServiceIdWithFuncName(Literal("add_module"), Literal("")),
                args: vec![Variable("module-bytes"), Variable("module_config")],
                output: Scalar("module"),
            }),
            seq(
                Instruction::Call(Call {
                    peer: PeerPk(Literal("A")),
                    f: ServiceIdWithFuncName(Literal("add_blueprint"), Literal("")),
                    args: vec![Variable("blueprint")],
                    output: Scalar("blueprint_id"),
                }),
                seq(
                    Instruction::Call(Call {
                        peer: PeerPk(Literal("A")),
                        f: ServiceIdWithFuncName(Literal("create"), Literal("")),
                        args: vec![Variable("blueprint_id")],
                        output: Scalar("service_id"),
                    }),
                    Instruction::Call(Call {
                        peer: PeerPk(Literal("remote_peer_id")),
                        f: ServiceIdWithFuncName(Literal(""), Literal("")),
                        args: vec![Variable("service_id")],
                        output: Scalar("client_result"),
                    }),
                ),
            ),
        ),
    );

    assert_eq!(instruction, expected);
}

// Test DSL

fn seq<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Seq(Seq(Box::new(l), Box::new(r)))
}
fn par<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Par(Par(Box::new(l), Box::new(r)))
}
fn xor<'a>(l: Instruction<'a>, r: Instruction<'a>) -> Instruction<'a> {
    Instruction::Xor(Xor(Box::new(l), Box::new(r)))
}
fn seqnn() -> Instruction<'static> {
    seq(null(), null())
}
fn null() -> Instruction<'static> {
    Instruction::Null(Null)
}
fn fold<'a>(iterable: &'a str, iterator: &'a str, instruction: Instruction<'a>) -> Instruction<'a> {
    Instruction::Fold(Fold {
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
