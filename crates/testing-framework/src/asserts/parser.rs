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

use super::{Assertion, AssertionBranch, AssertionChain, Condition, Meta, ServiceDesc};
use crate::services::JValue;

use air_test_utils::CallServiceResult;
use nom::{error::VerboseError, IResult, InputTakeAtPosition, Parser};

use std::{collections::HashMap, str::FromStr};

type ParseError<'inp> = VerboseError<&'inp str>;

enum Pair {
    Meta(Meta),
    Condition(Condition),
    Assertion(Assertion),
    ServiceDesc(ServiceDesc),
}

// this implementation uses nom as quick and dirty solution.  One might consider using
// lalrpop for codebase consistency.
impl FromStr for AssertionChain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        nom::combinator::all_consuming(parse_assertion_chain)(s)
            .map(|(_, val)| val)
            .map_err(|e| e.to_string())
    }
}

// chain is ";"-separated list of branches; can contain spaces
// examples:
// "id=firstcall"
// "id=firstcall,callback=check_values"
// "id=firstcall,iter=0,callback=check_values0; iter=1,callback=check_values1"
fn parse_assertion_chain(s: &str) -> IResult<&str, AssertionChain, ParseError> {
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::multi::separated_list1;

    map(
        separated_list1(tag(";"), parse_assertion_branch),
        |assertions| AssertionChain { assertions },
    )(s)
}

// branch is comma-separated list of key=value pairs; can contain spaces
// examples:
// "id=firstcall"
// "id=firstcall,callback=check_values"
// "id = firstcall, callback = check_values"
fn parse_assertion_branch(s: &str) -> IResult<&str, AssertionBranch, ParseError> {
    use nom::bytes::complete::tag;
    use nom::combinator::map_res;
    use nom::multi::separated_list1;

    map_res(delim_ws(separated_list1(tag(","), parse_kw)), |pairs| {
        let mut assertions = vec![];
        let mut conditions = vec![];
        let mut metas = vec![];
        let mut services = vec![];

        for pair in pairs {
            match pair {
                Pair::Meta(m) => metas.push(m),
                Pair::Condition(c) => conditions.push(c),
                Pair::Assertion(a) => assertions.push(a),
                Pair::ServiceDesc(s) => services.push(s),
            }
        }
        if services.len() > 1 {
            Err("Multiple service descriptors found")
        } else {
            let service_desc = services.into_iter().next();
            Ok(AssertionBranch {
                assertions,
                conditions,
                metas,
                service_desc,
            })
        }
    })(s)
}

// kw "=" val
// example: "id=firstcall"
fn parse_kw(inp: &str) -> IResult<&str, Pair, ParseError> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, u32 as parse_u32};
    use nom::combinator::{cut, map, map_res, rest};
    use nom::error::context;
    use nom::sequence::{pair, preceded, separated_pair};

    let equal = || delim_ws(tag("="));

    delim_ws(alt((
        map(
            preceded(
                pair(tag("iter"), equal()),
                cut(context("iter argument", parse_u32)),
            ),
            |n| Pair::Condition(Condition::Iter(n)),
        ),
        map(
            preceded(
                pair(tag("seq"), equal()),
                cut(context("seq argument", parse_u32)),
            ),
            |n| Pair::Assertion(Assertion::Seq(n)),
        ),
        map(
            preceded(
                pair(tag("is_called"), equal()),
                cut(context("is_called argument", parse_bool)),
            ),
            |flag| Pair::Assertion(Assertion::IsCalled(flag)),
        ),
        map_res(
            separated_pair(
                alt((
                    tag("result"),
                    tag("call_result"),
                    tag("seq_result"),
                    tag("service"),
                )),
                equal(),
                cut(context(
                    "result value is consumed to end and has to be a valid JSON",
                    // TODO taking rest of input means we cannot provide values for
                    // each branch; one might use some json parser, including nom's one
                    rest,
                )),
            ),
            |(tag, value): (&str, &str)| {
                let value = value.trim();
                match tag {
                    "result" => serde_json::from_str::<JValue>(value).map(ServiceDesc::Result),
                    "call_result" => serde_json::from_str::<CallServiceResult>(value)
                        .map(ServiceDesc::CallResult),
                    "seq_result" => serde_json::from_str::<HashMap<String, JValue>>(value)
                        .map(ServiceDesc::SeqResult),
                    "service" => Ok(ServiceDesc::Service(value.to_owned())),
                    _ => unreachable!("unknown tag {:?}", tag),
                }
                .map(Pair::ServiceDesc)
            },
        ),
        map_res(
            separated_pair(alphanumeric1, delim_ws(tag("=")), alphanumeric1),
            |(key, val)| match key {
                "id" => Ok(Pair::Meta(Meta::Id(val.to_owned()))),
                "on" => todo!(),
                "before" => Ok(Pair::Assertion(Assertion::Before(val.to_owned()))),
                "after" => Ok(Pair::Assertion(Assertion::After(val.to_owned()))),
                "filter" => Ok(Pair::Condition(Condition::Filter(val.to_owned()))),
                "callback" => Ok(Pair::Assertion(Assertion::Callback(val.to_owned()))),
                "next_peer_pk" => Ok(Pair::Assertion(Assertion::NextPeerPk(val.to_owned()))),
                _ => Err(()),
            },
        ),
    )))(inp)
}

pub(crate) fn delim_ws<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    E: nom::error::ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: nom::AsChar + Clone,
{
    use nom::character::complete::multispace0;
    use nom::sequence::delimited;

    delimited(multispace0, f, multispace0)
}

