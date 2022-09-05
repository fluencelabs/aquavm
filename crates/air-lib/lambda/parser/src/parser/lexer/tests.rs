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

use super::accessors_lexer::Spanned;
use super::AccessorsLexer;
use super::LexerError;
use super::Token;

fn run_lexer(input: &str) -> Vec<Spanned<Token<'_>, usize, LexerError>> {
    let lexer = AccessorsLexer::new(input);
    lexer.collect()
}

#[test]
fn array_access() {
    let array_access: &str = ".[0]";

    let actual = run_lexer(array_access);
    let expected = vec![
        Spanned::Ok((0, Token::ValuePathSelector, 1)),
        Spanned::Ok((1, Token::OpenSquareBracket, 2)),
        Spanned::Ok((2, Token::NumberAccessor(0), 3)),
        Spanned::Ok((3, Token::CloseSquareBracket, 4)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn field_access() {
    let field_name = "some_field_name";
    let field_access = format!(".{}", field_name);

    let actual = run_lexer(&field_access);
    let expected = vec![
        Spanned::Ok((0, Token::ValuePathSelector, 1)),
        Spanned::Ok((1, Token::StringAccessor(field_name), 1 + field_name.len())),
    ];
    assert_eq!(actual, expected);
}
