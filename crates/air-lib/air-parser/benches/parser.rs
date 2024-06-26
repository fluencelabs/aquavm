/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::rc::Rc;

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
