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

use super::{ServiceDefinition, ServiceTagName};
use crate::services::JValue;

use air_test_utils::CallServiceResult;
use nom::{error::VerboseError, IResult, InputTakeAtPosition, Parser};

use std::{collections::HashMap, str::FromStr};

type ParseError<'inp> = VerboseError<&'inp str>;

impl FromStr for ServiceDefinition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        nom::combinator::all_consuming(parse_kw)(s)
            .map(|(_, service_definition)| service_definition)
            .map_err(|e| e.to_string())
    }
}

// kw "=" val
// example: "id=firstcall"
pub fn parse_kw(inp: &str) -> IResult<&str, ServiceDefinition, ParseError> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::{cut, map_res, rest};
    use nom::error::context;
    use nom::sequence::separated_pair;

    let equal = || delim_ws(tag("="));

    delim_ws(map_res(
        separated_pair(
            alt((
                tag(ServiceTagName::Ok.as_ref()),
                tag(ServiceTagName::Error.as_ref()),
                tag(ServiceTagName::SeqOk.as_ref()),
                tag(ServiceTagName::SeqError.as_ref()),
                tag(ServiceTagName::Behaviour.as_ref()),
            )),
            equal(),
            cut(context(
                "result value is consumed to end and has to be a valid JSON",
                rest,
            )),
        ),
        |(tag, value): (&str, &str)| {
            let value = value.trim();
            match ServiceTagName::from_str(tag) {
                Ok(ServiceTagName::Ok) => {
                    serde_json::from_str::<JValue>(value).map(ServiceDefinition::Ok)
                }
                Ok(ServiceTagName::Error) => {
                    serde_json::from_str::<CallServiceResult>(value).map(ServiceDefinition::Error)
                }
                Ok(ServiceTagName::SeqOk) => serde_json::from_str::<HashMap<String, JValue>>(value)
                    .map(ServiceDefinition::SeqOk),
                Ok(ServiceTagName::SeqError) => {
                    serde_json::from_str::<HashMap<String, CallServiceResult>>(value)
                        .map(ServiceDefinition::SeqError)
                }
                Ok(ServiceTagName::Behaviour) => Ok(ServiceDefinition::Behaviour(value.to_owned())),
                Err(_) => unreachable!("unknown tag {:?}", tag),
            }
        },
    ))(inp)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let res = ServiceDefinition::from_str("");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_garbage0() {
        let res = ServiceDefinition::from_str("garbage");
        assert!(res.is_err(), "{:?}", res);
    }

    #[test]
    fn test_result_service() {
        use serde_json::json;

        let res = ServiceDefinition::from_str(r#"ok={"this":["is","value"]}"#);
        assert_eq!(
            res,
            Ok(ServiceDefinition::Ok(json!({"this": ["is", "value"]}))),
        );
    }

    #[test]
    fn test_result_service_malformed() {
        let res = ServiceDefinition::from_str(r#"ok={"this":["is","value"]"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_call_result() {
        use serde_json::json;

        let res = ServiceDefinition::from_str(r#"err={"ret_code": 0, "result": [1, 2, 3]}"#);
        assert_eq!(
            res,
            Ok(ServiceDefinition::Error(CallServiceResult::ok(json!([
                1, 2, 3
            ])))),
        );
    }

    #[test]
    fn test_call_result_malformed() {
        let res = ServiceDefinition::from_str(r#"err={"retcode": 0, "result": [1, 2, 3]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_call_result_invalid() {
        let res = ServiceDefinition::from_str(r#"err={"ret_code": 0, "result": 1, 2, 3]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_ok() {
        use serde_json::json;

        let res = ServiceDefinition::from_str(r#"seq_ok={"default": 42, "1": true, "3": []}"#);
        assert_eq!(
            res,
            Ok(ServiceDefinition::SeqOk(maplit::hashmap! {
                "default".to_owned() => json!(42),
                "1".to_owned() => json!(true),
                "3".to_owned() => json!([]),
            })),
        );
    }

    #[test]
    fn test_seq_ok_malformed() {
        let res = ServiceDefinition::from_str(r#"seq_ok={"default": 42, "1": true, "3": ]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_ok_invalid() {
        // TODO perhaps, we should support both arrays and maps
        let res = ServiceDefinition::from_str(r#"seq_ok=[42, 43]"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_error() {
        use serde_json::json;

        let res = ServiceDefinition::from_str(
            r#"seq_error={"default": {"ret_code": 0, "result": 42}, "1": {"ret_code": 0, "result": true}, "3": {"ret_code": 1, "result": "error"}}"#,
        );
        assert_eq!(
            res,
            Ok(ServiceDefinition::SeqError(maplit::hashmap! {
                "default".to_owned() => CallServiceResult::ok(json!(42)),
                "1".to_owned() => CallServiceResult::ok(json!(true)),
                "3".to_owned() => CallServiceResult::err(1, json!("error")),
            })),
        );
    }

    #[test]
    fn test_seq_error_malformed() {
        let res = ServiceDefinition::from_str(r#"seq_error={"default": 42, "1": true]}"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_seq_error_invalid() {
        // TODO perhaps, we should support both arrays and maps
        let res = ServiceDefinition::from_str(r#"seq_error=[42, 43]"#);
        assert!(res.is_err());
    }

    #[test]
    fn test_behaviour() {
        let res = ServiceDefinition::from_str(r#"behaviour=echo"#);
        assert_eq!(res, Ok(ServiceDefinition::Behaviour("echo".to_owned())),);
    }
}
