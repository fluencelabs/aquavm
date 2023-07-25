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

use crate::AirPos;

use super::air_lexer::Spanned;
use super::AIRLexer;
use super::LexerError;
use super::Token;

use air_lambda_parser::LambdaAST;
use air_lambda_parser::ValueAccessor;

use air_lambda_ast::Functor;

fn run_lexer(input: &str) -> Vec<Spanned<Token<'_>, AirPos, LexerError>> {
    let lexer = AIRLexer::new(input);
    lexer.collect()
}

#[allow(dead_code)]
enum TokenCompareStrategy<'token> {
    All(Vec<Spanned<Token<'token>, AirPos, LexerError>>),
    Some(Vec<usize>, Vec<Spanned<Token<'token>, AirPos, LexerError>>),
    One(usize, Spanned<Token<'token>, AirPos, LexerError>),
    Single(Spanned<Token<'token>, AirPos, LexerError>),
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
    lexer_test("call", Single(Ok((0.into(), Token::Call, 4.into()))));

    lexer_test(
        "(call)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Call, 5.into())),
            Ok((5.into(), Token::CloseRoundBracket, 6.into())),
        ]),
    );

    lexer_test("par", Single(Ok((0.into(), Token::Par, 3.into()))));

    lexer_test(
        "(par)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Par, 4.into())),
            Ok((4.into(), Token::CloseRoundBracket, 5.into())),
        ]),
    );

    lexer_test("seq", Single(Ok((0.into(), Token::Seq, 3.into()))));

    lexer_test(
        "(seq)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Seq, 4.into())),
            Ok((4.into(), Token::CloseRoundBracket, 5.into())),
        ]),
    );

    lexer_test("null", Single(Ok((0.into(), Token::Null, 4.into()))));

    lexer_test(
        "(null)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Null, 5.into())),
            Ok((5.into(), Token::CloseRoundBracket, 6.into())),
        ]),
    );

    lexer_test("fail", Single(Ok((0.into(), Token::Fail, 4.into()))));

    lexer_test("fold", Single(Ok((0.into(), Token::Fold, 4.into()))));

    lexer_test(
        "(fold)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Fold, 5.into())),
            Ok((5.into(), Token::CloseRoundBracket, 6.into())),
        ]),
    );

    lexer_test("next", Single(Ok((0.into(), Token::Next, 4.into()))));

    lexer_test(
        "(next)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Next, 5.into())),
            Ok((5.into(), Token::CloseRoundBracket, 6.into())),
        ]),
    );

    lexer_test("match", Single(Ok((0.into(), Token::Match, 5.into()))));

    lexer_test(
        "(match)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::Match, 6.into())),
            Ok((6.into(), Token::CloseRoundBracket, 7.into())),
        ]),
    );

    lexer_test(
        "mismatch",
        Single(Ok((0.into(), Token::MisMatch, 8.into()))),
    );

    lexer_test(
        "(mismatch)",
        All(vec![
            Ok((0.into(), Token::OpenRoundBracket, 1.into())),
            Ok((1.into(), Token::MisMatch, 9.into())),
            Ok((9.into(), Token::CloseRoundBracket, 10.into())),
        ]),
    );
}

#[test]
fn init_peer_id() {
    const INIT_PEER_ID: &str = "%init_peer_id%";

    lexer_test(
        INIT_PEER_ID,
        Single(Ok((0.into(), Token::InitPeerId, INIT_PEER_ID.len().into()))),
    );
}

#[test]
fn timestamp() {
    const TIMESTAMP: &str = "%timestamp%";

    lexer_test(
        TIMESTAMP,
        Single(Ok((0.into(), Token::Timestamp, TIMESTAMP.len().into()))),
    );
}

#[test]
fn ttl() {
    const TTL: &str = "%ttl%";

    lexer_test(TTL, Single(Ok((0.into(), Token::TTL, TTL.len().into()))));
}

#[test]
fn stream() {
    const STREAM: &str = "$stream____asdasd";

    lexer_test(
        STREAM,
        Single(Ok((
            0.into(),
            Token::Stream {
                name: STREAM,
                position: 0.into(),
            },
            STREAM.len().into(),
        ))),
    );
}

