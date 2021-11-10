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
use ast::*;

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
        let res = parser
            .parse(source_code, &mut errors, &mut validator, lexer)
            .expect("parsing should be successful");

        println!("{:?}", errors);
        res
    })
}

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
            CallInstrValue::Variable(VariableWithLambda::scalar("peer_id")),
            CallInstrValue::Variable(VariableWithLambda::scalar("service_id")),
            CallInstrValue::Variable(VariableWithLambda::scalar("function_name")),
            Rc::new(vec![AIRValue::EmptyArray, AIRValue::EmptyArray]),
            CallOutputValue::Variable(Variable::scalar("output")),
        ),
        call(
            CallInstrValue::Literal("peer_id"),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![
                AIRValue::Literal("hello"),
                AIRValue::EmptyArray,
                AIRValue::Variable(VariableWithLambda::scalar("name")),
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
                (call (peer_id service_A) ("service_B" function_name) [])
            )
            (call "peer_id" ("service_id" "function_name") ["hello" name] $output)
        )
        "#;
    let instruction = parse(source_code);
    let expected = seq(
        seq(
            call(
                CallInstrValue::Variable(VariableWithLambda::scalar("peer_id")),
                CallInstrValue::Variable(VariableWithLambda::scalar("service_id")),
                CallInstrValue::Variable(VariableWithLambda::scalar("function_name")),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
            call(
                CallInstrValue::Variable(VariableWithLambda::scalar("peer_id")),
                CallInstrValue::Literal("service_B"),
                CallInstrValue::Variable(VariableWithLambda::scalar("function_name")),
                Rc::new(vec![]),
                CallOutputValue::None,
            ),
        ),
        call(
            CallInstrValue::Literal("peer_id"),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![
                AIRValue::Literal("hello"),
                AIRValue::Variable(VariableWithLambda::scalar("name")),
            ]),
            CallOutputValue::Variable(Variable::stream("$output")),
        ),
    );
    assert_eq!(instruction, expected);
}

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
        )),
        CallInstrValue::Literal("service_id"),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            AIRValue::Literal("hello"),
            AIRValue::Variable(VariableWithLambda::scalar("name")),
        ]),
        CallOutputValue::Variable(Variable::stream("$void")),
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
        CallInstrValue::Variable(VariableWithLambda::scalar("peer_id")),
        CallInstrValue::Variable(VariableWithLambda::scalar("service_id")),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            AIRValue::Literal(""),
            AIRValue::EmptyArray,
            AIRValue::Variable(VariableWithLambda::scalar("arg")),
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
        CallInstrValue::Variable(VariableWithLambda::scalar("peer_id")),
        CallInstrValue::Literal("service_id"),
        CallInstrValue::Literal("function_name"),
        Rc::new(vec![
            AIRValue::Variable(VariableWithLambda::scalar("k")),
            AIRValue::EmptyArray,
            AIRValue::EmptyArray,
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
        (call "" ("" "") [$stream])
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
fn parse_undefined_stream_with_lambda() {
    let source_code = r#"
        (call "" ("" "") [$stream.$.json_path])
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
            )),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("void")),
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
            )),
            CallInstrValue::Literal("service_id"),
            CallInstrValue::Literal("function_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("void")),
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
        )),
        CallInstrValue::Literal("return"),
        CallInstrValue::Literal(""),
        Rc::new(vec![
            AIRValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "u",
                vec![
                    ValueAccessor::ArrayAccess { idx: 1 },
                    ValueAccessor::FieldAccess { field_name: "cde" },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::ArrayAccess { idx: 0 },
                    ValueAccessor::FieldAccess { field_name: "abc" },
                ],
            )),
            AIRValue::Variable(VariableWithLambda::from_raw_lambda_scalar(
                "u",
                vec![ValueAccessor::FieldAccess { field_name: "name" }],
            )),
        ]),
        CallOutputValue::Variable(Variable::stream("$void")),
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
    let expected = fold_scalar(ScalarWithLambda::new("iterable", None), "i", null());
    assert_eq!(instruction, expected);
}

