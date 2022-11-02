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
use crate::transform::parser::delim_ws;

use air_test_utils::CallServiceResult;
use nom::{error::VerboseError, IResult};

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
    use nom::character::complete::alphanumeric1;
    use nom::combinator::{cut, map_res, recognize};
    use nom::error::context;
    use nom::sequence::separated_pair;

    let equal = || delim_ws(tag("="));
    let json_value = || {
        cut(context(
            "result value has to be a valid JSON",
            recognize(super::json::json_value),
        ))
    };
    let json_map = || {
        cut(context(
            "result value has to be a valid JSON hash",
            recognize(super::json::hash),
        ))
    };

    delim_ws(map_res(
        alt((
            separated_pair(tag(ServiceTagName::Ok.as_ref()), equal(), json_value()),
            separated_pair(tag(ServiceTagName::Error.as_ref()), equal(), json_map()),
            separated_pair(tag(ServiceTagName::SeqOk.as_ref()), equal(), json_map()),
            separated_pair(tag(ServiceTagName::SeqError.as_ref()), equal(), json_map()),
            separated_pair(
                tag(ServiceTagName::Behaviour.as_ref()),
                equal(),
                cut(alphanumeric1),
            ),
            separated_pair(tag(ServiceTagName::Map.as_ref()), equal(), json_map()),
        )),
        |(tag, value): (&str, &str)| {
            let value = value.trim();
            match ServiceTagName::from_str(tag) {
                Ok(ServiceTagName::Ok) => {
                    serde_json::from_str::<JValue>(value).map(ServiceDefinition::Ok)
                }
                Ok(ServiceTagName::Error) => {
                    serde_json::from_str::<CallServiceResult>(value).map(ServiceDefinition::Error)
                }
                Ok(ServiceTagName::SeqOk) => {
                    serde_json::from_str(value).map(ServiceDefinition::SeqOk)
                }
                Ok(ServiceTagName::SeqError) => {
                    serde_json::from_str::<HashMap<String, CallServiceResult>>(value)
                        .map(ServiceDefinition::SeqError)
                }
                Ok(ServiceTagName::Behaviour) => Ok(ServiceDefinition::Behaviour(value.to_owned())),
                Ok(ServiceTagName::Map) => serde_json::from_str(value).map(ServiceDefinition::Map),
                Err(_) => unreachable!("unknown tag {:?}", tag),
            }
        },
    ))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

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

    #[test]
    fn test_map() {
        let res = ServiceDefinition::from_str(r#"map = {"42": [], "a": 2}"#);
        assert_eq!(
            res,
            Ok(ServiceDefinition::Map(maplit::hashmap! {
                "42".to_owned() => json!([]),
                "a".to_owned() => json!(2)
            }))
        );
    }

    #[test]
    fn test_composable() {
        use nom::bytes::complete::tag;
        use nom::multi::separated_list1;

        let res = separated_list1(tag(";"), parse_kw)(r#"ok={"ret_code": 0};map={"default": 42}"#);
        assert_eq!(
            res,
            Ok((
                "",
                vec![
                    ServiceDefinition::Ok(json!({"ret_code":0,})),
                    ServiceDefinition::Map(maplit::hashmap! {"default".to_owned()=>json!(42),})
                ]
            ))
        )
    }
}
