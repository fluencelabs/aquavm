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

use super::lambda_ast_lexer::Spanned;
use super::LambdaASTLexer;
use super::LexerError;
use super::Token;

fn run_lexer(input: &str) -> Vec<Spanned<Token<'_>, usize, LexerError>> {
    let lexer = LambdaASTLexer::new(input);
    lexer.collect()
}

#[test]
fn array_access() {
    let array_access: &str = ".$.[0]";

    let actual = run_lexer(array_access);
    let expected = vec![
        Spanned::Ok((0, Token::ValuePathStarter, 2)),
        Spanned::Ok((2, Token::ValuePathSelector, 3)),
        Spanned::Ok((3, Token::OpenSquareBracket, 4)),
        Spanned::Ok((4, Token::NumberAccessor(0), 5)),
        Spanned::Ok((5, Token::CloseSquareBracket, 6)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn field_access() {
    let field_name = "some_field_name";
    let field_access = format!(".$.{field_name}");

    let actual = run_lexer(&field_access);
    let expected = vec![
        Spanned::Ok((0, Token::ValuePathStarter, 2)),
        Spanned::Ok((2, Token::ValuePathSelector, 3)),
        Spanned::Ok((3, Token::StringAccessor(field_name), 3 + field_name.len())),
    ];
    assert_eq!(actual, expected);
}
