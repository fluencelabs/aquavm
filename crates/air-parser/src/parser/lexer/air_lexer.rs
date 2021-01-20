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
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(it) = self.chars.next() {
            match it {
                (i, '(') => return Some(Ok((i, Token::OpenRoundBracket, i + 1))),
                (i, ')') => return Some(Ok((i, Token::CloseRoundBracket, i + 1))),

                (i, '[') => return Some(Ok((i, Token::OpenSquareBracket, i + 1))),
                (i, ']') => return Some(Ok((i, Token::CloseSquareBracket, i + 1))),

                (i, '"') => return Some(Ok((i, Token::DoubleQuote, i + 1))),

                (_, ch) if ch.is_whitespace() => (),

                (start, _) => {
                    let mut end = start;

                    while let Some((i, ch)) = self.chars.next() {
                        end = i;

                        if is_term_char_for_supplement(ch) {
                            break;
                        }
                    }

                    // this slicing is safe here because borders come from the chars iterator
                    let token_str = &self.input[start..end];

                    let token = match try_to_token(token_str, start, end) {
                        Ok(token) => token,
                        Err(e) => return Some(Err(e)),
                    };

                    let token_str_len = end - start;
                    return Some(Ok((start, token, start + token_str_len)));
                }
            }
        }

        None
    }
}

fn is_term_char_for_supplement(ch: char) -> bool {
    ch.is_whitespace() || ch == '"' || ch == ')'
}

#[rustfmt::skip]
fn try_to_token(input: &str, start: usize, end: usize) -> Result<Token, LexicalError> {
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
            if str[0..str_len - ACC_END_TAG_SIZE].chars().all(char::is_alphanumeric) {
                return Ok(Token::Accumulator(&str[0..str_len - ACC_END_TAG_SIZE]));
            }

            Err(LexicalError::IsNotAlphanumeric(start, end))
        }

        str => {
            let mut json_path_start_pos = None;

            for (pos, ch) in str.chars().enumerate() {
                if !json_path_started(json_path_start_pos) && is_json_path_start_point(ch) {
                    json_path_start_pos = Some(pos);
                } else if !json_path_started(json_path_start_pos) && !char::is_alphanumeric(ch) {
                    return Err(LexicalError::IsNotAlphanumeric(start, end));
                } else if json_path_started(json_path_start_pos) & !json_path_allowed_char(ch) {
                    return Err(LexicalError::InvalidJsonPath(start, end));
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

const INIT_PEER_ID: &str = r#""init_peer_id""#;

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
        ch => ch.is_alphanumeric(),
    }
}