#[test]
fn stream_map() {
    const STREAM_MAP: &str = "%stream_map____asdasd";

    lexer_test(
        STREAM_MAP,
        Single(Ok((
            0.into(),
            Token::StreamMap {
                name: STREAM_MAP,
                position: 0.into(),
            },
            STREAM_MAP.len().into(),
        ))),
    );
}

#[test]
fn canon_stream() {
    for canon_stream_name in vec!["#stream____asdasd", "#$stream____asdasd"] {
        lexer_test(
            canon_stream_name,
            Single(Ok((
                0.into(),
                Token::CanonStream {
                    name: canon_stream_name,
                    position: 0.into(),
                },
                canon_stream_name.len().into(),
            ))),
        );
    }

    let cannon_stream_name = "#s$stream____asdasd";
    lexer_test(
        cannon_stream_name,
        Single(Err(LexerError::is_not_alphanumeric(2.into()..2.into()))),
    );

    let cannon_stream_name = "#";
    lexer_test(
        cannon_stream_name,
        Single(Err(LexerError::empty_tagged_name(0.into()..0.into()))),
    );
}

#[test]
fn canon_stream_with_functor() {
    for canon_stream_name in vec!["#canon_stream", "#$canon_stream"] {
        let canon_stream_with_functor: String = format!("{canon_stream_name}.length");

        lexer_test(
            &canon_stream_with_functor,
            Single(Ok((
                0.into(),
                Token::CanonStreamWithLambda {
                    name: canon_stream_name,
                    lambda: LambdaAST::Functor(Functor::Length),
                    position: 0.into(),
                },
                canon_stream_with_functor.len().into(),
            ))),
        );
    }

    let cannon_stream_name = "#s$stream____asdasd.length";
    lexer_test(
        cannon_stream_name,
        Single(Err(LexerError::is_not_alphanumeric(2.into()..2.into()))),
    );
    let cannon_stream_name = "#.length";
    lexer_test(
        cannon_stream_name,
        Single(Err(LexerError::empty_canon_name(0.into()..0.into()))),
    );
    let cannon_stream_name = "#$.length";
    lexer_test(
        cannon_stream_name,
        Single(Err(LexerError::empty_canon_name(1.into()..1.into()))),
    );
}

#[test]
fn scalar_with_functor() {
    let scalar_name = "scalar";
    let scalar_with_functor: String = format!("{scalar_name}.length");

    lexer_test(
        &scalar_with_functor,
        Single(Ok((
            0.into(),
            Token::ScalarWithLambda {
                name: scalar_name,
                lambda: LambdaAST::Functor(Functor::Length),
                position: 0.into(),
            },
            scalar_with_functor.len().into(),
        ))),
    );
}

#[test]
fn string_literal() {
    const STRING_LITERAL: &str = r#""some_string""#;

    lexer_test(
        STRING_LITERAL,
        Single(Ok((
            0.into(),
            Token::StringLiteral(&STRING_LITERAL[1..STRING_LITERAL.len() - 1]),
            STRING_LITERAL.len().into(),
        ))),
    );
}

#[test]
fn integer_numbers() {
    let test_integer = 123;
    let number_with_plus_sign = format!("+{test_integer}");

    lexer_test(
        &number_with_plus_sign,
        Single(Ok((
            0.into(),
            Token::I64(test_integer),
            number_with_plus_sign.len().into(),
        ))),
    );

    let number = format!("{test_integer}");

    lexer_test(
        &number,
        Single(Ok((
            0.into(),
            Token::I64(test_integer),
            number.len().into(),
        ))),
    );

    let number_with_minus_sign = format!("-{test_integer}");

    lexer_test(
        &number_with_minus_sign,
        Single(Ok((
            0.into(),
            Token::I64(-test_integer),
            number_with_minus_sign.len().into(),
        ))),
    );
}

