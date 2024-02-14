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

use futures::FutureExt;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn unit_call_service() -> CallServiceClosure<'static> {
    Box::new(|_| {
        async { CallServiceResult::ok(json!("result from unit_call_service")) }.boxed_local()
    })
}

pub fn echo_call_service() -> CallServiceClosure<'static> {
    Box::new(|mut params| {
        async move { CallServiceResult::ok(params.arguments.remove(0)) }.boxed_local()
    })
}

pub fn set_variable_call_service(json: serde_json::Value) -> CallServiceClosure<'static> {
    Box::new(move |_| {
        {
            let json = json.clone();
            async move { CallServiceResult::ok(json) }
        }
        .boxed_local()
    })
}

/// Manages which source will be used to choose a variable.
pub enum VariableOptionSource {
    // i-th argument
    Argument(usize),
    FunctionName,
    ServiceName,
}

pub fn set_variables_call_service(
    variables_mapping: HashMap<String, serde_json::Value>,
    variable_source: VariableOptionSource,
) -> CallServiceClosure<'static> {
    use VariableOptionSource::*;

    let variable_source = Rc::new(variable_source);
    let variables_mapping = Rc::new(variables_mapping);

    Box::new(move |params| {
        let variable_source = variable_source.clone();
        let variables_mapping = variables_mapping.clone();
        async move {
            let var_name = match variable_source.as_ref() {
                Argument(id) => match params.arguments.get(*id) {
                    Some(serde_json::Value::String(name)) => name.to_string(),
                    _ => "default".to_string(),
                },
                FunctionName => params.function_name,
                ServiceName => params.service_id,
            };

            variables_mapping.get(&var_name).map_or_else(
                || CallServiceResult::ok(json!("default result from set_variables_call_service")),
                |var| CallServiceResult::ok(var.clone()),
            )
        }
        .boxed_local()
    })
}

pub fn return_string_call_service(ret_str: impl Into<String>) -> CallServiceClosure<'static> {
    let ret_str = Rc::new(ret_str.into());

    Box::new(move |_| {
        let ret_str = ret_str.clone();
        async move { CallServiceResult::ok(json!(ret_str.to_string())) }.boxed_local()
    })
}

pub fn fallible_call_service(
    fallible_service_id: impl Into<String>,
) -> CallServiceClosure<'static> {
    let fallible_service_id = Rc::new(fallible_service_id.into());

    Box::new(move |params| {
        let fallible_service_id = fallible_service_id.clone();
        async move {
            // return a error for service with such id
            if params.service_id == fallible_service_id.as_str() {
                CallServiceResult::err(1, json!("failed result from fallible_call_service"))
            } else {
                // return success for services with other service id
                CallServiceResult::ok(json!("success result from fallible_call_service"))
            }
        }
        .boxed_local()
    })
}

pub fn fallible_call_service_by_arg(arg: impl Into<serde_json::Value>) -> CallServiceClosure<'static> {
    let arg = Rc::new(arg.into());

    Box::new(move |params| {
        let arg = arg.clone();
        async move {
            // return a error for service with specific arg
            if params.arguments.get(0) == Some(arg.as_ref()) {
                CallServiceResult::err(1, json!("failed result from fallible_call_service_by_arg"))
            } else {
                // return success for services with other arg
                CallServiceResult::ok(json!("success result from fallible_call_service_by_arg"))
            }
        }
        .boxed_local()
    })
}

pub type ArgTetraplets = Vec<Vec<SecurityTetraplet>>;

pub fn tetraplet_host_function(
    closure: CallServiceClosure<'static>,
) -> (CallServiceClosure<'static>, Rc<RefCell<ArgTetraplets>>) {
    let arg_tetraplets = Rc::new(RefCell::new(ArgTetraplets::new()));
    let closure = Rc::new(closure);

    let arg_tetraplets_inner = arg_tetraplets.clone();

    let host_function: CallServiceClosure<'_> = Box::new(move |params| {
        let arg_tetraplets_inner = arg_tetraplets_inner.clone();
        let closure = closure.clone();
        async move {
            *arg_tetraplets_inner.borrow_mut() = params.tetraplets.clone();
            closure(params).await
        }
        .boxed_local()
    });

    (host_function, arg_tetraplets)
}
