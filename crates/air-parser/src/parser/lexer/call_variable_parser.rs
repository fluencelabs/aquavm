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
    pub(self) first_dot_met_pos: Option<usize>,
    pub(self) non_numeric_met: bool,
    pub(self) digit_met: bool,
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
            first_dot_met_pos: None,
            non_numeric_met: false,
            digit_met: false,
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
            if self.is_possible_to_parse_as_number() {
                self.try_parse_as_number()?;
            } else {
                self.try_parse_as_variable()?;
            }

            self.set_first_char_met()
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

    fn is_possible_to_parse_as_number(&self) -> bool {
        !self.state.non_numeric_met
    }

    fn try_parse_as_number(&mut self) -> LexerResult<()> {
        if self.try_parse_as_sign() || self.try_parse_as_digit() || self.try_parse_as_float_dot()? {
            return Ok(());
        }

        self.handle_non_digit()
    }

    fn try_parse_as_sign(&self) -> bool {
        let ch = self.current_char();
        self.state.is_first_char && (ch == '-' || ch == '+')
    }

    fn try_parse_as_digit(&mut self) -> bool {
        if self.current_char().is_numeric() {
            self.state.digit_met = true;
            return true;
        }

        false
    }

    fn try_parse_as_float_dot(&mut self) -> LexerResult<bool> {
        // filter out +.12 -.2315 variants
        if self.try_parse_first_met_dot()? && self.state.digit_met == false {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::LeadingDot(error_pos, error_pos));
        }

        Ok(self.dot_met())
    }

    fn handle_non_digit(&mut self) -> LexerResult<()> {
        self.check_fallback_to_variable()?;

        self.state.non_numeric_met = true;
        self.try_parse_as_variable()
    }

    fn check_fallback_to_variable(&self) -> LexerResult<()> {
        println!("dot met is {}", self.dot_met());

        if self.dot_met() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::UnallowedCharInNumber(error_pos, error_pos));
        }

        Ok(())
    }

    fn try_parse_as_variable(&mut self) -> LexerResult<()> {
        if self.try_parse_as_json_path_start()? {
            return Ok(());
        } else if self.is_json_path_started() {
            self.try_parse_as_json_path()?;
        } else {
            self.try_parse_as_alphanumeric()?;
        }

        Ok(())
    }

    fn try_parse_as_json_path_start(&mut self) -> LexerResult<bool> {
        self.try_parse_first_met_dot()
    }

    fn try_parse_as_alphanumeric(&self) -> LexerResult<()> {
        if !self.aqua_alphanumeric() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::IsNotAlphanumeric(error_pos, error_pos));
        }

        Ok(())
    }

    fn try_parse_as_json_path(&self) -> LexerResult<()> {
        if !self.json_path_allowed_char() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::InvalidJsonPath(error_pos, error_pos));
        }

        Ok(())
    }

    fn try_parse_first_met_dot(&mut self) -> LexerResult<bool> {
        if !self.dot_met() && self.current_char() == '.' {
            if self.current_pos() == 0 {
                return Err(LexerError::LeadingDot(
                    self.start_pos,
                    self.pos_in_string_to_parse(),
                ));
            }
            self.state.first_dot_met_pos = Some(self.current_pos());
            return Ok(true);
        }

        Ok(false)
    }

    fn is_json_path_started(&self) -> bool {
        self.dot_met()
    }

    fn dot_met(&self) -> bool {
        self.state.first_dot_met_pos.is_some()
    }

    fn aqua_alphanumeric(&self) -> bool {
        super::is_aqua_alphanumeric(self.current_char())
    }

    fn json_path_allowed_char(&self) -> bool {
        super::is_json_path_allowed_char(self.current_char())
    }

    fn pos_in_string_to_parse(&self) -> usize {
        self.start_pos + self.current_pos()
    }

    fn set_first_char_met(&mut self) {
        self.state.is_first_char = false;
    }

    // this function should be called after the self.next_char()
    fn current_pos(&self) -> usize {
        self.state.current_pos.unwrap()
    }

    // this function should be called after the self.next_char()
    fn current_char(&self) -> char {
        self.state.current_char.unwrap()
    }

    fn to_token(&self) -> LexerResult<Token<'input>> {
        match (self.is_possible_to_parse_as_number(), self.dot_met()) {
            (true, false) => {
                let number = self
                    .string_to_parse
                    .parse::<i64>()
                    .map_err(|e| self.to_parse_int_error(e))?;
                let number = Number::Int(number);
                Ok(Token::Number(number))
            }
            (true, true) => {
                if self.string_to_parse.len() > 11 {
                    return Err(LexerError::TooBigFloat(
                        self.start_pos,
                        self.string_to_parse.len(),
                    ));
                }

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
                self.state.first_dot_met_pos.unwrap(),
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
