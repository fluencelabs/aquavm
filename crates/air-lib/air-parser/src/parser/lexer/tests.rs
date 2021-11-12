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

use super::air_lexer::Spanned;
use super::AIRLexer;
use super::LastErrorPath;
use super::LexerError;
use super::Number;
use super::Token;

use air_lambda_parser::{LambdaAST, ValueAccessor};

fn run_lexer(input: &str) -> Vec<Spanned<Token<'_>, usize, LexerError>> {
    let lexer = AIRLexer::new(input);
    lexer.collect()
}

#[allow(dead_code)]
enum TokenCompareStrategy<'token> {
    All(Vec<Spanned<Token<'token>, usize, LexerError>>),
    Some(Vec<usize>, Vec<Spanned<Token<'token>, usize, LexerError>>),
    One(usize, Spanned<Token<'token>, usize, LexerError>),
    Single(Spanned<Token<'token>, usize, LexerError>),
}

use TokenCompareStrategy::*;

fn lexer_test(input: &str, expected_tokens: TokenCompareStrategy) {
    let actual_tokens = run_lexer(input);

    match expected_tokens {
        All(expected_tokens) => assert_eq!(actual_tokens, expected_tokens),
        Some(token_ids, expected_tokens) => {
            for (&id, token) in token_ids.iter().zip(expected_tokens) {
                assert_eq!(actual_tokens[id], token);
            }
        }
        One(id, token) => assert_eq!(actual_tokens[id], token),
        Single(token) => assert_eq!(actual_tokens, vec![token]),
    }
}

#[test]
fn air_instructions() {
    lexer_test("call", Single(Ok((0, Token::Call, 4))));

    lexer_test(
        "(call)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Call, 5)),
            Ok((5, Token::CloseRoundBracket, 6)),
        ]),
    );

    lexer_test("par", Single(Ok((0, Token::Par, 3))));

    lexer_test(
        "(par)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Par, 4)),
            Ok((4, Token::CloseRoundBracket, 5)),
        ]),
    );

    lexer_test("seq", Single(Ok((0, Token::Seq, 3))));

    lexer_test(
        "(seq)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Seq, 4)),
            Ok((4, Token::CloseRoundBracket, 5)),
        ]),
    );

    lexer_test("null", Single(Ok((0, Token::Null, 4))));

    lexer_test(
        "(null)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Null, 5)),
            Ok((5, Token::CloseRoundBracket, 6)),
        ]),
    );

    lexer_test("fold", Single(Ok((0, Token::Fold, 4))));

    lexer_test(
        "(fold)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Fold, 5)),
            Ok((5, Token::CloseRoundBracket, 6)),
        ]),
    );

    lexer_test("next", Single(Ok((0, Token::Next, 4))));

    lexer_test(
        "(next)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Next, 5)),
            Ok((5, Token::CloseRoundBracket, 6)),
        ]),
    );

    lexer_test("match", Single(Ok((0, Token::Match, 5))));

    lexer_test(
        "(match)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Match, 6)),
            Ok((6, Token::CloseRoundBracket, 7)),
        ]),
    );

    lexer_test("mismatch", Single(Ok((0, Token::MisMatch, 8))));

    lexer_test(
        "(mismatch)",
        All(vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::MisMatch, 9)),
            Ok((9, Token::CloseRoundBracket, 10)),
        ]),
    );
}

#[test]
fn init_peer_id() {
    const INIT_PEER_ID: &str = "%init_peer_id%";

    lexer_test(
        INIT_PEER_ID,
        Single(Ok((0, Token::InitPeerId, INIT_PEER_ID.len()))),
    );
}

#[test]
fn stream() {
    const STREAM: &str = "$stream____asdasd";

    lexer_test(
        STREAM,
        Single(Ok((
            0,
            Token::Stream {
                name: STREAM,
                position: 0,
            },
            STREAM.len(),
        ))),
    );
}

