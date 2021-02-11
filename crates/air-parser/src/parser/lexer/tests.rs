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
use super::LexerError;
use super::Number;
use super::Token;

fn run_lexer(input: &str) -> Vec<Spanned<Token<'_>, usize, LexerError>> {
    let lexer = AIRLexer::new(input);
    lexer.collect()
}

#[test]
fn air_instructions() {
    let call_tokens = run_lexer("call");
    assert_eq!(call_tokens, vec![Ok((0, Token::Call, 4))]);

    let call_tokens = run_lexer("(call)");
    assert_eq!(
        call_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Call, 5)),
            Ok((5, Token::CloseRoundBracket, 6))
        ]
    );

    let par_tokens = run_lexer("par");
    assert_eq!(par_tokens, vec![Ok((0, Token::Par, 3))]);

    let par_tokens = run_lexer("(par)");
    assert_eq!(
        par_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Par, 4)),
            Ok((4, Token::CloseRoundBracket, 5))
        ]
    );

    let seq_tokens = run_lexer("seq");
    assert_eq!(seq_tokens, vec![Ok((0, Token::Seq, 3))]);

    let seq_tokens = run_lexer("(seq)");
    assert_eq!(
        seq_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Seq, 4)),
            Ok((4, Token::CloseRoundBracket, 5))
        ]
    );

    let null_tokens = run_lexer("null");
    assert_eq!(null_tokens, vec![Ok((0, Token::Null, 4))]);

    let null_tokens = run_lexer("(null)");
    assert_eq!(
        null_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Null, 5)),
            Ok((5, Token::CloseRoundBracket, 6))
        ]
    );

    let fold_tokens = run_lexer("fold");
    assert_eq!(fold_tokens, vec![Ok((0, Token::Fold, 4))]);

    let fold_tokens = run_lexer("(fold)");
    assert_eq!(
        fold_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Fold, 5)),
            Ok((5, Token::CloseRoundBracket, 6))
        ]
    );

    let next_tokens = run_lexer("next");
    assert_eq!(next_tokens, vec![Ok((0, Token::Next, 4))]);

    let next_tokens = run_lexer("(next)");
    assert_eq!(
        next_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Next, 5)),
            Ok((5, Token::CloseRoundBracket, 6))
        ]
    );

    let match_tokens = run_lexer("match");
    assert_eq!(match_tokens, vec![Ok((0, Token::Match, 5))]);

    let match_tokens = run_lexer("(match)");
    assert_eq!(
        match_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::Match, 6)),
            Ok((6, Token::CloseRoundBracket, 7))
        ]
    );

    let mismatch_tokens = run_lexer("mismatch");
    assert_eq!(mismatch_tokens, vec![Ok((0, Token::MisMatch, 8))]);

    let mismatch_tokens = run_lexer("(mismatch)");
    assert_eq!(
        mismatch_tokens,
        vec![
            Ok((0, Token::OpenRoundBracket, 1)),
            Ok((1, Token::MisMatch, 9)),
            Ok((9, Token::CloseRoundBracket, 10))
        ]
    );
}

#[test]
fn init_peer_id() {
    const INIT_PEER_ID: &str = "%init_peer_id%";

    let init_peer_id_tokens = run_lexer(INIT_PEER_ID);
    assert_eq!(
        init_peer_id_tokens,
        vec![Ok((0, Token::InitPeerId, INIT_PEER_ID.len()))]
    );
}

#[test]
fn accumulator() {
    const ACC: &str = "accumulator____asdasd[]";

    let init_peer_id_tokens = run_lexer(ACC);
    assert_eq!(
        init_peer_id_tokens,
        vec![Ok((
            0,
            Token::Accumulator(&ACC[0..ACC.len() - 2]),
            ACC.len()
        ))]
    );
}

#[test]
fn string_literal() {
    const STRING_LITERAL: &str = r#""some_string""#;

    let string_literal_tokens = run_lexer(STRING_LITERAL);
    assert_eq!(
        string_literal_tokens,
        vec![Ok((
            0,
            Token::StringLiteral(&STRING_LITERAL[1..STRING_LITERAL.len() - 1]),
            STRING_LITERAL.len()
        ))]
    );
}

