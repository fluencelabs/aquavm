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
use super::Token;
use crate::LambdaAST;

use std::iter::Peekable;
use std::str::CharIndices;

pub(super) fn try_parse_call_variable(
    string_to_parse: &str,
    start_pos: usize,
) -> LexerResult<Token<'_>> {
    CallVariableParser::try_parse(string_to_parse, start_pos)
}

#[derive(Debug)]
enum StreamTagMet {
    None,
    Stream,
    CanonStream,
}

#[derive(Debug)]
struct ParserState {
    pub(self) first_dot_met_pos: Option<usize>,
    pub(self) non_numeric_met: bool,
    pub(self) digit_met: bool,
    pub(self) flattening_met: bool,
    pub(self) is_first_char: bool,
    pub(self) is_first_stream_tag: StreamTagMet,
    pub(self) current_char: char,
    pub(self) current_pos: usize,
}

struct CallVariableParser<'input> {
    string_to_parse_iter: Peekable<CharIndices<'input>>,
    string_to_parse: &'input str,
    start_pos: usize,
    state: ParserState,
}

impl<'input> CallVariableParser<'input> {
    fn new(string_to_parse: &'input str, start_pos: usize) -> LexerResult<Self> {
        let mut string_to_parse_iter = string_to_parse.char_indices().peekable();
        let (current_pos, current_char) = match string_to_parse_iter.next() {
            Some(pos_and_ch) => pos_and_ch,
            None => return Err(LexerError::empty_variable_or_const(start_pos..start_pos)),
        };

        let state = ParserState {
            first_dot_met_pos: None,
            non_numeric_met: false,
            digit_met: false,
            flattening_met: false,
            is_first_char: true,
            is_first_stream_tag: StreamTagMet::None,
            current_char,
            current_pos,
        };

        let parser = Self {
            string_to_parse_iter,
            string_to_parse,
            start_pos,
            state,
        };

        Ok(parser)
    }

    pub(self) fn try_parse(
        string_to_parse: &'input str,
        start_pos: usize,
    ) -> LexerResult<Token<'input>> {
        let mut parser = Self::new(string_to_parse, start_pos)?;

        loop {
            if parser.is_possible_to_parse_as_number() {
                parser.try_parse_as_number()?;
            } else {
                parser.try_parse_as_variable()?;
            }

            if !parser.next_char() {
                break;
            }
        }

