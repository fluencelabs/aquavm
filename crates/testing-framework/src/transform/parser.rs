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

use super::{Call, Sexp, Triplet};
use crate::asserts::parser::delim_ws;
use crate::asserts::AssertionChain;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, multispace0, multispace1, one_of, space1};
use nom::combinator::{cut, map, map_res, opt, recognize, value};
use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::multi::{many1_count, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::IResult;
use nom_locate::LocatedSpan;

use std::str::FromStr;

type Input<'inp> = LocatedSpan<&'inp str>;
type ParseError<'inp> = VerboseError<Input<'inp>>;

impl FromStr for Sexp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::{
            asserts::parser::delim_ws,
            transform::parser::{parse_error_to_message, parse_sexp},
        };
        use nom::combinator::all_consuming;

        let span = nom_locate::LocatedSpan::new(s);
        cut(all_consuming(delim_ws(parse_sexp)))(span)
            .map(|(_, v)| v)
            .map_err(parse_error_to_message)
    }
}

pub(crate) fn parse_error_to_message(e: nom::Err<ParseError>) -> String {
    let e = match e {
        nom::Err::Failure(e) => e,
        _ => panic!("shouldn't happen because of top-level cut"),
    };
    let contexts = e
        .errors
        .iter()
        .rev()
        .filter_map(|(span, kind)| {
            if let VerboseErrorKind::Context(c) = kind {
                Some(format!(
                    "  {}:{}: {}",
                    span.location_line(),
                    span.get_utf8_column(),
                    c
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if contexts.is_empty() {
        e.to_string()
    } else {
        format!("Failed to parse the script:\n{}", contexts.join("\n"))
    }
}

pub(crate) fn parse_sexp(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    alt((
        parse_sexp_call,
        parse_sexp_list,
        parse_sexp_string,
        parse_sexp_symbol,
    ))(inp)
}

fn parse_sexp_list(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    context(
        "within generic list",
        preceded(
            terminated(tag("("), multispace0),
            cut(terminated(
                map(separated_list0(multispace1, parse_sexp), Sexp::list),
                preceded(
                    multispace0,
                    context("closing parentheses not found", tag(")")),
                ),
            )),
        ),
    )(inp)
}

fn parse_sexp_string(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    // N.B. escape are rejected by AIR parser, but we simply treat backslash
    // as any other character
    map(
        context(
            "within string",
            preceded(
                tag("\""),
                cut(terminated(
                    is_not("\""),
                    context("closing quotes not found", tag("\"")),
                )),
            ),
        ),
        Sexp::string,
    )(inp)
}

fn parse_sexp_symbol(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    map(
        recognize(many1_count(alt((
            value((), alphanumeric1),
            value((), one_of("_.$")),
        )))),
        Sexp::symbol,
    )(inp)
}

fn parse_sexp_call(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    preceded(
        delim_ws(tag("(")),
        preceded(
            tag("call "),
            context("within call list", cut(parse_sexp_call_content)),
        ),
        // call_content includes ")" and possible comment ^
    )(inp)
}

fn parse_sexp_call_content(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    map(
        pair(
            // triplet and arguments
            pair(parse_sexp_call_triplet, parse_sexp_call_arguments),
            // possible variable, closing ")", possible annotation
            pair(
                terminated(
                    opt(preceded(multispace1, map(parse_sexp_symbol, Box::new))),
                    preceded(multispace0, tag(")")),
                ),
                alt((
                    opt(preceded(pair(space1, tag("# ")), parse_annotation)),
                    value(None, multispace0),
                )),
            ),
        ),
        |((triplet, args), (var, annotation))| {
            Sexp::Call(Call {
                triplet,
                args,
                var,
                annotation,
            })
        },
    )(inp)
}

fn parse_annotation(inp: Input<'_>) -> IResult<Input<'_>, AssertionChain, ParseError<'_>> {
    map_res(
        is_not("\r\n"),
        |span: Input<'_>| -> Result<AssertionChain, ParseError<'_>> {
            Ok(AssertionChain::from_str(&span).unwrap())
        },
    )(inp)
}

fn parse_sexp_call_triplet(inp: Input<'_>) -> IResult<Input<'_>, Box<Triplet>, ParseError<'_>> {
    map(
        separated_pair(
            context("triplet peer_id", parse_sexp),
            multispace0,
            delimited(
                delim_ws(tag("(")),
                separated_pair(
                    context("triplet service name", parse_sexp_string),
                    multispace0,
                    context("triplet function name", parse_sexp),
                ),
                delim_ws(tag(")")),
            ),
        ),
        |(peer_id, (service, function))| Box::new((peer_id, service, function)),
    )(inp)
}

fn parse_sexp_call_arguments(inp: Input<'_>) -> IResult<Input<'_>, Vec<Sexp>, ParseError<'_>> {
    delimited(tag("["), separated_list0(multispace1, parse_sexp), tag("]"))(inp)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    use crate::asserts::{AssertionBranch, Meta};

    #[test]
    fn test_symbol() {
        let res = Sexp::from_str("symbol");
        assert_eq!(res, Ok(Sexp::symbol("symbol")));
    }

    #[test]
    fn test_symbol_lambda() {
        let res = Sexp::from_str("sym_bol.$.blabla");
        assert_eq!(res, Ok(Sexp::symbol("sym_bol.$.blabla")));
    }

    #[test]
    fn test_string() {
        let res = Sexp::from_str(r#""str ing""#);
        assert_eq!(res, Ok(Sexp::string("str ing")));
    }

    #[test]
    fn test_empty_list() {
        let res = Sexp::from_str("()");
        assert_eq!(res, Ok(Sexp::List(vec![])));
    }

    #[test]
    fn test_small_list() {
        let res = Sexp::from_str("(null)");
        assert_eq!(res, Ok(Sexp::list(vec![Sexp::symbol("null")])));
    }

    #[test]
    fn test_call_no_args() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call(Call {
                triplet: Box::new((
                    Sexp::symbol("peer_id"),
                    Sexp::string("serv"),
                    Sexp::string("func"),
                )),
                args: vec![],
                var: None,
                annotation: None,
            }))
        );
    }

    #[test]
    fn test_call_after_call() {
        let res = Sexp::from_str(
            r#"(seq
    (call peer_id ("serv" "func") [])
    (call peer_id ("serv" "func") [])
)"#,
        );
        assert_eq!(
            res,
            Ok(Sexp::list(vec![
                Sexp::symbol("seq"),
                Sexp::Call(Call {
                    triplet: Box::new((
                        Sexp::symbol("peer_id"),
                        Sexp::string("serv"),
                        Sexp::string("func"),
                    )),
                    args: vec![],
                    var: None,
                    annotation: None,
                }),
                Sexp::Call(Call {
                    triplet: Box::new((
                        Sexp::symbol("peer_id"),
                        Sexp::string("serv"),
                        Sexp::string("func"),
                    )),
                    args: vec![],
                    var: None,
                    annotation: None,
                }),
            ]))
        );
    }

    #[test]
    fn test_call_annotation_newline() {
        let res = Sexp::from_str(
            r#"(seq (call peer_id ("serv" "func") [])
# result=42
)"#,
        );
        assert_eq!(
            res,
            Err("Failed to parse the script:\n  1:1: within generic list\n  2:1: closing parentheses not found".to_owned())
        );
    }

    #[test]
    fn test_call_args1() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call(Call {
                triplet: Box::new((
                    Sexp::symbol("peer_id"),
                    Sexp::string("serv"),
                    Sexp::string("func"),
                )),
                args: vec![Sexp::symbol("a")],
                var: None,
                annotation: None,
            }))
        );
    }

    #[test]
    fn test_call_args2() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b])"#);
        assert_eq!(
            res,
            Ok(Sexp::Call(Call {
                triplet: Box::new((
                    Sexp::symbol("peer_id"),
                    Sexp::string("serv"),
                    Sexp::string("func"),
                )),
                args: vec![Sexp::symbol("a"), Sexp::symbol("b")],
                var: None,
                annotation: None,
            }))
        );
    }

    #[test]
    fn test_call_var() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b] var)"#);
        assert_eq!(
            res,
            Ok(Sexp::Call(Call {
                triplet: Box::new((
                    Sexp::Symbol("peer_id".to_owned()),
                    Sexp::String("serv".to_owned()),
                    Sexp::String("func".to_owned()),
                )),
                args: vec![Sexp::Symbol("a".to_owned()), Sexp::Symbol("b".to_owned())],
                var: Some(Box::new(Sexp::Symbol("var".to_owned()))),
                annotation: None,
            }))
        );
    }

    #[test]
    fn test_call_with_annotation() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b] var) # result=42 "#);
        let expected_annotation =
            AssertionChain::new(vec![AssertionBranch::from_metas(vec![Meta::Result(
                json!(42),
            )])]);
        assert_eq!(
            res,
            Ok(Sexp::Call(Call {
                triplet: Box::new((
                    Sexp::symbol("peer_id"),
                    Sexp::string("serv"),
                    Sexp::string("func"),
                )),
                args: vec![Sexp::symbol("a"), Sexp::symbol("b")],
                var: Some(Box::new(Sexp::symbol("var"))),
                annotation: Some(expected_annotation),
            }))
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
            Ok(Sexp::list(vec![
                Sexp::symbol("fold"),
                Sexp::symbol("i"),
                Sexp::symbol("n"),
                Sexp::list(vec![
                    Sexp::symbol("par"),
                    Sexp::list(vec![Sexp::symbol("null")]),
                    Sexp::list(vec![
                        Sexp::symbol("match"),
                        Sexp::symbol("y"),
                        Sexp::string("asdf"),
                        Sexp::list(vec![Sexp::symbol("fail"),])
                    ])
                ])
            ]))
        );
    }

    #[test]
    fn test_trailing_error() {
        let res = Sexp::from_str("(null))");
        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_incomplete_string() {
        let err = Sexp::from_str(
            r#"(seq
   "string"#,
        )
        .unwrap_err();
        assert_eq!(
            err,
            "Failed to parse the script:
  1:1: within generic list
  2:4: within string
  2:11: closing quotes not found"
        );
    }

    #[test]
    fn test_incomplete_list() {
        let err = Sexp::from_str(
            r#"(seq
   "string"
"#,
        )
        .unwrap_err();
        assert_eq!(
            err,
            "Failed to parse the script:
  1:1: within generic list
  3:1: closing parentheses not found"
        );
    }

    #[test]
    fn test_parse_fmt_call() {
        let sexp_str = r#"(call "my_id" ("serv" "function") [other_peer_id "other_arg"])"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_call_var() {
        let sexp_str = r#"(call "my_id" ("serv" "function") [other_peer_id "other_arg"] var)"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_symbol() {
        let sexp_str = "symbol";
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_string() {
        let sexp_str = r#""my_id""#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }

    #[test]
    fn test_parse_fmt_sexp() {
        let sexp_str = r#"(par (ap x y) (fold x y (next)))"#;
        let sexp = Sexp::from_str(sexp_str).unwrap();
        assert_eq!(format!("{}", sexp), sexp_str);
    }
}
