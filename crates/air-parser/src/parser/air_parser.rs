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

use super::air;
use super::ast::Instruction;
use super::lexer::AIRLexer;
use super::lexer::LexerError;
use super::lexer::Token;
use super::ParserError;

use crate::parser::VariableValidator;
use air::AIRParser;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{Buffer, ColorChoice, StandardStream};
use lalrpop_util::{ErrorRecovery, ParseError};

// Caching parser to cache internal regexes, which are expensive to instantiate
// See also https://github.com/lalrpop/lalrpop/issues/269
thread_local!(static PARSER: AIRParser = AIRParser::new());

/// Parse AIR `source_code` to `Box<Instruction>`
pub fn parse(air_script: &str) -> Result<Box<Instruction<'_>>, String> {
    let mut files = SimpleFiles::new();
    let file_id = files.add("script.aqua", air_script);

    PARSER.with(|parser| {
        let mut errors = Vec::new();
        let lexer = AIRLexer::new(air_script);
        let mut validator = VariableValidator::new();
        let result = parser.parse(air_script, &mut errors, &mut validator, lexer);

        let validator_errors = validator.finalize();
        errors.extend(validator_errors);

        match result {
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
    errors: Vec<ErrorRecovery<usize, Token<'_>, ParserError>>,
) -> String {
    let labels = errors_to_labels(file_id, errors);
    let diagnostic = Diagnostic::error().with_labels(labels);

    // Write to stderr
    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = codespan_reporting::term::Config::default();
    term::emit(&mut writer.lock(), &config, &files, &diagnostic).expect("term emit to stderr");

    // Return as a string
    let mut buffer = Buffer::no_color();
    term::emit(&mut buffer, &config, &files, &diagnostic).expect("term emit to buffer");
    String::from_utf8_lossy(buffer.as_slice())
        .as_ref()
        .to_string()
}

fn errors_to_labels(
    file_id: usize,
    errors: Vec<ErrorRecovery<usize, Token<'_>, ParserError>>,
) -> Vec<Label<usize>> {
    errors
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
            ParseError::User { error } => parser_error_to_label(file_id, error),
        })
        .collect()
}

fn pretty_expected(expected: Vec<String>) -> String {
    if expected.is_empty() {
        "<nothing>".to_string()
    } else {
        expected.join(" or ")
    }
}

fn parser_error_to_label(file_id: usize, error: ParserError) -> Label<usize> {
    use ParserError::*;

    match error {
        LexerError(error) => lexical_error_to_label(file_id, error),
        CallArgsNotFlattened(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        UndefinedIterable(start, end, _) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        UndefinedVariable(start, end, _) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
    }
}

fn lexical_error_to_label(file_id: usize, error: LexerError) -> Label<usize> {
    use LexerError::*;
    match error {
        UnclosedQuote(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        EmptyString(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        IsNotAlphanumeric(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        EmptyAccName(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        EmptyVariableOrConst(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        InvalidJsonPath(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        UnallowedCharInNumber(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        ParseIntError(start, end, _) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        ParseFloatError(start, end, _) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        TooBigFloat(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
        LeadingDot(start, end) => {
            Label::primary(file_id, start..end).with_message(error.to_string())
        }
    }
}

pub(super) fn into_variable_and_path(str: &str, pos: usize, should_flatten: bool) -> (&str, &str) {
    let json_path = if should_flatten {
        &str[pos + 1..str.len() - 1]
    } else {
        &str[pos + 1..]
    };

    (&str[0..pos], json_path)
}

pub(super) fn make_flattened_error(
    start_pos: usize,
    token: Token<'_>,
    end_pos: usize,
) -> ErrorRecovery<usize, Token<'_>, ParserError> {
    let error = ParserError::CallArgsNotFlattened(start_pos, end_pos);
    let error = ParseError::User { error };

    let dropped_tokens = vec![(start_pos, token, end_pos)];

    ErrorRecovery {
        error,
        dropped_tokens,
    }
}
