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

use crate::ast::Instruction;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, RuleType};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "peg/aqua.pest"]
pub struct Expression {}

#[derive(Debug, Default)]
pub struct Script {}

// impl Script {
//     pub fn add(&mut self, _inst: Instruction) {}
// }

type PestPairs<'a> = pest::iterators::Pairs<'a, Rule>;
type Result<T> = std::result::Result<T, Error<Rule>>;

fn parse(source_code: &str) -> Box<Instruction> {
    parse_core(source_code);
    Box::new(Instruction::Null)
}

fn next(mut pairs: PestPairs<'_>) {
    if let Some(pair) = pairs.next() {
        let rule = pair.as_rule();
        let inner = pair.into_inner();
        println!("rule: {:#?}\ninner:{:#?}\n", rule, inner);
        inner
    }
}

pub fn parse_core(air_script: &str) -> Script {
    let expr: PestPairs<'_> = Expression::parse(Rule::expr, "( seq )").expect("parse expr");
    next(expr);

    let script = Script::default();

    /*for pair in expr {
        println!("pair: {:#?}", pair);
        // Loop over the pairs converted as an iterator of the tokens
        // which composed it.
        /*for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::instruction => {
                    let inst = inner_pair.into_inner().next().expect("instruction");
                    match inst.as_rule() {
                        Rule::seq => println!("rule: seq; inst: {:#?}", inst),
                        r => println!("rule: {:?}; inst: {:#?}", r, inst),
                    }
                }
                r => println!("rule: {:?}", r),
            }
            // let inner_span = inner_pair.clone().as_span().as_str();
            // println!("inner_span {:#?}", inner_span);

            // Populate the group based on the rules found by the
            // parser.
            // println!("inner_pair {:?}", inner_pair);
        }*/
    }*/

    Script {}
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ast::CallOutput::*;
    use crate::ast::FunctionPart::*;
    use crate::ast::PeerPart::*;
    use crate::ast::Value::*;
    use crate::ast::*;

    #[test]
    fn do_parse() {
        let script = parse("(seq)");
        println!("{:?}", script);
    }

    #[test]
    fn parse_seq() {
        let source_code = r#"
        (seq
            (call peerid function () void)
            (call "id" "f" ("hello" name) void[])
        )
        "#;
        let instruction = *parse(source_code);
        let expected = Instruction::Seq(Seq(
            Box::new(Instruction::Call(Call {
                peer: PeerPk(Variable("peerid")),
                f: FuncName(Variable("function")),
                args: vec![],
                output: Scalar("void"),
            })),
            Box::new(Instruction::Call(Call {
                peer: PeerPk(Literal("id")),
                f: FuncName(Literal("f")),
                args: vec![Literal("hello"), Variable("name")],
                output: Accumulator("void"),
            })),
        ));
        assert_eq!(instruction, expected);
    }

    #[test]
    fn parse_seq_seq() {
        let source_code = r#"
        (seq
            (seq
                (call peerid function () void)
                (call peerid function () void)
            )
            (call "id" "f" ("hello" name) void[])
        )
        "#;
        let instruction = *parse(source_code);
        let expected = Instruction::Seq(Seq(
            Box::new(Instruction::Seq(Seq(
                Box::new(Instruction::Call(Call {
                    peer: PeerPk(Variable("peerid")),
                    f: FuncName(Variable("function")),
                    args: vec![],
                    output: Scalar("void"),
                })),
                Box::new(Instruction::Call(Call {
                    peer: PeerPk(Variable("peerid")),
                    f: FuncName(Variable("function")),
                    args: vec![],
                    output: Scalar("void"),
                })),
            ))),
            Box::new(Instruction::Call(Call {
                peer: PeerPk(Literal("id")),
                f: FuncName(Literal("f")),
                args: vec![Literal("hello"), Variable("name")],
                output: Accumulator("void"),
            })),
        ));
        assert_eq!(instruction, expected);
    }
}