fn parse_bool<'inp, E>(inp: &'inp str) -> IResult<&'inp str, bool, E>
where
    E: nom::error::ParseError<&'inp str>,
{
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::value;

    alt((value(true, tag("true")), value(false, tag("false"))))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let res = AssertionChain::from_str("");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_garbage0() {
        let res = AssertionChain::from_str("garbage");
        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_parse_garbage1() {
        let res = AssertionChain::from_str("id=myid;garbage");
        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_parse_single() {
        let res = AssertionChain::from_str("id=myid,before=other");

        assert_eq!(
            res,
            Ok(AssertionChain {
                assertions: vec![AssertionBranch {
                    assertions: vec![Assertion::Before("other".to_owned())],
                    metas: vec![Meta::Id("myid".to_owned())],
                    conditions: vec![],
                    service_desc: None,
                }]
            })
        );
    }

    #[test]
    fn test_parse_multiple() {
        let res = AssertionChain::from_str("id=myid,iter=0,before=other;iter=1,after=another");

        assert_eq!(
            res,
            Ok(AssertionChain {
                assertions: vec![
                    AssertionBranch {
                        assertions: vec![Assertion::Before("other".to_owned())],
                        conditions: vec![Condition::Iter(0)],
                        metas: vec![Meta::Id("myid".to_owned())],
                        service_desc: None,
                    },
                    AssertionBranch {
                        assertions: vec![Assertion::After("another".to_owned())],
                        conditions: vec![Condition::Iter(1)],
                        metas: vec![],
                        service_desc: None,
                    },
                ]
            })
        );
    }

    #[test]
    fn test_parse_whitespace() {
        let res = AssertionChain::from_str(
            " id = myid  , iter = 0 ,\tbefore = other ;\n iter =1,     after=another ",
        );

        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![
                AssertionBranch::new(
                    vec![Condition::Iter(0)],
                    vec![Assertion::Before("other".to_owned())],
                    vec![Meta::Id("myid".to_owned())],
                    None,
                ),
                AssertionBranch::new(
                    vec![Condition::Iter(1)],
                    vec![Assertion::After("another".to_owned())],
                    vec![],
                    None,
                ),
            ]))
        );
    }

    #[test]
    fn test_iter_malformed() {
        let res = AssertionChain::from_str("iter=$");

        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_iter() {
        let res = AssertionChain::from_str("iter=42");

        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![AssertionBranch::from_conditions(
                vec![Condition::Iter(42)]
            ),]))
        );
    }

    #[test]
    fn test_seq_malformed() {
        let res = AssertionChain::from_str("seq=$");

        assert!(res.is_err());
    }

    #[test]
    fn test_seq() {
        let res = AssertionChain::from_str("seq=42");

        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![AssertionBranch::from_assertions(
                vec![Assertion::Seq(42)]
            ),]))
        );
    }

    #[test]
    fn test_is_called_malformed() {
        let res = AssertionChain::from_str("is_called=1");

        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_is_called() {
        for (inp, val) in [
            ("is_called=true", true),
            ("is_called=false", false),
            ("is_called = false ", false),
        ] {
            assert_eq!(
                AssertionChain::from_str(inp),
                Ok(AssertionChain::new(vec![AssertionBranch::from_assertions(
                    vec![Assertion::IsCalled(val)]
                )])),
                "failed on {:?}",
                inp,
            );
        }
    }

    #[test]
    fn test_result_service() {
        use serde_json::json;

        let res = AssertionChain::from_str(r#"id=myid,result={"this":["is","value"]}"#);
        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![AssertionBranch::new(
                vec![],
                vec![],
                vec![Meta::Id("myid".to_owned()),],
                Some(ServiceDesc::Result(json!({"this": ["is", "value"]})))
            )]))
        );
    }

    #[test]
    fn test_result_service_malformed() {
        let res = AssertionChain::from_str(r#"id=myid,result={"this":["is","value"]"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_call_result() {
        use serde_json::json;

        let res =
            AssertionChain::from_str(r#"id=myid,call_result={"ret_code": 0, "result": [1, 2, 3]}"#);
        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![AssertionBranch::new(
                vec![],
                vec![],
                vec![Meta::Id("myid".to_owned()),],
                Some(ServiceDesc::CallResult(CallServiceResult::ok(json!([
                    1, 2, 3
                ])))),
            )]))
        );
    }

    #[test]
    fn test_call_result_malformed() {
        let res =
            AssertionChain::from_str(r#"id=myid,call_result={"retcode": 0, "result": [1, 2, 3]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_call_result_invalid() {
        let res =
            AssertionChain::from_str(r#"id=myid,call_result={"ret_code": 0, "result": 1, 2, 3]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_result() {
        use serde_json::json;

        let res =
            AssertionChain::from_str(r#"id=myid,seq_result={"default": 42, "1": true, "3": []}"#);
        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![AssertionBranch::new(
                vec![],
                vec![],
                vec![Meta::Id("myid".to_owned()),],
                Some(ServiceDesc::SeqResult(maplit::hashmap! {
                    "default".to_owned() => json!(42),
                    "1".to_owned() => json!(true),
                    "3".to_owned() => json!([]),
                }))
            )]))
        );
    }

    #[test]
    fn test_seq_result_malformed() {
        let res =
            AssertionChain::from_str(r#"id=myid,seq_result={"default": 42, "1": true, "3": ]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_result_invalid() {
        // TODO perhaps, we should support both arrays and maps
        let res = AssertionChain::from_str(r#"id=myid,seq_result=[42, 43]"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_service() {
        let res = AssertionChain::from_str(r#"service=echo"#);
        assert_eq!(
            res,
            Ok(AssertionChain::new(vec![
                AssertionBranch::from_service_desc(ServiceDesc::Service("echo".to_owned()))
            ]))
        );
    }
}