#[test]
fn parse_match() {
    let source_code = r#"
        (match v1 v2
            (null)
        )
        "#;
    let instruction = parse(&source_code);
    let expected = match_(
        AIRValue::Variable(VariableWithLambda::scalar("v1")),
        AIRValue::Variable(VariableWithLambda::scalar("v2")),
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
        AIRValue::Variable(VariableWithLambda::scalar("v1")),
        AIRValue::InitPeerId,
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
        AIRValue::Variable(VariableWithLambda::scalar("v1")),
        AIRValue::Variable(VariableWithLambda::scalar("v2")),
        null(),
    );
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
            ScalarWithLambda::new("iterable", None),
            "i",
            instr(null(), null()),
        );
        assert_eq!(instruction, expected);
    }
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
            Rc::new(vec![AIRValue::LastError(LastErrorPath::None)]),
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
                CallOutputValue::Variable(Variable::scalar("result_1")),
            ),
            call(
                CallInstrValue::Literal(peer_id),
                CallInstrValue::Literal("service_id"),
                CallInstrValue::Literal("fn_name"),
                Rc::new(vec![]),
                CallOutputValue::Variable(Variable::scalar("g")),
            ),
        ),
        call(
            CallInstrValue::Literal(peer_id),
            CallInstrValue::Literal("local_service_id"),
            CallInstrValue::Literal("local_fn_name"),
            Rc::new(vec![]),
            CallOutputValue::Variable(Variable::scalar("result_2")),
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
                    Rc::new(vec![AIRValue::Literal("module-bytes")]),
                    CallOutputValue::Variable(Variable::scalar("module-bytes")),
                ),
                call(
                    CallInstrValue::Literal("set_variables"),
                    CallInstrValue::Literal(""),
                    CallInstrValue::Literal(""),
                    Rc::new(vec![AIRValue::Literal("module_config")]),
                    CallOutputValue::Variable(Variable::scalar("module_config")),
                ),
            ),
            call(
                CallInstrValue::Literal("set_variables"),
                CallInstrValue::Literal(""),
                CallInstrValue::Literal(""),
                Rc::new(vec![AIRValue::Literal("blueprint")]),
                CallOutputValue::Variable(Variable::scalar("blueprint")),
            ),
        ),
        seq(
            call(
                CallInstrValue::Literal("A"),
                CallInstrValue::Literal("add_module"),
                CallInstrValue::Literal(""),
                Rc::new(vec![
                    AIRValue::Variable(VariableWithLambda::scalar("module-bytes")),
                    AIRValue::Variable(VariableWithLambda::scalar("module_config")),
                ]),
                CallOutputValue::Variable(Variable::scalar("module")),
            ),
            seq(
                Instruction::Call(Call {
                    triplet: Triplet {
                        peer_pk: CallInstrValue::Literal("A"),
                        service_id: CallInstrValue::Literal("add_blueprint"),
                        function_name: CallInstrValue::Literal(""),
                    },
                    args: Rc::new(vec![AIRValue::Variable(VariableWithLambda::scalar(
                        "blueprint",
                    ))]),
                    output: CallOutputValue::Variable(Variable::scalar("blueprint_id")),
                }),
                seq(
                    call(
                        CallInstrValue::Literal("A"),
                        CallInstrValue::Literal("create"),
                        CallInstrValue::Literal(""),
                        Rc::new(vec![AIRValue::Variable(VariableWithLambda::scalar(
                            "blueprint_id",
                        ))]),
                        CallOutputValue::Variable(Variable::scalar("service_id")),
                    ),
                    call(
                        CallInstrValue::Literal("remote_peer_id"),
                        CallInstrValue::Literal(""),
                        CallInstrValue::Literal(""),
                        Rc::new(vec![AIRValue::Variable(VariableWithLambda::scalar(
                            "service_id",
                        ))]),
                        CallOutputValue::Variable(Variable::scalar("client_result")),
                    ),
                ),
            ),
        ),
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

    let left_value = AIRValue::Variable(VariableWithLambda::scalar("isOnline"));
    let right_value = AIRValue::Boolean(true);
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

    let left_value = AIRValue::Boolean(true);
    let right_value = AIRValue::Variable(VariableWithLambda::scalar("isOnline"));
    let null = null();
    let expected = mismatch(left_value, right_value, null);

    let instruction = parse(source_code);
    assert_eq!(expected, instruction);
}

