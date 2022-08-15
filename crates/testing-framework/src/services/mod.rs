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

pub(crate) mod results;

use air_test_utils::{CallRequestParams, CallServiceClosure, CallServiceResult};

use std::{rc::Rc, time::Duration};

pub type JValue = serde_json::Value;

/// Somewhat modified type from fluence.  The Duration defines when the caller receives it, imitating
/// real execution time.
#[derive(Debug)]
pub enum FunctionOutcome {
    Ok(JValue, Duration),
    Err(i32, JValue, Duration),
    NotDefined,
    Empty,
}

/// A mocked Marine service.
pub trait Service {
    fn call(&self, params: &CallRequestParams) -> FunctionOutcome;
}

pub(crate) fn services_to_call_service_closure(
    services: Rc<[Rc<dyn Service>]>,
) -> CallServiceClosure {
    Box::new(move |params: CallRequestParams| -> CallServiceResult {
        for service in services.as_ref() {
            let outcome = service.call(&params);
            match outcome {
                FunctionOutcome::Ok(value, _) => return CallServiceResult::ok(value),
                FunctionOutcome::Err(err_code, value, _) => {
                    return CallServiceResult::err(err_code, value)
                }
                FunctionOutcome::NotDefined => continue,
                FunctionOutcome::Empty => todo!("It's not clear yet what to return"),
            }
        }
        todo!("Do not know yet what to return here")
    })
}