#[test]
fn string_literal() {
    const STRING_LITERAL: &str = r#""some_string""#;

    lexer_test(
        STRING_LITERAL,
        Single(Ok((
            0,
            Token::StringLiteral(&STRING_LITERAL[1..STRING_LITERAL.len() - 1]),
            STRING_LITERAL.len(),
        ))),
    );
}

#[test]
fn integer_numbers() {
    const NUMBER_WITH_PLUS_SIGN: &str = "+123";
    let number = Number::Int(123);

    lexer_test(
        NUMBER_WITH_PLUS_SIGN,
        Single(Ok((
            0,
            Token::Number(number.clone()),
            NUMBER_WITH_PLUS_SIGN.len(),
        ))),
    );

    const NUMBER: &str = "123";

    lexer_test(
        NUMBER,
        Single(Ok((0, Token::Number(number.clone()), NUMBER.len()))),
    );

    const NUMBER_WITH_MINUS_SIGN: &str = "-123";
    let number = Number::Int(-123);

    lexer_test(
        NUMBER_WITH_MINUS_SIGN,
        Single(Ok((0, Token::Number(number), NUMBER_WITH_MINUS_SIGN.len()))),
    );
}

#[test]
fn float_number() {
    const FNUMBER_WITH_PLUS_SIGN: &str = "+123.123";
    let number = Number::Float(123.123);

    lexer_test(
        FNUMBER_WITH_PLUS_SIGN,
        Single(Ok((
            0,
            Token::Number(number.clone()),
            FNUMBER_WITH_PLUS_SIGN.len(),
        ))),
    );

    const FNUMBER: &str = "123.123";

    lexer_test(
        FNUMBER,
        Single(Ok((0, Token::Number(number), FNUMBER.len()))),
    );

    const FNUMBER_WITH_MINUS_SIGN: &str = "-123.123";
    let number = Number::Float(-123.123);

    lexer_test(
        FNUMBER_WITH_MINUS_SIGN,
        Single(Ok((
            0,
            Token::Number(number),
            FNUMBER_WITH_MINUS_SIGN.len(),
        ))),
    );
}

#[test]
fn too_big_number() {
    const NUMBER: &str = "1231231564564545684564646515313546547682131";

    let number_tokens = run_lexer(NUMBER);

    assert!(matches!(
        number_tokens[0],
        Err(LexerError::ParseIntError(..))
    ));
}

#[test]
fn too_big_float_number() {
    const FNUMBER: &str =
        "10000000000000000000000000000001.1231564564545684564646515313546547682131";

    lexer_test(
        FNUMBER,
        Single(Err(LexerError::TooBigFloat(0, FNUMBER.len()))),
    );
}

#[test]
fn lambda() {
    // this lambda contains all allowed in lambda characters
    const LAMBDA: &str = r#"value.$.field[1]"#;

    lexer_test(
        LAMBDA,
        Single(Ok((
            0,
            Token::ScalarWithLambda {
                name: "value",
                lambda: unsafe {
                    LambdaAST::new_unchecked(vec![
                        ValueAccessor::FieldAccess {
                            field_name: "field",
                        },
                        ValueAccessor::ArrayAccess { idx: 1 },
                    ])
                },
            },
            LAMBDA.len(),
        ))),
    );
}

#[test]
fn lambda_path_numbers() {
    const LAMBDA: &str = r#"12345.$[$@[]():?.*,"]"#;

    lexer_test(LAMBDA, Single(Err(LexerError::UnallowedCharInNumber(6, 6))));

    const LAMBDA1: &str = r#"+12345.$[$@[]():?.*,"]"#;

    lexer_test(
        LAMBDA1,
        Single(Err(LexerError::UnallowedCharInNumber(7, 7))),
    );
}

#[test]
fn leading_dot() {
    const LEADING_DOT: &str = ".111";

    lexer_test(LEADING_DOT, Single(Err(LexerError::LeadingDot(0, 0))));

    const LEADING_DOT_AFTER_SIGN: &str = "+.1111";

    lexer_test(
        LEADING_DOT_AFTER_SIGN,
        Single(Err(LexerError::LeadingDot(1, 1))),
    );
}

