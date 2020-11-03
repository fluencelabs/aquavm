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
use codespan_reporting::term::termcolor::Buffer;
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};

#[derive(Debug)]
pub enum InstructionError {
    #[allow(dead_code)]
    InvalidPeerId,
}

impl std::error::Error for InstructionError {}
impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionError")
    }
}

// Caching parser to cache internal regexes, which are expensive to instantiate
// See also https://github.com/lalrpop/lalrpop/issues/269
thread_local!(static PARSER: aqua::InstrParser = aqua::InstrParser::new());

pub fn parse(source_code: &str) -> Result<Box<Instruction>, String> {
    let mut files = SimpleFiles::new();
    let file_id = files.add("script.aqua", source_code);

    let parse = |s| -> Result<Box<Instruction<'_>>, Vec<ErrorRecovery<_, _, _>>> {
        // let parser = aqua::InstrParser::new();
        PARSER.with(|parser| {
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
                    Err(vec![ErrorRecovery {
                        error: err,
                        dropped_tokens: vec![],
                    }])
                }
            }
        })
    };

    match parse(source_code.as_ref()) {
        Err(errors) => {
            let labels: Vec<Label<usize>> = errors
                .into_iter()
                .map(|err| match err.error {
                    ParseError::UnrecognizedToken {
                        token: (start, _, end),
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
                    ParseError::InvalidToken { location } => {
                        Label::primary(file_id, location..(location + 1))
                            .with_message("unexpected token")
                    }
                    err => unimplemented!("parse error not implemented: {:?}", err),
                    /*

                        ParseError::UnrecognizedToken { .. } => {}
                        ParseError::ExtraToken { .. } => {}
                        ParseError::User { .. } => {}
                    */
                })
                .collect();
            println!("labels {}", labels.len());
            let diagnostic = Diagnostic::error().with_labels(labels);

            let writer = StandardStream::stderr(ColorChoice::Auto);
            let config = codespan_reporting::term::Config::default();
            term::emit(&mut writer.lock(), &config, &files, &diagnostic)
                .expect("term emit to stderr");

            let mut buffer = Buffer::no_color();
            term::emit(&mut buffer, &config, &files, &diagnostic).expect("term emit to buffer");
            Err(String::from_utf8_lossy(buffer.as_slice())
                .as_ref()
                .to_string())
        }
        Ok(r) => Ok(r),
    }
}