#[test]
fn integer_numbers() {
    const NUMBER_WITH_PLUS_SIGN: &str = "+123";

    let number_tokens = run_lexer(NUMBER_WITH_PLUS_SIGN);
    let number = Number::Int(123);

    assert_eq!(
        number_tokens,
        vec![Ok((0, Token::Number(number.clone()), NUMBER_WITH_PLUS_SIGN.len()))]
    );

    const NUMBER: &str = "123";

    let number_tokens = run_lexer(NUMBER);

    assert_eq!(
        number_tokens,
        vec![Ok((0, Token::Number(number), NUMBER.len()))]
    );

    const NUMBER_WITH_MINUS_SIGN: &str = "-123";

    let number_tokens = run_lexer(NUMBER_WITH_MINUS_SIGN);
    let number = Number::Int(-123);

    assert_eq!(
        number_tokens,
        vec![Ok((0, Token::Number(number), NUMBER_WITH_MINUS_SIGN.len()))]
    );
}

#[test]
fn positive_float_number() {
    const FNUMBER_WITH_PLUS_SIGN: &str = "+123.123";

    let number_tokens = run_lexer(FNUMBER_WITH_PLUS_SIGN);
    let number = Number::Float(123.123);

    assert_eq!(
        number_tokens,
        vec![Ok((0, Token::Number(number.clone()), FNUMBER_WITH_PLUS_SIGN.len()))]
    );

    const FNUMBER: &str = "123.123";

    let number_tokens = run_lexer(FNUMBER);

    assert_eq!(
        number_tokens,
        vec![Ok((0, Token::Number(number), FNUMBER.len()))]
    );

    const FNUMBER_WITH_MINUS_SIGN: &str = "-123.123";

    let number_tokens = run_lexer(FNUMBER_WITH_MINUS_SIGN);
    let number = Number::Float(-123.123);

    assert_eq!(
        number_tokens,
        vec![Ok((
            0,
            Token::Number(number),
            FNUMBER_WITH_MINUS_SIGN.len()
        ))]
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
    const FNUMBER: &str = "10000000000000000000000000000001.1231564564545684564646515313546547682131";

    let number_tokens = run_lexer(FNUMBER);

    println!("tokens are {:?}", number_tokens);

    assert!(matches!(
        number_tokens[0],
        Err(LexerError::ParseFloatError(..))
    ));
}

#[test]
fn json_path() {
    // this json path contains all allowed in json path charactes
    const JSON_PATH: &str = r#"value.$[$@[]():?.*,"!]"#;

    let json_path_tokens = run_lexer(JSON_PATH);
    assert_eq!(
        json_path_tokens,
        vec![Ok((0, Token::JsonPath(JSON_PATH, 5), JSON_PATH.len()))]
    );
}

#[test]
fn json_path_numbers() {
    // this json path contains all allowed in json path charactes
    const JSON_PATH: &str = r#"12345.$[$@[]():?.*,"!]"#;

    let json_path_tokens = run_lexer(JSON_PATH);
    assert_eq!(
        json_path_tokens,
        vec![Ok((0, Token::JsonPath(JSON_PATH, 5), JSON_PATH.len()))]
    );
}

#[test]
fn unclosed_quote() {
    const UNCLOSED_QUOTE_AIR: &str = r#"(call ("peer_name) ("service_name" "function_name") [])"#;

    let unclosed_quote_air_tokens = run_lexer(UNCLOSED_QUOTE_AIR);
    assert_eq!(
        unclosed_quote_air_tokens[4],
        Err(LexerError::IsNotAlphanumeric(33, 33))
    );
}

#[test]
fn bad_value() {
    // value contains ! that only allowed in json path
    const INVALID_VALUE: &str = r#"val!ue.$[$@[]():?.*,"\!]"#;

    let invalid_value_tokens = run_lexer(INVALID_VALUE);
    assert_eq!(
        invalid_value_tokens,
        vec![Err(LexerError::IsNotAlphanumeric(3, 3))]
    );
}

#[test]
fn invalid_json_path() {
    const INVALID_JSON_PATH: &str = r#"value.$%"#;

    let invalid_json_path_tokens = run_lexer(INVALID_JSON_PATH);
    assert_eq!(
        invalid_json_path_tokens,
        vec![Err(LexerError::InvalidJsonPath(7, 7))]
    );
}

#[test]
fn invalid_json_path_numbers() {
    // this json path contains all allowed in json path charactes
    const JSON_PATH: &str = r#"+12345$[$@[]():?.*,"!]"#;

    let json_path_tokens = run_lexer(JSON_PATH);
    assert_eq!(
        json_path_tokens,
        vec![Err(LexerError::IsNotAlphanumeric(6, 6))]
    );
}
