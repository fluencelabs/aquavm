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

use std::rc::Rc;

use air_parser::ast::Instruction;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use air_parser::AIRLexer;
use air_parser::AIRParser;
use air_parser::VariableValidator;

const SOURCE_CODE_BAD: &str = r#"(seq
        (seq
            (call node ("identity" "") [] $void)
            (call provider (service_id "{fname}") {arg_list} result)
        )
        (seq
            (call node ("identity" "") [] $void)
            (call "{LOCAL_VM}" ("return" "result") [result] $void)
        )
    )"#;

const SOURCE_CODE_GOOD: &str = r#"
    (seq
        (seq
            (call node ("identity" "") [] $void)
            (call provider (service_id "fname") [arg list] result)
        )
        (seq
            (call node ("identity" "") [] $void)
            (call "local_vm" ("return" "result") [result] $void)
        )
    )"#;

#[cfg(test)]
mod gen {
    use crate::SOURCE_CODE_GOOD;

    pub fn seq(left: &str, right: &str) -> String {
        format!(r"(seq {left} {right})")
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
    c.bench_function("create_parser", move |b| b.iter(AIRParser::new));
}

fn parse(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    c.bench_function(
        format!("parse {} bytes", SOURCE_CODE_GOOD.len()).as_str(),
        move |b| {
            let parser = parser.clone();
            b.iter(move || {
                let mut validator = VariableValidator::new();
                let lexer = AIRLexer::new(SOURCE_CODE_GOOD);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), &mut validator, lexer)
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
                let mut validator = VariableValidator::new();
                let lexer = AIRLexer::new(SOURCE_CODE_BAD);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), &mut validator, lexer)
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

    let mut group = c.benchmark_group("parse generated script");
    for (i, _) in index {
        group.bench_function(i.to_string(), |b| {
            let parser = parser.clone();
            let code = &source_code[i];
            b.iter(move || {
                let mut validator = VariableValidator::new();
                let lexer = AIRLexer::new(code);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), &mut validator, lexer)
                    .expect("success")
            });
        });
    }
}

fn parse_dashboard_script(c: &mut Criterion) {
    let parser = Rc::new(AIRParser::new());
    const DASHBOARD_SCRIPT: &str =
        include_str!("../../../../air/tests/test_module/integration/scripts/dashboard.air");

    c.bench_function(
        format!("parse {} bytes", DASHBOARD_SCRIPT.len()).as_str(),
        move |b| {
            let parser = parser.clone();
            b.iter(move || {
                let mut validator = VariableValidator::new();
                let lexer = AIRLexer::new(DASHBOARD_SCRIPT);

                parser
                    .clone()
                    .parse("", &mut Vec::new(), &mut validator, lexer)
                    .expect("success")
            })
        },
    );
}

criterion_group!(
    parser,
    create_parser,
    parse,
    parse_to_fail,
    parse_dashboard_script,
    parse_deep,
);
criterion_main!(parser);
