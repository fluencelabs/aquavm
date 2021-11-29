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
    Box::new(|_| -> CallServiceResult {
        CallServiceResult::ok(json!("result from unit_call_service"))
    })
}

pub fn echo_call_service() -> CallServiceClosure {
    Box::new(|mut params| -> CallServiceResult {
        CallServiceResult::ok(params.arguments.remove(0))
    })
}

pub fn set_variable_call_service(json: JValue) -> CallServiceClosure {
    Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(json.clone()) })
}

/// Manages which source will be used to choose a variable.
pub enum VariableOptionSource {
    // i-th argument
    Argument(usize),
    FunctionName,
    ServiceName,
}

pub fn set_variables_call_service(
    variables_mapping: HashMap<String, JValue>,
    variable_source: VariableOptionSource,
) -> CallServiceClosure {
    use VariableOptionSource::*;

    Box::new(move |params| -> CallServiceResult {
        let var_name = match variable_source {
            Argument(id) => match params.arguments.get(id) {
                Some(JValue::String(name)) => name.to_string(),
                _ => "default".to_string(),
            },
            FunctionName => params.function_name,
            ServiceName => params.service_id,
        };

        variables_mapping.get(&var_name).map_or_else(
            || CallServiceResult::ok(json!("default result from set_variables_call_service")),
            |var| CallServiceResult::ok(var.clone()),
        )
    })
}

pub fn return_string_call_service(ret_str: impl Into<String>) -> CallServiceClosure {
    let ret_str = ret_str.into();

    Box::new(move |_| -> CallServiceResult { CallServiceResult::ok(json!(ret_str)) })
}

pub fn fallible_call_service(fallible_service_id: impl Into<String>) -> CallServiceClosure {
    let fallible_service_id = fallible_service_id.into();

    Box::new(move |params| -> CallServiceResult {
        // return a error for service with such id
        if params.service_id == fallible_service_id {
            CallServiceResult::err(1, json!("failed result from fallible_call_service"))
        } else {
            // return success for services with other service id
            CallServiceResult::ok(json!("success result from fallible_call_service"))
        }
    })
}
