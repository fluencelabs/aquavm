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

use air_test_utils::{
    prelude::{echo_call_service, unit_call_service},
    CallServiceResult,
};
use nom::IResult;
use serde_json::json;
use strum::{AsRefStr, EnumDiscriminants, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(strum(serialize_all = "snake_case"))]
#[strum_discriminants(derive(AsRefStr, EnumString))]
#[strum_discriminants(name(BehaviorTagName))]
pub enum Behavior {
    Echo,
    Unit,
    Service,
    Function,
    Arg(usize),
    Tetraplet,
}

pub(crate) fn parse_behaviour(inp: &str) -> IResult<&str, Behavior, super::parser::ParseError> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32 as u32_parse;
    use nom::combinator::{map, value};
    use nom::sequence::{pair, preceded};

    alt((
        value(Behavior::Echo, tag(BehaviorTagName::Echo.as_ref())),
        value(Behavior::Unit, tag(BehaviorTagName::Unit.as_ref())),
        value(Behavior::Function, tag(BehaviorTagName::Function.as_ref())),
        value(Behavior::Service, tag(BehaviorTagName::Service.as_ref())),
        map(
            preceded(
                pair(tag(BehaviorTagName::Arg.as_ref()), tag(".")),
                u32_parse,
            ),
            |n| Behavior::Arg(n as _),
        ),
        value(
            Behavior::Tetraplet,
            tag(BehaviorTagName::Tetraplet.as_ref()),
        ),
    ))(inp)
}

impl Behavior {
    pub(crate) fn call(&self, params: air_test_utils::CallRequestParams) -> CallServiceResult {
        use Behavior::*;

        match self {
            Echo => {
                println!("echo_call_service() {:#?}", params);
                echo_call_service()(params)
            }
            Unit => unit_call_service()(params),
            Function => CallServiceResult::ok(params.function_name.into()),
            Service => CallServiceResult::ok(params.service_id.into()),
            Arg(n) => match params.arguments.get(*n) {
                Some(val) => CallServiceResult::ok(val.clone()),
                None => CallServiceResult::err(
                    // TODO test-utils uses just json!{ "default" } value.
                    42,
                    json!("not enough arguments"),
                ),
            },
            Tetraplet => CallServiceResult::ok(serde_json::to_value(&params.tetraplets).unwrap()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_behavior_echo() {
        let res = parse_behaviour("echo");
        assert_eq!(
            res,
            Ok(("", Behavior::Echo)),
            "{:?}",
            BehaviorTagName::Echo.as_ref()
        );
    }

    #[test]
    fn test_parse_behavior_unit() {
        let res = parse_behaviour("unit");
        assert_eq!(res, Ok(("", Behavior::Unit)));
    }

    #[test]
    fn test_parse_behavior_service() {
        let res = parse_behaviour("service");
        assert_eq!(res, Ok(("", Behavior::Service)));
    }

    #[test]
    fn test_parse_behavior_function() {
        let res = parse_behaviour("function");
        assert_eq!(res, Ok(("", Behavior::Function)));
    }

    #[test]
    fn test_parse_behavior_arg() {
        let res = parse_behaviour("arg.42");
        assert_eq!(res, Ok(("", Behavior::Arg(42))));
    }

    #[test]
    fn test_parse_behavior_tetraplet() {
        let res = parse_behaviour("tetraplet");
        assert_eq!(res, Ok(("", Behavior::Tetraplet)));
    }
}
