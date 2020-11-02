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

use super::aqua;
use crate::ast::Instruction;

use lalrpop_util::{ErrorRecovery, ParseError};
use std::fmt::Formatter;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};

#[derive(Debug)]
pub enum InstructionError {
    InvalidPeerId,
}

impl std::error::Error for InstructionError {}
impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionError")
    }
}

fn parse(source_code: &str) -> Box<Instruction> {
    let mut files = SimpleFiles::new();
    let file_id = files.add("script.aqua", source_code);

    let parse = |s| -> Result<Box<Instruction<'_>>, Vec<ErrorRecovery<_, _, _>>> {
        let parser = aqua::InstrParser::new();
        let mut errors = Vec::new();
        match parser.parse(&mut errors, s) {
            Ok(r) if errors.is_empty() => Ok(r),
            Ok(_) => {
                for error in errors.iter() {
                    println!("Parse error: {:?}", error);
                }
                Err(errors)
            }
            Err(err) => {
                println!("Parsing failed: {:?}", err);
                Err(errors)
            }
        }
    };

    match parse(source_code) {
        Err(errors) => {
            let labels = errors
                .into_iter()
                .map(|err| match err.error {
                    ParseError::UnrecognizedToken {
                        token: (start, token, end),
                        expected,
                    } => {
                        Label::primary(file_id, start..end).with_message(format!("expected {}", {
                            if expected.is_empty() {
                                "<nothing>".to_string()
                            } else {
                                expected.join(" or ")
                            }
                        }))
                    }
                    err => unimplemented!("parse error not implemented: {:?}", err),
                    /*
                        ParseError::InvalidToken { .. } => {}
                        ParseError::UnrecognizedToken { .. } => {}
                        ParseError::ExtraToken { .. } => {}
                        ParseError::User { .. } => {}
                    */
                })
                .collect();
            let diagnostic = Diagnostic::error()
                .with_message("some error")
                .with_labels(labels);

            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = codespan_reporting::term::Config::default();

            term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("term emit");
            panic!("parsing failed");
        }
        Ok(r) => r,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use CallOutput::*;
    use FunctionPart::*;
    use PeerPart::*;
    use Value::*;

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
                (call (peerid serviceA) ("serviceB" function) () void)
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
                    peer: PeerPkWithServiceId(Variable("peerid"), Variable("serviceA")),
                    f: ServiceIdWithFuncName(Literal("serviceB"), Variable("function")),
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

    #[test]
    fn parse_json_path() {
        let source_code = r#"
        (call id.$.a "f" ("hello" name) void[])
        "#;
        let instruction = *parse(source_code);
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
}