#[test]
fn unclosed_quote() {
    const UNCLOSED_QUOTE_AIR: &str = r#"(call ("peer_name) ("service_name" "function_name") [])"#;

    lexer_test(
        UNCLOSED_QUOTE_AIR,
        One(4, Err(LexerError::IsNotAlphanumeric(33, 33))),
    );
}

#[test]
fn bad_value() {
    // value contains ! that only allowed at the end of a lambda expression
    const INVALID_VALUE: &str = r#"val!ue.$[$@[]():?.*,"\]"#;

    lexer_test(
        INVALID_VALUE,
        Single(Err(LexerError::IsNotAlphanumeric(3, 3))),
    );

    // value contains ! that only allowed at the end of a lambda expression
    const INVALID_VALUE2: &str = r#"value.$![$@[]():?.*,"\]"#;

    lexer_test(INVALID_VALUE2, Single(Err(LexerError::InvalidLambda(7, 7))));
}

#[test]
fn invalid_lambda() {
    const INVALID_LAMBDA: &str = r#"value.$%"#;

    lexer_test(INVALID_LAMBDA, Single(Err(LexerError::InvalidLambda(7, 7))));
}

#[test]
fn invalid_lambda_numbers() {
    // this lambda contains all allowed in lambda characters
    const LAMBDA: &str = r#"+12345$[$@[]():?.*,"!]"#;

    lexer_test(LAMBDA, Single(Err(LexerError::IsNotAlphanumeric(6, 6))));
}

#[test]
fn last_error() {
    const LAST_ERROR: &str = r#"%last_error%"#;

    lexer_test(
        LAST_ERROR,
        Single(Ok((
            0,
            Token::LastError(LastErrorPath::None),
            LAST_ERROR.len(),
        ))),
    );
}

#[test]
fn last_error_instruction() {
    const LAST_ERROR: &str = r#"%last_error%.$.instruction"#;

    lexer_test(
        LAST_ERROR,
        Single(Ok((
            0,
            Token::LastError(LastErrorPath::Instruction),
            LAST_ERROR.len(),
        ))),
    );
}

#[test]
fn last_error_msg() {
    const LAST_ERROR: &str = r#"%last_error%.$.msg"#;

    lexer_test(
        LAST_ERROR,
        Single(Ok((
            0,
            Token::LastError(LastErrorPath::Message),
            LAST_ERROR.len(),
        ))),
    );
}

#[test]
fn last_error_peer_id() {
    const LAST_ERROR: &str = r#"%last_error%.$.peer_id"#;

    lexer_test(
        LAST_ERROR,
        Single(Ok((
            0,
            Token::LastError(LastErrorPath::PeerId),
            LAST_ERROR.len(),
        ))),
    );
}

#[test]
fn last_error_incorrect_field() {
    const LAST_ERROR: &str = r#"%last_error%.$.asdasd"#;

    lexer_test(
        LAST_ERROR,
        Single(Err(LexerError::LastErrorPathError(
            12,
            LAST_ERROR.len(),
            ".$.asdasd".to_string(),
        ))),
    );
}

#[test]
fn booleans() {
    const TRUE_BOOL_CONST: &str = "true";

    lexer_test(
        TRUE_BOOL_CONST,
        Single(Ok((0, Token::Boolean(true), TRUE_BOOL_CONST.len()))),
    );

    const FALSE_BOOL_CONST: &str = "false";

    lexer_test(
        FALSE_BOOL_CONST,
        Single(Ok((0, Token::Boolean(false), FALSE_BOOL_CONST.len()))),
    );

    const NON_BOOL_CONST: &str = "true1";

    lexer_test(
        NON_BOOL_CONST,
        Single(Ok((
            0,
            Token::Scalar {
                name: NON_BOOL_CONST,
            },
            NON_BOOL_CONST.len(),
        ))),
    );
}
