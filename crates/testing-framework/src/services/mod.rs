/*
 * AquaVM Workflow Engine
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub(crate) mod results;

use self::results::{MarineServiceWrapper, ResultStore};

use futures::future::LocalBoxFuture;
use futures::FutureExt;

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

impl FunctionOutcome {
    pub fn from_service_result(service_result: CallServiceResult) -> Self {
        FunctionOutcome::ServiceResult(service_result, Duration::ZERO)
    }

    pub fn from_value(value: JValue) -> Self {
        Self::from_service_result(CallServiceResult::ok(value))
    }
}

/// A mocked Marine service.
pub trait MarineService {
    fn call<'this>(
        &'this self,
        params: CallRequestParams,
    ) -> LocalBoxFuture<'this, FunctionOutcome>;

    fn to_handle(self) -> MarineServiceHandle
    where
        Self: Sized + 'static,
    {
        MarineServiceHandle(Rc::new(RefCell::new(Box::new(self))))
    }
}

#[derive(Clone)]
pub struct MarineServiceHandle(Rc<RefCell<Box<dyn MarineService>>>);

impl MarineService for MarineServiceHandle {
    fn call<'this>(
        &'this self,
        params: CallRequestParams,
    ) -> LocalBoxFuture<'this, FunctionOutcome> {
        async {
            let mut guard = self.0.borrow_mut();
            MarineService::call(guard.as_mut(), params).await
        }
        .boxed_local()
    }
}

pub(crate) fn services_to_call_service_closure(
    services: Rc<[MarineServiceHandle]>,
) -> CallServiceClosure<'static> {
    Box::new(move |params: CallRequestParams| {
        let services = services.clone();
        async move {
            for service_handler in services.as_ref() {
                let outcome = service_handler.call(params.clone()).await;
                match outcome {
                    FunctionOutcome::ServiceResult(result, _) => return result,
                    FunctionOutcome::NotDefined => continue,
                    FunctionOutcome::Empty => {
                        return CallServiceResult::ok(serde_json::Value::Null)
                    }
                }
            }
            panic!("No function found for params {:?}", params)
        }
        .boxed_local()
    })
}

pub(crate) struct NetworkServices {
    result_store: Rc<ResultStore>,
    common_services: Rc<[MarineServiceHandle]>,
}

impl NetworkServices {
    pub(crate) fn new(mut common_services: Vec<MarineServiceHandle>) -> Self {
        let result_service = Rc::<ResultStore>::default();

        // insert result service into all services:
        let wrapper = MarineServiceWrapper::new(result_service.clone()).to_handle();
        common_services.insert(0, wrapper);

        Self {
            result_store: result_service,
            common_services: common_services.into(),
        }
    }

    pub(crate) fn get_result_store(&self) -> Rc<ResultStore> {
        self.result_store.clone()
    }

    pub(crate) fn get_services(&self) -> Rc<[MarineServiceHandle]> {
        self.common_services.clone()
    }
}
