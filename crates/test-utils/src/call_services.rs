/*
 * Copyright 2021 Fluence Labs Limited
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

use super::*;
use std::collections::HashMap;

pub fn unit_call_service() -> CallServiceClosure {
    Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(String::from("\"test\"")),
            ])
                .unwrap(),
        ))
    })
}

pub fn echo_string_call_service() -> CallServiceClosure {
    Box::new(|_, args| -> Option<IValue> {
        let arg = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let arg: Vec<String> = serde_json::from_str(arg).unwrap();
        let arg = serde_json::to_string(&arg[0]).unwrap();

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(arg)]).unwrap(),
        ))
    })
}

pub fn echo_number_call_service() -> CallServiceClosure {
    Box::new(|_, args| -> Option<IValue> {
        let arg = match &args[2] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        let arg: Vec<String> = serde_json::from_str(arg).unwrap();

        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(format!(r#""{}, {}""#, arg[0], arg[1])),
            ])
                .unwrap(),
        ))
    })
}

pub fn set_variable_call_service(json: impl Into<String>) -> CallServiceClosure {
    let json = json.into();
    Box::new(move |_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(json.clone())]).unwrap(),
        ))
    })
}

pub fn set_variables_call_service(ret_mapping: HashMap<String, String>) -> CallServiceClosure {
    Box::new(move |_, args| -> Option<IValue> {
        let arg_name = match &args[2] {
            IValue::String(json_str) => {
                let json = serde_json::from_str(json_str).expect("a valid json");
                match json {
                    JValue::Array(array) => match array.first() {
                        Some(JValue::String(str)) => str.to_string(),
                        _ => String::from("default"),
                    },
                    _ => String::from("default"),
                }
            }
            _ => String::from("default"),
        };

        let result = ret_mapping
            .get(&arg_name)
            .cloned()
            .unwrap_or_else(|| String::from(r#""test""#));

        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(result)]).unwrap(),
        ))
    })
}

pub fn fallible_call_service(fallible_service_id: impl Into<String>) -> CallServiceClosure {
    let fallible_service_id = fallible_service_id.into();

    Box::new(move |_, args| -> Option<IValue> {
        let builtin_service = match &args[0] {
            IValue::String(str) => str,
            _ => unreachable!(),
        };

        // return a error for service with such id
        if builtin_service == &fallible_service_id {
            Some(IValue::Record(
                NEVec::new(vec![IValue::S32(1), IValue::String(String::from("error"))]).unwrap(),
            ))
        } else {
            // return success for services with other ids
            Some(IValue::Record(
                NEVec::new(vec![
                    IValue::S32(0),
                    IValue::String(String::from(r#""res""#)),
                ])
                    .unwrap(),
            ))
        }
    })
}
