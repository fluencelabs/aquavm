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
use crate::asserts::ServiceDefinition;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, multispace0, multispace1, one_of, space1};
use nom::combinator::{cut, map, map_res, opt, recognize, value};
use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::multi::{many0, many1, many1_count, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::{IResult, InputTakeAtPosition};
use nom_locate::LocatedSpan;

use std::str::FromStr;

type Input<'inp> = LocatedSpan<&'inp str>;
type ParseError<'inp> = VerboseError<Input<'inp>>;

impl FromStr for Sexp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            terminated(tag("("), sexp_multispace0),
            cut(terminated(
                map(separated_list0(sexp_multispace1, parse_sexp), Sexp::list),
                preceded(
                    sexp_multispace0,
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
                    alt((
                        is_not("\""),
                        //
                        tag(""),
                    )),
                    context("closing quotes not found", tag("\"")),
                )),
            ),
        ),
        Sexp::string,
    )(inp)
}

fn parse_sexp_symbol(inp: Input<'_>) -> IResult<Input<'_>, Sexp, ParseError<'_>> {
    map(
        recognize(pair(
            many1_count(alt((value((), alphanumeric1), value((), one_of("_-.$#%"))))),
            opt(delimited(tag("["), parse_sexp_symbol, tag("]"))),
        )),
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
                    opt(preceded(sexp_multispace1, map(parse_sexp_symbol, Box::new))),
                    preceded(sexp_multispace0, tag(")")),
                ),
                alt((
                    opt(preceded(pair(space1, tag("; ")), parse_annotation)),
                    value(None, sexp_multispace0),
                )),
            ),
        ),
        |((triplet, args), (var, annotation))| {
            Sexp::Call(Call {
                triplet,
                args,
                var,
                service_desc: annotation,
            })
        },
    )(inp)
}

fn parse_annotation(inp: Input<'_>) -> IResult<Input<'_>, ServiceDefinition, ParseError<'_>> {
    map_res(
        is_not("\r\n"),
        |span: Input<'_>| -> Result<ServiceDefinition, ParseError<'_>> {
            Ok(ServiceDefinition::from_str(&span).unwrap())
        },
    )(inp)
}

