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

use std::{cell::RefCell, rc::Rc, time::Duration};

pub type JValue = serde_json::Value;

/// Somewhat modified type from fluence.  The Duration defines when the caller receives it, imitating
/// real execution time.
#[derive(Debug)]
pub enum FunctionOutcome {
    ServiceResult(CallServiceResult, Duration),
    NotDefined,
    Empty,
}

/// A mocked Marine service.
pub trait Service {
    fn call(&self, params: CallRequestParams) -> FunctionOutcome;

    fn to_handle(self) -> ServiceHandle
    where
        Self: Sized + 'static,
    {
        ServiceHandle(Rc::new(RefCell::new(Box::new(self))))
    }
}

#[derive(Clone)]
pub struct ServiceHandle(Rc<RefCell<Box<dyn Service>>>);

impl Service for ServiceHandle {
    fn call(&self, params: CallRequestParams) -> FunctionOutcome {
        let mut guard = self.0.borrow_mut();
        Service::call(guard.as_mut(), params)
    }
}

pub(crate) fn services_to_call_service_closure(
    services: Rc<[ServiceHandle]>,
) -> CallServiceClosure {
    Box::new(move |params: CallRequestParams| -> CallServiceResult {
        for service_handler in services.as_ref() {
            let outcome = service_handler.call(params.clone());
            match outcome {
                FunctionOutcome::ServiceResult(result, _) => return result,
                FunctionOutcome::NotDefined => continue,
                FunctionOutcome::Empty => return CallServiceResult::ok(serde_json::Value::Null),
            }
        }
        panic!("No function found for params {:?}", params)
    })
}
