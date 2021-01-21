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

#[macro_use]
extern crate fstrings;

use std::rc::Rc;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use air_parser::AIRParser;
use air_parser::AIRLexer;

const SOURCE_CODE_BAD: &'static str = r#"(seq
        (seq
            (call node ("identity" "") [] void[])
            (call provider (service_id "{fname}") {arg_list} result)
        )
        (seq
            (call node ("identity" "") [] void[])
            (call "{LOCAL_VM}" ("return" "result") [result] void[])
        )
    )"#;

const SOURCE_CODE_GOOD: &'static str = r#"
    (seq
        (seq
            (call node ("identity" "") [] void[])
            (call provider (service_id "fname") [arg list] result)
        )
        (seq
            (call node ("identity" "") [] void[])
            (call "local_vm" ("return" "result") [result] void[])
        )
    )"#;

#[cfg(test)]
mod gen {
    use crate::SOURCE_CODE_GOOD;

    pub fn seq(left: &str, right: &str) -> String {
        f!(r"(seq {left} {right})")
    }

    pub fn deep_seq(mut depth: usize) -> String {
        let mut instr = SOURCE_CODE_GOOD.to_string();
        loop {
            depth -= 1;
            if depth == 0 {
                break;
            }
            instr = seq(&instr, &instr)
        }

        instr
    }
}

fn create_parser(c: &mut Criterion) {
    c.bench_function("create_parser", move |b| b.iter(move || AIRParser::new()));
}

fn clone_parser(c: &mut Criterion) {
    let parser = AIRParser::new();
    c.bench_function("clone_parser", move |b| {
        let parser = parser.clone();
        b.iter(move || parser.clone())
    });
}

fn clone_parser_rc(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    c.bench_function("clone_parser_rc", move |b| {
        let parser = parser.clone();
        b.iter(move || parser.clone())
    });
}

fn parse(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    c.bench_function(
        format!("parse {} bytes", SOURCE_CODE_GOOD.len()).as_str(),
        move |b| {
            let parser = parser.clone();
            b.iter(move || {
                let lexer = AIRLexer::new(SOURCE_CODE_GOOD);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), lexer)
                    .expect("success")
            })
        },
    );
}

fn parse_to_fail(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    c.bench_function(
        format!("parse {} bytes to FAIL", SOURCE_CODE_BAD.len()).as_str(),
        move |b| {
            let parser = parser.clone();
            b.iter(move || {
                let lexer = AIRLexer::new(SOURCE_CODE_BAD);
                parser.clone().parse("", &mut Vec::new(), lexer)
            })
        },
    );
}

fn parse_deep(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    let source_code: Vec<_> = (1..10).map(gen::deep_seq).collect();
    let index: Vec<_> = source_code
        .iter()
        .enumerate()
        .map(|(i, code)| (i, code.len()))
        .collect();

    c.bench_function_over_inputs(
        "parse generated script",
        move |b, (i, _)| {
            let parser = parser.clone();
            let code = &source_code[*i];
            b.iter(move || {
                let lexer = AIRLexer::new(code);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), lexer)
                    .expect("success")
            });
        },
        index,
    );
}

criterion_group!(
    parser,
    create_parser,
    parse,
    parse_to_fail,
    parse_deep,
    clone_parser,
    clone_parser_rc,
);
criterion_main!(parser);
