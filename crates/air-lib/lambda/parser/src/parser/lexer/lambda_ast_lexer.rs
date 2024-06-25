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

use super::errors::LexerError;
use super::token::Token;
use crate::parser::lexer::is_air_alphanumeric;

use std::iter::Peekable;
use std::str::CharIndices;

const ARRAY_IDX_BASE: u32 = 10;
const LENGTH_FUNCTOR: &str = ".length";
const VALUE_PATH_STARTER: &str = ".$";

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct LambdaASTLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    is_first_token: bool,
}

impl<'input> Iterator for LambdaASTLexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'input> LambdaASTLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            is_first_token: true,
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Token<'input>, usize, LexerError>> {
        if self.input.is_empty() {
            return None;
        }

        if self.is_first_token {
            self.is_first_token = false;
            return Some(self.try_parse_first_token());
        }

        self.chars.next().map(|(start_offset, ch)| match ch {
            '[' => Ok((start_offset, Token::OpenSquareBracket, start_offset + 1)),
            ']' => Ok((start_offset, Token::CloseSquareBracket, start_offset + 1)),

            '.' => Ok((start_offset, Token::ValuePathSelector, start_offset + 1)),

            d if d.is_digit(ARRAY_IDX_BASE) => self.tokenize_arrays_idx(start_offset),
            s if is_air_alphanumeric(s) => self.tokenize_field_name(start_offset),

            '!' => Ok((start_offset, Token::FlatteningSign, start_offset + 1)),

            _ => Err(LexerError::UnexpectedSymbol(start_offset, start_offset + 1)),
        })
    }

    fn tokenize_arrays_idx(
        &mut self,
        start_offset: usize,
    ) -> Spanned<Token<'input>, usize, LexerError> {
        let array_idx = self.tokenize_until(start_offset, |ch| ch.is_digit(ARRAY_IDX_BASE));
        match array_idx
            .parse::<u32>()
            .map_err(|e| LexerError::ParseIntError(start_offset, start_offset + array_idx.len(), e))
        {
            Ok(idx) => Ok((
                start_offset,
                Token::NumberAccessor(idx),
                start_offset + array_idx.len(),
            )),
            Err(e) => Err(e),
        }
    }

    fn tokenize_field_name(
        &mut self,
        start_offset: usize,
    ) -> Spanned<Token<'input>, usize, LexerError> {
        let field_name = self.tokenize_until(start_offset, is_air_alphanumeric);

        Ok((
            start_offset,
            Token::StringAccessor(field_name),
            start_offset + field_name.len(),
        ))
    }

    fn tokenize_until(
        &mut self,
        start_offset: usize,
        condition: impl Fn(char) -> bool,
    ) -> &'input str {
        let mut end_pos = start_offset;
        while let Some((pos, ch)) = self.chars.peek() {
            if !condition(*ch) {
                break;
            }
            end_pos = *pos;
            self.chars.next();
        }

        &self.input[start_offset..end_pos + 1]
    }

    fn try_parse_first_token(&mut self) -> Spanned<Token<'input>, usize, LexerError> {
        let (token, token_size) = if self.input == LENGTH_FUNCTOR {
            (Token::LengthFunctor, LENGTH_FUNCTOR.len())
        } else if self.input.starts_with(VALUE_PATH_STARTER) {
            (Token::ValuePathStarter, VALUE_PATH_STARTER.len())
        } else {
            return Err(LexerError::UnexpectedSymbol(0, self.input.len()));
        };

        self.advance_by(token_size);
        Ok((0, token, token_size))
    }

    fn advance_by(&mut self, advance_size: usize) {
        // advance_by is unstable
        for _ in 0..advance_size {
            self.chars.next();
        }
    }
}
