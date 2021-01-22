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

use super::errors::LexerError;
use super::token::Token;

use std::iter::Peekable;
use std::str::CharIndices;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct AIRLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Iterator for AIRLexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'input> AIRLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Token<'input>, usize, LexerError>> {
        while let Some((start_pos, ch)) = self.chars.next() {
            match ch {
                '(' => return Some(Ok((start_pos, Token::OpenRoundBracket, start_pos + 1))),
                ')' => return Some(Ok((start_pos, Token::CloseRoundBracket, start_pos + 1))),

                '[' => return Some(Ok((start_pos, Token::OpenSquareBracket, start_pos + 1))),
                ']' => return Some(Ok((start_pos, Token::CloseSquareBracket, start_pos + 1))),

                ';' => self.skip_comment(),

                ch if ch.is_whitespace() => {}

                '"' => return self.tokenize_string_literal(start_pos),

                _ => return self.tokenize_string(start_pos),
            }
        }

        None
    }

    fn skip_comment(&mut self) {
        const NEW_LINE: char = '\n'; // TODO: consider '\n\r'

        while let Some((_, ch)) = self.chars.next() {
            if ch == NEW_LINE {
                break;
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn tokenize_string_literal(
        &mut self,
        start_pos: usize,
    ) -> Option<Spanned<Token<'input>, usize, LexerError>> {
        while let Some((pos, ch)) = self.chars.next() {
            if ch == '"' {
                // + 1 to count an open double quote
                let string_size = pos - start_pos + 1;

                return Some(Ok((
                    start_pos,
                    Token::StringLiteral(&self.input[start_pos + 1..pos]),
                    start_pos + string_size,
                )));
            }
        }

        Some(Err(LexerError::UnclosedQuote(start_pos, self.input.len())))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn tokenize_string(
        &mut self,
        start_pos: usize,
    ) -> Option<Spanned<Token<'input>, usize, LexerError>> {
        let end_pos = self.advance_to_token_end(start_pos);

        // this slicing is safe here because borders come from the chars iterator
        let token_str = &self.input[start_pos..end_pos];

        let token = match string_to_token(token_str, start_pos) {
            Ok(token) => token,
            Err(e) => return Some(Err(e)),
        };

        let token_str_len = end_pos - start_pos;
        Some(Ok((start_pos, token, start_pos + token_str_len)))
    }

    fn advance_to_token_end(&mut self, start_pos: usize) -> usize {
        let mut end_pos = start_pos;
        let mut round_brackets_balance: i64 = 0;
        let mut square_brackets_balance: i64 = 0;

        while let Some((pos, ch)) = self.chars.peek() {
            end_pos = *pos;
            let ch = *ch;

            update_brackets_count(
                ch,
                &mut round_brackets_balance,
                &mut square_brackets_balance,
            );

            if should_stop(ch, round_brackets_balance, square_brackets_balance) {
                break;
            }

            self.chars.next();
        }

        self.advance_end_pos(&mut end_pos);
        end_pos
    }

    // if it was the last char, advance end position.
    fn advance_end_pos(&mut self, end_pos: &mut usize) {
        if self.chars.peek().is_none() {
            *end_pos += 1;
        }
    }
}

fn update_brackets_count(
    ch: char,
    round_brackets_balance: &mut i64,
    square_brackets_balance: &mut i64,
) {
    if ch == '(' {
        *round_brackets_balance += 1;
    } else if ch == ')' {
        *round_brackets_balance -= 1;
    } else if ch == '[' {
        *square_brackets_balance += 1;
    } else if ch == ']' {
        *square_brackets_balance -= 1;
    }
}

fn should_stop(ch: char, round_brackets_balance: i64, open_square_brackets_balance: i64) -> bool {
    ch.is_whitespace() || round_brackets_balance < 0 || open_square_brackets_balance < 0
}

fn string_to_token(input: &str, start_pos: usize) -> Result<Token, LexerError> {
    match input {
        "" => Err(LexerError::EmptyString(start_pos, start_pos)),

        CALL_INSTR => Ok(Token::Call),
        SEQ_INSTR => Ok(Token::Seq),
        PAR_INSTR => Ok(Token::Par),
        NULL_INSTR => Ok(Token::Null),
        FOLD_INSTR => Ok(Token::Fold),
        XOR_INSTR => Ok(Token::Xor),
        NEXT_INSTR => Ok(Token::Next),

        INIT_PEER_ID => Ok(Token::InitPeerId),

        str if str.ends_with(ACC_END_TAG) => try_parse_accumulator(str, start_pos),
        str => try_parse_call_variable(str, start_pos),
    }
}

fn try_parse_accumulator(maybe_acc: &str, start: usize) -> Result<Token, LexerError> {
    const ACC_END_TAG_SIZE: usize = 2;

    let str_len = maybe_acc.len();
    if str_len == ACC_END_TAG_SIZE {
        return Err(LexerError::EmptyAccName(start, start));
    }

    // this slice is safe here because str's been checked for ending with "[]"
    let maybe_acc = &maybe_acc[0..str_len - ACC_END_TAG_SIZE];

    for (pos, ch) in maybe_acc.chars().enumerate() {
        if !is_aqua_alphanumeric(ch) {
            return Err(LexerError::IsNotAlphanumeric(start + pos, start + pos));
        }
    }

    Ok(Token::Accumulator(maybe_acc))
}

fn try_parse_call_variable(maybe_var: &str, start: usize) -> Result<Token, LexerError> {
    let mut json_path_start_pos = None;

    for (pos, ch) in maybe_var.chars().enumerate() {
        if !json_path_started(json_path_start_pos) && is_json_path_start_point(ch) {
            json_path_start_pos = Some(pos);
        } else if !json_path_started(json_path_start_pos) && !is_aqua_alphanumeric(ch) {
            return Err(LexerError::IsNotAlphanumeric(start + pos, start + pos));
        } else if json_path_started(json_path_start_pos) & !json_path_allowed_char(ch) {
            return Err(LexerError::InvalidJsonPath(start + pos, start + pos));
        }
    }

    match json_path_start_pos {
        Some(pos) => Ok(Token::JsonPath(maybe_var, pos)),
        None => Ok(Token::Alphanumeric(maybe_var)),
    }
}

const CALL_INSTR: &str = "call";
const SEQ_INSTR: &str = "seq";
const PAR_INSTR: &str = "par";
const NULL_INSTR: &str = "null";
const FOLD_INSTR: &str = "fold";
const XOR_INSTR: &str = "xor";
const NEXT_INSTR: &str = "next";

const INIT_PEER_ID: &str = "%init_peer_id%";

const ACC_END_TAG: &str = "[]";

fn is_json_path_start_point(ch: char) -> bool {
    ch == '.'
}

fn json_path_started(first_dot_pos: Option<usize>) -> bool {
    first_dot_pos.is_some()
}

fn json_path_allowed_char(ch: char) -> bool {
    // we don't have spec for json path now, but some possible example could be found here
    // https://packagist.org/packages/softcreatr/jsonpath

    // good old switch faster here than hash set
    match ch {
        '$' => true,
        '@' => true,
        '[' => true,
        ']' => true,
        '(' => true,
        ')' => true,
        ':' => true,
        '?' => true,
        '.' => true,
        '*' => true,
        ',' => true,
        '"' => true,
        '\'' => true,
        '!' => true,
        ch => is_aqua_alphanumeric(ch),
    }
}

fn is_aqua_alphanumeric(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
}
