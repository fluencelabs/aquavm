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
use crate::lalrpop::aqua::Token;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{Buffer, ColorChoice, StandardStream};
use lalrpop_util::{ErrorRecovery, ParseError};

use std::fmt::Formatter;

#[derive(Debug)]
/// Represents custom parsing errors. Isn't used yet.
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

/// Parse AIR `source_code` to `Box<Instruction>`
pub fn parse(source_code: &str) -> Result<Box<Instruction>, String> {
    let mut files = SimpleFiles::new();
    let file_id = files.add("script.aqua", source_code);

    PARSER.with(|parser| {
        let mut errors = Vec::new();
        match parser.parse(&mut errors, source_code) {
            Ok(r) if errors.is_empty() => Ok(r),
            Ok(_) => Err(report_errors(file_id, files, errors)),
            Err(err) => Err(report_errors(
                file_id,
                files,
                vec![ErrorRecovery {
                    error: err,
                    dropped_tokens: vec![],
                }],
            )),
        }
    })
}

fn report_errors(
    file_id: usize,
    files: SimpleFiles<&str, &str>,
    errors: Vec<ErrorRecovery<usize, Token, InstructionError>>,
) -> String {
    let labels: Vec<Label<usize>> = errors
        .into_iter()
        .map(|err| match err.error {
            ParseError::UnrecognizedToken {
                token: (start, _, end),
                expected,
            } => Label::primary(file_id, start..end)
                .with_message(format!("expected {}", pretty_expected(expected))),
            ParseError::InvalidToken { location } => {
                Label::primary(file_id, location..(location + 1)).with_message("unexpected token")
            }
            ParseError::ExtraToken {
                token: (start, _, end),
            } => Label::primary(file_id, start..end).with_message("extra token"),
            ParseError::UnrecognizedEOF { location, expected } => {
                Label::primary(file_id, location..(location + 1))
                    .with_message(format!("expected {}", pretty_expected(expected)))
            }
            // TODO: capture start & end in user error; maybe return it as a separate Diagnostic::error?
            ParseError::User { error } => {
                Label::primary(file_id, 0..0).with_message(error.to_string())
            }
        })
        .collect();
    let diagnostic = Diagnostic::error().with_labels(labels);
    let config = codespan_reporting::term::Config::default();

    // Write to stderr
    let writer = StandardStream::stderr(ColorChoice::Auto);
    term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("term emit to stderr");

    // Return as a string
    let mut buffer = Buffer::no_color();
    term::emit(&mut buffer, &config, &files, &diagnostic).expect("term emit to buffer");
    String::from_utf8_lossy(buffer.as_slice())
        .as_ref()
        .to_string()
}

fn pretty_expected(expected: Vec<String>) -> String {
    if expected.is_empty() {
        "<nothing>".to_string()
    } else {
        expected.join(" or ")
    }
}
