/*
 * Copyright 2022 Fluence Labs Limited
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

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace0;
use nom::character::complete::{alphanumeric1, multispace1, one_of};
use nom::combinator::{cut, map, opt, recognize, value};
use nom::multi::{many1_count, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::{error::VerboseError, IResult};
use std::str::FromStr;

use crate::asserts::parser::delim_ws;

type ParseError<'inp> = VerboseError<&'inp str>;
type Triplet = (Sexp, Sexp, Sexp);

#[derive(Debug, PartialEq)]
enum Sexp {
    Call {
        triplet: Box<Triplet>,
        args: Vec<Sexp>,
        var: Option<Box<Sexp>>,
        annotation: Option<String>,
    },
    List(Vec<Sexp>),
    Symbol(String),
    String(String),
}

impl FromStr for Sexp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::combinator::all_consuming;
        all_consuming(delim_ws(parse_sexp))(s)
            .map(|(_, v)| v)
            .map_err(|e| e.to_string())
    }
}

fn parse_sexp(inp: &str) -> IResult<&str, Sexp, ParseError> {
    alt((parse_sexp_list_like, parse_sexp_string, parse_sexp_symbol))(inp)
}

fn parse_sexp_list_like(inp: &str) -> IResult<&str, Sexp, ParseError> {
    alt((parse_sexp_call, parse_sexp_list))(inp)
}

fn parse_sexp_list(inp: &str) -> IResult<&str, Sexp, ParseError> {
    delimited(
        terminated(tag("("), multispace0),
        cut(map(separated_list0(multispace1, parse_sexp), Sexp::List)),
        preceded(multispace0, tag(")")),
    )(inp)
}

fn parse_sexp_string(inp: &str) -> IResult<&str, Sexp, ParseError> {
    // N.B. escape are rejected by AIR parser, but we simply treat backslash
    // as any other character
    map(
        delimited(tag("\""), cut(is_not("\"")), tag("\"")),
        |s: &str| Sexp::String(s.to_owned()),
    )(inp)
}

fn parse_sexp_symbol(inp: &str) -> IResult<&str, Sexp, ParseError> {
    map(
        recognize(many1_count(alt((
            value((), alphanumeric1),
            value((), one_of("_.$")),
        )))),
        |s: &str| Sexp::Symbol(s.to_owned()),
    )(inp)
}

fn parse_sexp_call(inp: &str) -> IResult<&str, Sexp, ParseError> {
    preceded(
        delim_ws(tag("(")),
        preceded(tag("call "), cut(parse_sexp_call_content)),
        // call content includes ")" and possible comment
    )(inp)
}

fn parse_sexp_call_content(inp: &str) -> IResult<&str, Sexp, ParseError> {
    map(
        pair(
            // triplet and arguments
            pair(parse_sexp_call_triplet, parse_sexp_call_arguments),
            // possible variable, closing ")", possible annotation
            pair(
                terminated(
                    opt(preceded(multispace1, map(parse_sexp_symbol, Box::new))),
                    delim_ws(tag(")")),
                ),
                opt(preceded(tag("# "), is_not("\r\n"))),
            ),
        ),
        |((triplet, args), (var, annotation))| Sexp::Call {
            triplet,
            args,
            var,
            annotation: annotation.map(|x| x.trim().to_owned()),
        },
    )(inp)
}

fn parse_sexp_call_triplet(inp: &str) -> IResult<&str, Box<Triplet>, ParseError> {
    map(
        separated_pair(
            parse_sexp,
            multispace0,
            delimited(
                delim_ws(tag("(")),
                separated_pair(parse_sexp_string, multispace0, parse_sexp),
                delim_ws(tag(")")),
            ),
        ),
        |(peer_id, (service, function))| Box::new((peer_id, service, function)),
    )(inp)
}

fn parse_sexp_call_arguments(inp: &str) -> IResult<&str, Vec<Sexp>, ParseError> {
    delimited(tag("["), separated_list0(multispace1, parse_sexp), tag("]"))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        let res = Sexp::from_str("symbol");
        assert_eq!(res, Ok(Sexp::Symbol("symbol".to_owned())));
    }

    #[test]
    fn test_symbol_lambda() {
        let res = Sexp::from_str("sym_bol.$.blabla");
        assert_eq!(res, Ok(Sexp::Symbol("sym_bol.$.blabla".to_owned())));
    }

    #[test]
    fn test_string() {
        let res = Sexp::from_str(r#""str ing""#);
        assert_eq!(res, Ok(Sexp::String("str ing".to_owned())));
    }

    #[test]
    fn test_empty_list() {
        let res = Sexp::from_str("()");
        assert_eq!(res, Ok(Sexp::List(vec![])));
    }

    #[test]
    fn test_small_list() {
        let res = Sexp::from_str("(null)");
        assert_eq!(res, Ok(Sexp::List(vec![Sexp::Symbol("null".to_owned())])));
    }

    #[test]
    fn test_call_no_args() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![],
                var: None,
                annotation: None,
            })
        );
    }

    #[test]
    fn test_call_args1() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![Sexp::Symbol("a".to_owned())],
                var: None,
                annotation: None,
            })
        );
    }

    #[test]
    fn test_call_args2() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![Sexp::Symbol("a".to_owned()), Sexp::Symbol("b".to_owned())],
                var: None,
                annotation: None,
            })
        );
    }

    #[test]
    fn test_call_var() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b] var)"#);
        assert_eq!(
            res,
            Ok(Sexp::Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![Sexp::Symbol("a".to_owned()), Sexp::Symbol("b".to_owned())],
                var: Some(Box::new(Sexp::Symbol("var".to_owned()))),
                annotation: None,
            })
        );
    }

    #[test]
    fn test_call_with_annotation() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b] var) # result=42 "#);
        assert_eq!(
            res,
            Ok(Sexp::Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![Sexp::Symbol("a".to_owned()), Sexp::Symbol("b".to_owned())],
                var: Some(Box::new(Sexp::Symbol("var".to_owned()))),
                annotation: Some("result=42".to_owned()),
            })
        );
    }

    #[test]
    fn test_call_with_annotation2() {
        let res = Sexp::from_str(
            r#"(par
  (call peerid ("serv" "func") [a b] var) # result=42
  (call peerid2 ("serv" "func") []))"#,
        );
        assert!(res.is_ok(), "{:?}", res);
    }

    #[test]
    fn test_generic_sexp() {
        let res = Sexp::from_str(" (fold i n ( par (null) (match y \"asdf\" (fail ))) )");
        assert_eq!(
            res,
            Ok(Sexp::List(vec![
                Sexp::Symbol("fold".to_owned()),
                Sexp::Symbol("i".to_owned()),
                Sexp::Symbol("n".to_owned()),
                Sexp::List(vec![
                    Sexp::Symbol("par".to_owned()),
                    Sexp::List(vec![Sexp::Symbol("null".to_owned())]),
                    Sexp::List(vec![
                        Sexp::Symbol("match".to_owned()),
                        Sexp::Symbol("y".to_owned()),
                        Sexp::String("asdf".to_owned()),
                        Sexp::List(vec![Sexp::Symbol("fail".to_owned()),])
                    ])
                ])
            ]))
        );
    }

    #[test]
    fn test_trailing_error() {
        let res = Sexp::from_str("(null))");
        assert!(res.is_err());
    }
}
