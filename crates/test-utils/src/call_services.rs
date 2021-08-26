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

use serde_json::json;
use std::collections::HashMap;

pub fn unit_call_service() -> CallServiceClosure {
    Box::new(|_| -> CallServiceResult { CallServiceResult::ok(&json!("test")) })
}

pub fn echo_call_service() -> CallServiceClosure {
    Box::new(|params| -> CallServiceResult {
        let args: Vec<serde_json::Value> = serde_json::from_str(&params.arguments).unwrap();
        CallServiceResult::ok(&args[0])
    })
}

pub fn set_variable_call_service(json: JValue) -> CallServiceClosure {
    Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(&json) })
}

pub fn set_variables_call_service(
    variables_mapping: HashMap<String, JValue>,
) -> CallServiceClosure {
    Box::new(move |params| -> CallServiceResult {
        let args: Vec<serde_json::Value> = serde_json::from_str(&params.arguments).unwrap();
        let var_name = match args.first() {
            Some(JValue::String(name)) => name.clone(),
            _ => "default".to_string(),
        };

        variables_mapping.get(&var_name).map_or_else(
            || CallServiceResult::ok(&json!("test")),
            |var| CallServiceResult::ok(var),
        )
    })
}

pub fn return_string_call_service(ret_str: impl Into<String>) -> CallServiceClosure {
    let ret_str = ret_str.into();

    Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(&json!(ret_str)) })
}

pub fn fallible_call_service(fallible_service_id: impl Into<String>) -> CallServiceClosure {
    let fallible_service_id = fallible_service_id.into();

    Box::new(move |params| -> CallServiceResult {
        // return a error for service with such id
        if &params.service_id == &fallible_service_id {
            CallServiceResult::err(1, &json!("error"))
        } else {
            // return success for services with other service id
            CallServiceResult::ok(&json!("test"))
        }
    })
}