        parser.to_token()
    }

    fn next_char(&mut self) -> bool {
        let (pos, ch) = match self.string_to_parse_iter.next() {
            Some(pos_and_ch) => pos_and_ch,
            None => return false,
        };

        self.state.current_char = ch;
        self.state.current_pos = pos;
        self.state.is_first_char = false;

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
        let is_first_dot = self.try_parse_first_met_dot()?;

        // filter out +.12 -.2315 variants
        if is_first_dot && !self.state.digit_met {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::leading_dot(error_pos..error_pos));
        }

        Ok(is_first_dot)
    }

    fn handle_non_digit(&mut self) -> LexerResult<()> {
        self.check_fallback_to_variable()?;

        self.state.non_numeric_met = true;
        self.try_parse_as_variable()
    }

    fn check_fallback_to_variable(&self) -> LexerResult<()> {
        if self.dot_met() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::unallowed_char_in_number(error_pos..error_pos));
        }

        Ok(())
    }

    fn try_parse_as_variable(&mut self) -> LexerResult<()> {
        if self.try_parse_as_stream_start()? || self.try_parse_as_json_path_start()? {
            return Ok(());
        } else if self.is_json_path_started() {
            self.try_parse_as_json_path()?;
        } else {
            self.try_parse_as_alphanumeric()?;
        }

        Ok(())
    }

    fn try_parse_as_stream_start(&mut self) -> LexerResult<bool> {
        let stream_tag = StreamTagMet::from_tag(self.current_char());
        if self.current_pos() == 0 && stream_tag.is_tag() {
            if self.string_to_parse.len() == 1 {
                let error_pos = self.pos_in_string_to_parse();
                return Err(LexerError::empty_stream_name(error_pos..error_pos));
            }

            self.state.is_first_stream_tag = stream_tag;
            return Ok(true);
        }

        Ok(false)
    }

    fn try_parse_as_json_path_start(&mut self) -> LexerResult<bool> {
        self.try_parse_first_met_dot()
    }

    fn try_parse_as_alphanumeric(&self) -> LexerResult<()> {
        if !self.air_alphanumeric() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::is_not_alphanumeric(error_pos..error_pos));
        }

        Ok(())
    }

    fn try_parse_as_json_path(&mut self) -> LexerResult<()> {
        if !self.json_path_allowed_char() && !self.try_parse_as_flattening() {
            let error_pos = self.pos_in_string_to_parse();
            return Err(LexerError::invalid_lambda(error_pos..error_pos));
        }

        Ok(())
    }

    fn try_parse_as_flattening(&mut self) -> bool {
        if self.is_last_char() && self.current_char() == '!' {
            self.state.flattening_met = true;
            return true;
        }

        false
    }

    fn try_parse_first_met_dot(&mut self) -> LexerResult<bool> {
        if !self.dot_met() && self.current_char() == '.' {
            if self.current_pos() == 0 {
                return Err(LexerError::leading_dot(
                    self.start_pos..self.pos_in_string_to_parse(),
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

    fn air_alphanumeric(&self) -> bool {
        super::is_air_alphanumeric(self.current_char())
    }

    fn json_path_allowed_char(&self) -> bool {
        super::is_json_path_allowed_char(self.current_char())
    }

    fn pos_in_string_to_parse(&self) -> usize {
        self.start_pos + self.current_pos()
    }

    fn current_pos(&self) -> usize {
        self.state.current_pos
    }

    fn current_char(&self) -> char {
        self.state.current_char
    }

    fn is_last_char(&self) -> bool {
        self.current_pos() == self.string_to_parse.len() - 1
    }

    fn to_variable_token<'v>(&self, name: &'v str) -> Token<'v> {
        match self.state.is_first_stream_tag {
            StreamTagMet::None => Token::Scalar {
                name,
                position: self.start_pos,
            },
            StreamTagMet::Stream => Token::Stream {
                name,
                position: self.start_pos,
            },
            StreamTagMet::CanonStream => Token::CanonStream {
                name,
                position: self.start_pos,
            },
        }
    }

    fn to_variable_token_with_lambda<'v>(&self, name: &'v str, lambda: LambdaAST<'v>) -> Token<'v> {
        match self.state.is_first_stream_tag {
            StreamTagMet::None => Token::ScalarWithLambda {
                name,
                lambda,
                position: self.start_pos,
            },
            StreamTagMet::Stream => Token::StreamWithLambda {
                name,
                lambda,
                position: self.start_pos,
            },
            StreamTagMet::CanonStream => Token::CanonStreamWithLambda {
                name,
                lambda,
                position: self.start_pos,
            },
        }
    }

    fn try_to_variable_and_lambda(&self, lambda_start_pos: usize) -> LexerResult<Token<'input>> {
        // +2 to ignore ".$" prefix
        let lambda =
            crate::parse_lambda(&self.string_to_parse[lambda_start_pos + 2..]).map_err(|e| {
                LexerError::lambda_parser_error(
                    self.start_pos + lambda_start_pos..self.start_pos + self.string_to_parse.len(),
                    e.to_string(),
                )
            })?;

        let token =
            self.to_variable_token_with_lambda(&self.string_to_parse[0..lambda_start_pos], lambda);
        Ok(token)
    }

    fn try_to_i64(&self) -> LexerResult<Token<'input>> {
        let raw_value = self.string_to_parse;
        let number = raw_value.parse::<i64>().map_err(|e| {
            let start_pos = self.start_pos;
            LexerError::parse_int_error(start_pos..start_pos + raw_value.len(), e)
        })?;

        let token = Token::I64(number);
        Ok(token)
    }

    fn try_to_f64(&self) -> LexerResult<Token<'input>> {
        // safe threshold for floating-point numbers to obtain determinism
        const SAFE_FLOAT_SIGNIFICAND_SIZE: usize = 11;

        let raw_value = self.string_to_parse;
        let start_pos = self.start_pos;
        if raw_value.len() > SAFE_FLOAT_SIGNIFICAND_SIZE {
            return Err(LexerError::too_big_float(
                start_pos..start_pos + raw_value.len(),
            ));
        }

        let number = raw_value.parse::<f64>().map_err(|e| {
            LexerError::parse_float_error(start_pos..start_pos + raw_value.len(), e)
        })?;

        let token = Token::F64(number);
        Ok(token)
    }

    fn to_token(&self) -> LexerResult<Token<'input>> {
        let is_number = self.is_possible_to_parse_as_number();
        match (is_number, self.state.first_dot_met_pos) {
            (true, None) => self.try_to_i64(),
            (true, Some(_)) => self.try_to_f64(),
            (false, None) => Ok(self.to_variable_token(self.string_to_parse)),
            (false, Some(lambda_start_pos)) => self.try_to_variable_and_lambda(lambda_start_pos),
        }
    }
}

impl StreamTagMet {
    fn from_tag(tag: char) -> Self {
        match tag {
            '$' => Self::Stream,
            '#' => Self::CanonStream,
            _ => Self::None,
        }
    }

    fn is_tag(&self) -> bool {
        !matches!(self, Self::None)
    }
}
