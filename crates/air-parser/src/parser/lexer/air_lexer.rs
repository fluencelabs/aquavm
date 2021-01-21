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

use super::errors::LexicalError;
use super::token::Token;

use std::iter::Peekable;
use std::str::CharIndices;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<Spanned<Token<'input>, usize, LexicalError>> {
        while let Some(it) = self.chars.next() {
            match it {
                (i, '(') => return Some(Ok((i, Token::OpenRoundBracket, i + 1))),
                (i, ')') => return Some(Ok((i, Token::CloseRoundBracket, i + 1))),

                (i, '[') => return Some(Ok((i, Token::OpenSquareBracket, i + 1))),
                (i, ']') => return Some(Ok((i, Token::CloseSquareBracket, i + 1))),

                (_, ';') => {
                    while let Some((_, ch)) = self.chars.next() {
                        if ch == '\n' {
                            break;
                        }
                    }
                }

                (start, '"') => {
                    while let Some((pos, ch)) = self.chars.next() {
                        if ch == '"' {
                            let string_size = pos - start;
                            return Some(Ok((
                                start,
                                Token::StringLiteral(&self.input[start + 1..pos]),
                                pos + string_size,
                            )));
                        }
                    }

                    return Some(Err(LexicalError::UnclosedQuote(start, self.input.len())));
                }

                (_, ch) if ch.is_whitespace() => (),

                (start, _) => {
                    let mut end = start;
                    let mut round_brackets_balance: i64 = 0;
                    let mut square_brackets_balance: i64 = 0;

                    while let Some((i, ch)) = self.chars.peek() {
                        end = *i;
                        let ch = *ch;
                        if ch == '(' {
                            round_brackets_balance += 1;
                        } else if ch == ')' {
                            round_brackets_balance -= 1;
                        } else if ch == '[' {
                            square_brackets_balance += 1;
                        } else if ch == ']' {
                            square_brackets_balance -= 1;
                        }

                        if should_stop(ch, round_brackets_balance, square_brackets_balance) {
                            break;
                        }
                        self.chars.next();
                    }

                    // this slicing is safe here because borders come from the chars iterator
                    let token_str = &self.input[start..end];

                    let token = match try_to_token(token_str, start, end) {
                        Ok(token) => token,
                        Err(e) => return Some(Err(e)),
                    };

                    let mut token_str_len = end - start;
                    if round_brackets_balance < 0 || square_brackets_balance < 0 {
                        token_str_len -= 1;
                    }

                    return Some(Ok((start, token, start + token_str_len)));
                }
            }
        }

        None
    }
}

fn should_stop(ch: char, round_brackets_balance: i64, open_square_brackets_balance: i64) -> bool {
    ch.is_whitespace() || round_brackets_balance < 0 || open_square_brackets_balance < 0
}

#[rustfmt::skip]
fn try_to_token(input: &str, start: usize, end: usize) -> Result<Token, LexicalError> {
    println!("input: {}", input);

    match input {
        "" => Err(LexicalError::EmptyString(start, end)),

        CALL_INSTR => Ok(Token::Call),
        SEQ_INSTR => Ok(Token::Seq),
        PAR_INSTR => Ok(Token::Par),
        NULL_INSTR => Ok(Token::Null),
        FOLD_INSTR => Ok(Token::Fold),
        XOR_INSTR => Ok(Token::Xor),
        NEXT_INSTR => Ok(Token::Next),

        INIT_PEER_ID => Ok(Token::InitPeerId),

        str if str.ends_with(ACC_END_TAG) => {
            const ACC_END_TAG_SIZE: usize = 2;

            let str_len = str.len();
            if str_len == ACC_END_TAG_SIZE {
                return Err(LexicalError::EmptyAccName(start, end));
            }

            // this slice is safe here because str's been checked for ending with "[]"
            if str[0..str_len - ACC_END_TAG_SIZE].chars().all(is_aqua_alphanumeric) {
                return Ok(Token::Accumulator(&str[0..str_len - ACC_END_TAG_SIZE]));
            }

            Err(LexicalError::IsNotAlphanumeric(start, end))
        }

        str => {
            let mut json_path_start_pos = None;

            for (pos, ch) in str.chars().enumerate() {
                if !json_path_started(json_path_start_pos) && is_json_path_start_point(ch) {
                    json_path_start_pos = Some(pos);
                } else if !json_path_started(json_path_start_pos) && !is_aqua_alphanumeric(ch) {
                    return Err(LexicalError::IsNotAlphanumeric(start, end));
                } else if json_path_started(json_path_start_pos) & !json_path_allowed_char(ch) {
                    return Err(LexicalError::InvalidJsonPath(start+pos, start+pos));
                }
            }

            match json_path_start_pos {
                Some(pos) => Ok(Token::JsonPath(str, pos)),
                None => Ok(Token::Alphanumeric(str)),
            }
        }
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
        '-' => true,
        ',' => true,
        '"' => true,
        '\'' => true,
        '!' => true,
        ch => is_aqua_alphanumeric(ch),
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

fn is_aqua_alphanumeric(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
}