fn parse_sexp_call_triplet(inp: Input<'_>) -> IResult<Input<'_>, Box<Triplet>, ParseError<'_>> {
    map(
        separated_pair(
            context("triplet peer_id", parse_sexp),
            sexp_multispace0,
            delimited(
                delim_ws(tag("(")),
                separated_pair(
                    context("triplet service name", parse_sexp_string),
                    sexp_multispace0,
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

pub(crate) fn delim_ws<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
    I: nom::InputTakeAtPosition
        + nom::InputLength
        + for<'a> nom::Compare<&'a str>
        + nom::InputTake
        + Clone,
    <I as InputTakeAtPosition>::Item: nom::AsChar + Clone,
    for<'a> &'a str: nom::FindToken<<I as InputTakeAtPosition>::Item>,
{
    delimited(sexp_multispace0, f, sexp_multispace0)
}

pub(crate) fn sexp_multispace0<I, E>(inp: I) -> IResult<I, (), E>
where
    E: nom::error::ParseError<I>,
    I: InputTakeAtPosition
        + nom::InputLength
        + for<'a> nom::Compare<&'a str>
        + nom::InputTake
        + Clone,
    <I as InputTakeAtPosition>::Item: nom::AsChar + Clone,
    for<'a> &'a str: nom::FindToken<<I as InputTakeAtPosition>::Item>,
{
    map(
        opt(many0(pair(
            // white space
            multispace1,
            // possible ;;, ;;; comment
            opt(pair(tag(";;"), is_not("\r\n"))),
        ))),
        |_| (),
    )(inp)
}

pub(crate) fn sexp_multispace1(inp: Input<'_>) -> IResult<Input<'_>, (), ParseError<'_>> {
    map(
        // It's not the fastest implementation, but easiest to write.
        // It passes initial whitespace two times.
        many1(alt((
            map(
                pair(
                    // white space
                    multispace0,
                    // ;;, ;;;, etc comment
                    pair(tag(";;"), is_not("\r\n")),
                ),
                |_| (),
            ),
            map(multispace1, |_| ()),
        ))),
        |_| (),
    )(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asserts::ServiceDefinition;

    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_multispace0_empty() {
        let res = sexp_multispace0::<_, ()>("");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace0_spaces() {
        let res = sexp_multispace0::<_, ()>("  ");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace0_comment() {
        let res = sexp_multispace0::<_, ()>(";; this is comment");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace0_comment_with_space() {
        let res = sexp_multispace0::<_, ()>(" ;; ");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace0_multiline() {
        let res = sexp_multispace0::<_, ()>(" ;; \n ;;;; \n ");
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace1_empty() {
        let res = sexp_multispace1("".into());
        assert!(res.is_err());
    }

    #[test]
    fn test_multispace1_space() {
        let res = sexp_multispace1(" ".into());
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace1_comment() {
        let res = sexp_multispace1(" ;; ".into());
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

    #[test]
    fn test_multispace1_multiline() {
        let res = sexp_multispace1(" ;; \n ;;;; \n ".into());
        assert!(res.is_ok(), "{}", res.unwrap_err());
    }

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
    fn test_symbol_stream() {
        let res = Sexp::from_str("$stream");
        assert_eq!(res, Ok(Sexp::symbol("$stream")));
    }

    #[test]
    fn test_symbol_canon() {
        let res = Sexp::from_str("#canon");
        assert_eq!(res, Ok(Sexp::symbol("#canon")));
    }

    #[test]
    fn test_symbol_lambda2() {
        let res = Sexp::from_str(r#"$result.$[0]"#);
        assert_eq!(res, Ok(Sexp::symbol(r#"$result.$[0]"#)));
    }

    #[test]
    fn test_string_empty() {
        let res = Sexp::from_str(r#""""#);
        assert_eq!(res, Ok(Sexp::string("")));
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
                service_desc: None,
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
                    service_desc: None,
                }),
                Sexp::Call(Call {
                    triplet: Box::new((
                        Sexp::symbol("peer_id"),
                        Sexp::string("serv"),
                        Sexp::string("func"),
                    )),
                    args: vec![],
                    var: None,
                    service_desc: None,
                }),
            ]))
        );
    }

    #[test]
    fn test_call_annotation_newline() {
        let res = Sexp::from_str(
            r#"(seq (call peer_id ("serv" "func") [])
; result=42
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
                service_desc: None,
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
                service_desc: None,
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
                service_desc: None,
            }))
        );
    }

    #[test]
    fn test_call_with_annotation() {
        let res = Sexp::from_str(r#"(call peer_id ("serv" "func") [a b] var) ; ok=42 "#);
        let expected_annotation = ServiceDefinition::Ok(json!(42));
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
                service_desc: Some(expected_annotation),
            }))
        );
    }

    #[test]
    fn test_call_with_annotation2() {
        let res = Sexp::from_str(
            r#"(par
  (call peerid ("serv" "func") [a b] var) ; ok=42
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

    #[test]
    fn test_canon_syntax() {
        let sexp_str = r#"(seq (canon peer_id $stream #canon) (fold #canon i (next)))"#;
        let res = Sexp::from_str(sexp_str);
        assert!(res.is_ok(), "{:?}", res);
    }

    #[test]
    fn test_comments() {
        let sexp_str = r#" ;; One comment
( ;;; Second comment
  ;; The third one
  (par ;;;; Comment comment comment
    ;;;; Comment comment comment
    (null)  ;;;;; Comment
    (fail  ;; Fails
        1    ;; Retcode
        "test"  ;; Message
        ;; Nothing more
    )
  )   ;;;;; Comment
  ;;;;; Comment
 )  ;;;;; Comment
;;; Comment
"#;
        let res = Sexp::from_str(sexp_str);
        assert_eq!(
            res,
            Ok(Sexp::list(vec![Sexp::list(vec![
                Sexp::symbol("par"),
                Sexp::list(vec![Sexp::symbol("null"),]),
                Sexp::list(vec![
                    Sexp::symbol("fail"),
                    Sexp::symbol("1"),
                    Sexp::string("test"),
                ]),
            ])]))
        );
    }
}