#[test]
fn no_output() {
    let source_code = r#"
        (call peer (service fname) [])
    "#;

    let actual = parse(source_code);

    let expected = call(
        CallInstrValue::Variable(VariableWithLambda::scalar("peer")),
        CallInstrValue::Variable(VariableWithLambda::scalar("service")),
        CallInstrValue::Variable(VariableWithLambda::scalar("fname")),
        Rc::new(vec![]),
        CallOutputValue::None,
    );
    assert_eq!(actual, expected);
}

#[test]
fn ap_with_literal() {
    let source_code = r#"
        (ap "some_string" $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Literal("some_string"),
        VariableWithLambda::stream("$stream"),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_number() {
    use ast::Number;

    let source_code = r#"
        (ap -100 $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::Number(Number::Int(-100)),
        VariableWithLambda::stream("$stream"),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_bool() {
    let source_code = r#"
        (ap true $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ast::ApArgument::Boolean(true),
        VariableWithLambda::stream("$stream"),
    );

    assert_eq!(actual, expected);
}

#[test]
fn ap_with_last_error() {
    let source_code = r#"
        (ap %last_error%.$.msg! $stream)
    "#;

    let actual = parse(source_code);
    let expected = ap(
        ApArgument::LastError(LastErrorPath::Message),
        VariableWithLambda::stream("$stream"),
    );

    assert_eq!(actual, expected);
}

#[test]
fn fold_json_path() {
    let source_code = r#"
        ; comment
        (fold members.$.[123321] m (null)) ;;; comment
        ;;; comment
    "#;

    let instruction = parse(source_code);
    let expected = fold_scalar(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![ValueAccessor::ArrayAccess { idx: 123321 }],
        ),
        "m",
        null(),
    );
    assert_eq!(instruction, expected);
}

#[test]
fn fold_on_stream() {
    let source_code = r#"
        (fold $stream iterator (null))
    "#;

    let instruction = parse(source_code);
    let expected = fold_stream("$stream", "iterator", null());
    assert_eq!(instruction, expected);
}

#[test]
fn comments() {
    let source_code = r#"
        ; comment
        (fold members.$.field[1] m (null)) ;;; comment ;;?()()
        ;;; comme;?!.$.  nt[][][][()()()null;$::!
    "#;
    let instruction = parse(source_code);
    let expected = fold_scalar(
        ScalarWithLambda::from_raw_lambda(
            "members",
            vec![
                ValueAccessor::FieldAccess {
                    field_name: "field",
                },
                ValueAccessor::ArrayAccess { idx: 1 },
            ],
        ),
        "m",
        null(),
    );
    assert_eq!(instruction, expected);
}

// Test DSL

fn call<'i>(
    peer_pk: CallInstrValue<'i>,
    service_id: CallInstrValue<'i>,
    function_name: CallInstrValue<'i>,
    args: Rc<Vec<AIRValue<'i>>>,
    output: CallOutputValue<'i>,
) -> Instruction<'i> {
    let triplet = Triplet {
        peer_pk,
        service_id,
        function_name,
    };

    Instruction::Call(Call {
        triplet,
        args,
        output,
    })
}

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
    iterable: ScalarWithLambda<'a>,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::FoldScalar(FoldScalar {
        iterable,
        iterator: Scalar::new(iterator),
        instruction: std::rc::Rc::new(instruction),
    })
}

fn fold_stream<'a>(
    stream_name: &'a str,
    iterator: &'a str,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::FoldStream(FoldStream {
        iterable: Stream::new(stream_name),
        iterator: Scalar::new(iterator),
        instruction: std::rc::Rc::new(instruction),
    })
}

fn match_<'a>(
    left_value: AIRValue<'a>,
    right_value: AIRValue<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::Match(ast::Match {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

fn mismatch<'a>(
    left_value: AIRValue<'a>,
    right_value: AIRValue<'a>,
    instruction: Instruction<'a>,
) -> Instruction<'a> {
    Instruction::MisMatch(ast::MisMatch {
        left_value,
        right_value,
        instruction: Box::new(instruction),
    })
}

fn ap<'i>(argument: ApArgument<'i>, result: VariableWithLambda<'i>) -> Instruction<'i> {
    Instruction::Ap(Ap { argument, result })
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