#[test]
fn float_number() {
    let test_float = 123.123;
    let float_number_with_plus_sign = format!("+{test_float}");

    lexer_test(
        &float_number_with_plus_sign,
        Single(Ok((
            0.into(),
            Token::F64(test_float),
            float_number_with_plus_sign.len().into(),
        ))),
    );

    let float_number = format!("{test_float}");

    lexer_test(
        &float_number,
        Single(Ok((
            0.into(),
            Token::F64(test_float),
            float_number.len().into(),
        ))),
    );

    let float_number_with_minus_sign = format!("-{test_float}");

    lexer_test(
        &float_number_with_minus_sign,
        Single(Ok((
            0.into(),
            Token::F64(-test_float),
            float_number_with_minus_sign.len().into(),
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
        Single(Err(LexerError::too_big_float(
            0.into()..FNUMBER.len().into(),
        ))),
    );
}

#[test]
fn lambda() {
    // this lambda contains all allowed in lambda characters
    const LAMBDA: &str = r#"value.$.field[1]"#;

    lexer_test(
        LAMBDA,
        Single(Ok((
            0.into(),
            Token::ScalarWithLambda {
                name: "value",
                lambda: LambdaAST::try_from_accessors(vec![
                    ValueAccessor::FieldAccessByName {
                        field_name: "field",
                    },
                    ValueAccessor::ArrayAccess { idx: 1 },
                ])
                .unwrap(),
                position: 0.into(),
            },
            LAMBDA.len().into(),
        ))),
    );
}

#[test]
fn lambda_path_numbers() {
    const LAMBDA: &str = r#"12345.$[$@[]():?.*,"]"#;

    lexer_test(
        LAMBDA,
        Single(Err(LexerError::unallowed_char_in_number(
            6.into()..6.into(),
        ))),
    );

    const LAMBDA1: &str = r#"+12345.$[$@[]():?.*,"]"#;

    lexer_test(
        LAMBDA1,
        Single(Err(LexerError::unallowed_char_in_number(
            7.into()..7.into(),
        ))),
    );
}

#[test]
fn leading_dot() {
    const LEADING_DOT: &str = ".111";

    lexer_test(
        LEADING_DOT,
        Single(Err(LexerError::leading_dot(0.into()..0.into()))),
    );

    const LEADING_DOT_AFTER_SIGN: &str = "+.1111";

    lexer_test(
        LEADING_DOT_AFTER_SIGN,
        Single(Err(LexerError::leading_dot(1.into()..1.into()))),
    );
}

#[test]
fn unclosed_quote() {
    const UNCLOSED_QUOTE_AIR: &str = r#"(call ("peer_name) ("service_name" "function_name") [])"#;

    lexer_test(
        UNCLOSED_QUOTE_AIR,
        One(
            4,
            Err(LexerError::is_not_alphanumeric(33.into()..33.into())),
        ),
    );
}

#[test]
fn bad_value() {
    // value contains ! that only allowed at the end of a lambda expression
    const INVALID_VALUE: &str = r#"val!ue.$[$@[]():?.*,"\]"#;

    lexer_test(
        INVALID_VALUE,
        Single(Err(LexerError::is_not_alphanumeric(3.into()..3.into()))),
    );

    // value contains ! that only allowed at the end of a lambda expression
    const INVALID_VALUE2: &str = r#"value.$![$@[]():?.*,"\]"#;

    lexer_test(
        INVALID_VALUE2,
        Single(Err(LexerError::invalid_lambda(7.into()..7.into()))),
    );
}

#[test]
fn invalid_lambda() {
    const INVALID_LAMBDA: &str = r#"value.$%"#;

    lexer_test(
        INVALID_LAMBDA,
        Single(Err(LexerError::invalid_lambda(7.into()..7.into()))),
    );
}

#[test]
fn invalid_lambda_numbers() {
    // this lambda contains all allowed in lambda characters
    const LAMBDA: &str = r#"+12345$[$@[]():?.*,"!]"#;

    lexer_test(
        LAMBDA,
        Single(Err(LexerError::is_not_alphanumeric(6.into()..6.into()))),
    );
}

#[test]
fn last_error() {
    const LAST_ERROR: &str = r#"%last_error%"#;

    lexer_test(
        LAST_ERROR,
        Single(Ok((0.into(), Token::LastError, LAST_ERROR.len().into()))),
    );
}

#[test]
fn last_error_instruction() {
    const LAST_ERROR: &str = r#"%last_error%.$.instruction"#;

    let token = Token::LastErrorWithLambda(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "instruction",
        }])
        .unwrap(),
    );

    lexer_test(
        LAST_ERROR,
        Single(Ok((0.into(), token, LAST_ERROR.len().into()))),
    );
}

