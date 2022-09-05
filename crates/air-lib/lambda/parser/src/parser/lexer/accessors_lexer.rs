/*
 * Copyright 2021 Fluence Labs Limited
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

use super::errors::LexerError;
use super::token::Token;
use crate::parser::lexer::is_air_alphanumeric;

use std::iter::Peekable;
use std::str::CharIndices;

const ARRAY_IDX_BASE: u32 = 10;
const LENGTH_FUNCTOR: &str = "length";
const VALUE_PATH_STARTER: &str = ".$";

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct AccessorsLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Iterator for AccessorsLexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'input> AccessorsLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Token<'input>, usize, LexerError>> {
        self.chars.next().map(|(start_pos, ch)| match ch {
            '[' => Ok((start_pos, Token::OpenSquareBracket, start_pos + 1)),
            ']' => Ok((start_pos, Token::CloseSquareBracket, start_pos + 1)),

            '.' => Ok((start_pos, Token::ValuePathSelector, start_pos + 1)),

            LENGTH_FUNCTOR => Ok((start_pos, Token::LengthFunctor, start_pos + LENGTH_FUNCTOR.len())),
            VALUE_PATH_STARTER => Ok((start_pos, Token::ValuePathStarter, start_pos + VALUE_PATH_STARTER.len())),

            d if d.is_digit(ARRAY_IDX_BASE) => self.tokenize_arrays_idx(start_pos),
            s if is_air_alphanumeric(s) => self.tokenize_field_name(start_pos),

            '!' => Ok((start_pos, Token::FlatteningSign, start_pos + 1)),

            _ => Err(LexerError::UnexpectedSymbol(start_pos, start_pos + 1)),
        })
    }

    fn tokenize_arrays_idx(
        &mut self,
        start_pos: usize,
    ) -> Spanned<Token<'input>, usize, LexerError> {
        let array_idx = self.tokenize_until(start_pos, |ch| ch.is_digit(ARRAY_IDX_BASE));
        match array_idx
            .parse::<u32>()
            .map_err(|e| LexerError::ParseIntError(start_pos, start_pos + array_idx.len(), e))
        {
            Ok(idx) => Ok((
                start_pos,
                Token::NumberAccessor(idx),
                start_pos + array_idx.len(),
            )),
            Err(e) => Err(e),
        }
    }

    fn tokenize_field_name(
        &mut self,
        start_pos: usize,
    ) -> Spanned<Token<'input>, usize, LexerError> {
        let field_name = self.tokenize_until(start_pos, is_air_alphanumeric);

        Ok((
            start_pos,
            Token::StringAccessor(field_name),
            start_pos + field_name.len(),
        ))
    }

    fn tokenize_until(
        &mut self,
        start_pos: usize,
        condition: impl Fn(char) -> bool,
    ) -> &'input str {
        let mut end_pos = start_pos;
        while let Some((pos, ch)) = self.chars.peek() {
            if !condition(*ch) {
                break;
            }
            end_pos = *pos;
            self.chars.next();
        }

        &self.input[start_pos..end_pos + 1]
    }

    fn tokenize_string(string_to_parse: &str, start_pos: usize) -> Spanned<Token<'input>, usize, LexerError> {
        if string_to_parse == LENGTH_FUNCTOR {
            return Ok((start_pos, Token::LengthFunctor, start_pos + LENGTH_FUNCTOR.len()));
        } else if string_to_parse.starts_with(VALUE_PATH_STARTER) {
            return Ok((start_pos, Token::ValuePathStarter, start_pos + VALUE_PATH_STARTER.len()))
        }

        Err(LexerError::UnexpectedSymbol(start_pos, start_pos + 1))
    }
}
