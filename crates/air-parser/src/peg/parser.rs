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

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;
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

pub fn parse(air_script: &str) -> Script {
    let expr: PestPairs<'_> = Expression::parse(Rule::expr, "( seq )").expect("parse expr");

    let script = Script::default();

    for pair in expr {
        println!("pair: {:#?}", pair);
        // Loop over the pairs converted as an iterator of the tokens
        // which composed it.
        for inner_pair in pair.into_inner() {
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
        }
    }

    Script {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_parse() {
        let script = parse("(seq)");
        println!("{:?}", script);
    }
}