#[test]
fn last_error_message() {
    const LAST_ERROR: &str = r#"%last_error%.$.message"#;

    let token = Token::LastErrorWithLambda(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "message",
        }])
        .unwrap(),
    );
    lexer_test(
        LAST_ERROR,
        Single(Ok((0.into(), token, LAST_ERROR.len().into()))),
    );
}

#[test]
fn last_error_peer_id() {
    const LAST_ERROR: &str = r#"%last_error%.$.peer_id"#;

    let token = Token::LastErrorWithLambda(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "peer_id",
        }])
        .unwrap(),
    );
    lexer_test(
        LAST_ERROR,
        Single(Ok((0.into(), token, LAST_ERROR.len().into()))),
    );
}

#[test]
fn last_error_non_standard_field() {
    const LAST_ERROR: &str = r#"%last_error%.$.asdasd"#;

    let token = Token::LastErrorWithLambda(
        LambdaAST::try_from_accessors(vec![ValueAccessor::FieldAccessByName {
            field_name: "asdasd",
        }])
        .unwrap(),
    );
    lexer_test(
        LAST_ERROR,
        Single(Ok((0.into(), token, LAST_ERROR.len().into()))),
    );
}

#[test]
fn booleans() {
    const TRUE_BOOL_CONST: &str = "true";

    lexer_test(
        TRUE_BOOL_CONST,
        Single(Ok((
            0.into(),
            Token::Boolean(true),
            TRUE_BOOL_CONST.len().into(),
        ))),
    );

    const FALSE_BOOL_CONST: &str = "false";

    lexer_test(
        FALSE_BOOL_CONST,
        Single(Ok((
            0.into(),
            Token::Boolean(false),
            FALSE_BOOL_CONST.len().into(),
        ))),
    );

    const NON_BOOL_CONST: &str = "true1";

    lexer_test(
        NON_BOOL_CONST,
        Single(Ok((
            0.into(),
            Token::Scalar {
                name: NON_BOOL_CONST,
                position: 0.into(),
            },
            NON_BOOL_CONST.len().into(),
        ))),
    );
}

#[test]
fn match_with_empty_array__() {
    const MATCH_WITH_EMPTY_ARRAY: &str = "(match scalar []
        (null)
    )";

    lexer_test(
        MATCH_WITH_EMPTY_ARRAY,
        Some(
            vec![3, 4],
            vec![
                Ok((14.into(), Token::OpenSquareBracket, 15.into())),
                Ok((15.into(), Token::CloseSquareBracket, 16.into())),
            ],
        ),
    );
}

// #[test]
// fn stream_map_index() {
//     lexer_test(
//         r#"#%canon["key"]"#,
//         All(vec![
//             Ok((
//                 0.into(),
//                 Token::CanonStreamMap {
//                     name: "#%canon",
//                     position: 0.into(),
//                 },
//                 7.into(),
//             )),
//             Ok((7.into(), Token::OpenSquareBracket, 8.into())),
//             Ok((8.into(), Token::StringLiteral("key"), 13.into())),
//             Ok((13.into(), Token::CloseSquareBracket, 14.into())),
//         ]),
//     );

//     lexer_test(
//         "#%canon[42]",
//         All(vec![
//             Ok((
//                 0.into(),
//                 Token::CanonStreamMap {
//                     name: "#%canon",
//                     position: 0.into(),
//                 },
//                 7.into(),
//             )),
//             Ok((7.into(), Token::OpenSquareBracket, 8.into())),
//             Ok((8.into(), Token::I64(42), 10.into())),
//             Ok((10.into(), Token::CloseSquareBracket, 11.into())),
//         ]),
//     );
// }
