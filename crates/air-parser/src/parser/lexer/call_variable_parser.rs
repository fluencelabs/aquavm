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

use super::LexerError;
use super::LexerResult;
use super::Number;
use super::Token;

use std::iter::Peekable;
use std::str::CharIndices;

pub(super) fn try_parse_call_variable(
    string_to_parse: &str,
    start_pos: usize,
) -> LexerResult<Token<'_>> {
    CallVariableParser::new(string_to_parse, start_pos).try_parse()
}

#[derive(Debug)]
struct ParserState {
    pub(self) dot_met_pos: Option<usize>,
    pub(self) non_numeric_met: bool,
    pub(self) is_sign_met: bool,
    pub(self) is_first_char: bool,
    pub(self) current_char: Option<char>,
    pub(self) current_pos: Option<usize>,
}

struct CallVariableParser<'input> {
    string_to_parse_iter: Peekable<CharIndices<'input>>,
    string_to_parse: &'input str,
    start_pos: usize,
    state: ParserState,
}

impl<'input> CallVariableParser<'input> {
    pub(self) fn new(string_to_parse: &'input str, start_pos: usize) -> Self {
        let string_to_parse_iter = string_to_parse.char_indices().peekable();
        let state = ParserState {
            dot_met_pos: None,
            non_numeric_met: false,
            is_sign_met: false,
            is_first_char: true,
            current_char: None,
            current_pos: None,
        };

        Self {
            string_to_parse_iter,
            string_to_parse,
            start_pos,
            state,
        }
    }

    pub(self) fn try_parse(mut self) -> LexerResult<Token<'input>> {
        while self.next_char() {
            if self.is_it_possible_to_parse_as_number() {
                self.try_parse_as_number()?;
            } else {
                self.try_parse_as_variable()?;
            }

            self.state.is_first_char = false;
        }

        self.to_token()
    }

    fn next_char(&mut self) -> bool {
        let (pos, ch) = match self.string_to_parse_iter.next() {
            Some(pos_and_ch) => pos_and_ch,
            None => return false,
        };

        self.state.current_char = Some(ch);
        self.state.current_pos = Some(pos);

        true
    }

    fn is_it_possible_to_parse_as_number(&self) -> bool {
        !self.state.non_numeric_met
    }

    fn try_parse_as_number(&mut self) -> LexerResult<()> {
        if self.try_parse_sign() || self.try_parse_digit() || self.try_parse_float_dot() {
            return Ok(());
        }

        self.handle_non_digit()
    }

    fn try_parse_sign(&self) -> bool {
        let ch = self.state.current_char.unwrap();
        self.state.is_first_char && (ch == '-' || ch == '+')
    }

    fn try_parse_digit(&self) -> bool {
        self.state.current_char.unwrap().is_numeric()
    }

    fn try_parse_float_dot(&mut self) -> bool {
        self.try_parse_first_met_dot()
    }

    fn handle_non_digit(&mut self) -> LexerResult<()> {
        self.state.non_numeric_met = true;

        if self.state.is_sign_met {
            return Err(LexerError::UnallowedCharInNumber(
                self.current_pos(),
                self.current_pos(),
            ));
        }

        self.try_parse_as_variable()
    }

    fn try_parse_as_variable(&mut self) -> LexerResult<()> {
        if self.try_parse_json_path_start() {
            return Ok(());
        } else if self.is_json_path_started() {
            self.try_parse_json_path()?;
        } else {
            self.try_parse_alphanumeric()?;
        }

        Ok(())
    }

    fn try_parse_json_path_start(&mut self) -> bool {
        self.try_parse_first_met_dot()
    }

    fn try_parse_alphanumeric(&self) -> LexerResult<()> {
        if !self.aqua_alphanumeric() {
            return Err(LexerError::IsNotAlphanumeric(
                self.current_pos(),
                self.current_pos(),
            ));
        }

        Ok(())
    }

    fn try_parse_json_path(&self) -> LexerResult<()> {
        if !self.json_path_allowed_char() {
            return Err(LexerError::InvalidJsonPath(
                self.current_pos(),
                self.current_pos(),
            ));
        }

        Ok(())
    }

    fn try_parse_first_met_dot(&mut self) -> bool {
        if !self.dot_met() && self.state.current_char.unwrap() == '.' {
            self.state.dot_met_pos = Some(self.state.current_pos.unwrap());
            return true;
        }

        false
    }

    fn is_json_path_started(&self) -> bool {
        self.dot_met()
    }

    fn dot_met(&self) -> bool {
        self.state.dot_met_pos.is_some()
    }

    fn aqua_alphanumeric(&self) -> bool {
        super::is_aqua_alphanumeric(self.state.current_char.unwrap())
    }

    fn json_path_allowed_char(&self) -> bool {
        super::is_json_path_allowed_char(self.state.current_char.unwrap())
    }

    fn current_pos(&self) -> usize {
        self.start_pos + self.state.current_pos.unwrap()
    }

    fn to_token(&self) -> LexerResult<Token<'input>> {
        match (self.is_it_possible_to_parse_as_number(), self.dot_met()) {
            (true, false) => {
                let number = self
                    .string_to_parse
                    .parse::<i64>()
                    .map_err(|e| self.to_parse_int_error(e))?;
                let number = Number::Int(number);
                Ok(Token::Number(number))
            }
            (true, true) => {
                // TODO: check float
                let number = self
                    .string_to_parse
                    .parse::<f64>()
                    .map_err(|e| self.to_parse_float_error(e))?;
                let number = Number::Float(number);
                Ok(Token::Number(number))
            }
            (false, false) => Ok(Token::Alphanumeric(self.string_to_parse)),
            (false, true) => Ok(Token::JsonPath(
                self.string_to_parse,
                self.state.dot_met_pos.unwrap(),
            )),
        }
    }

    fn to_parse_int_error(&self, e: std::num::ParseIntError) -> LexerError {
        LexerError::ParseIntError(
            self.start_pos,
            self.start_pos + self.string_to_parse.len(),
            e,
        )
    }

    fn to_parse_float_error(&self, e: std::num::ParseFloatError) -> LexerError {
        LexerError::ParseFloatError(
            self.start_pos,
            self.start_pos + self.string_to_parse.len(),
            e,
        )
    }
}
