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

use crate::aqua;
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

#[test]
fn parse_aqua() {
    let parser = aqua::InstrParser::new();
    let parse = |s| -> Result<Box<Instruction<'_>>, Vec<ErrorRecovery<_, _, _>>> {
        let mut errors = Vec::new();
        match parser.parse(&mut errors, s) {
            Ok(r) if errors.is_empty() => Ok(r),
            Ok(_) => {
                for error in errors.iter() {
                    println!("Parse error: {:?}", error);
                }
                Err(errors)
            }
            Err(_) => {
                for error in errors.iter() {
                    println!("Parse error: {:?}", error);
                }
                Err(errors)
            }
        }
    };

    let _: Box<Instruction> = parse("(call peerid function)").unwrap();

    let mut files = SimpleFiles::new();
    let source_code = r#"
    (seq
        (call peerid function)
        (call id)
        ()
    )
    "#;
    let file_id = files.add("seq.aqua", unindent::unindent(source_code));

    match parse(source_code) {
        Err(errors) => {
            let labels = errors
                .into_iter()
                .map(|err| match err.error {
                    ParseError::UnrecognizedEOF { location, expected } => {
                        Label::primary(file_id, location..location)
                            .with_message(format!("expected {:?}", expected))
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
        }
        _ => {}
    }
}
