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

use super::{FunctionOutcome, JValue, Service};

use air_test_utils::{CallRequestParams, CallServiceResult};

use std::{collections::HashMap, time::Duration};

pub struct ResultService {
    results: HashMap<u32, JValue>,
}

impl ResultService {
    pub(crate) fn new(results: HashMap<u32, JValue>) -> Self {
        Self { results }
    }
}

impl Service for ResultService {
    fn call(&self, params: &CallRequestParams) -> FunctionOutcome {
        if let Some((_, suffix)) = params.service_id.split_once("..") {
            if let Ok(key) = suffix.parse() {
                let value = self.results.get(&key).expect("Unknown result id");
                // It is rather CallServiceResult or plain value.
                let result = serde_json::from_value::<CallServiceResult>(value.clone())
                    .unwrap_or_else(|_| CallServiceResult::ok(value.clone()));
                FunctionOutcome::ServiceResult(result, Duration::ZERO)
            } else {
                // Pass malformed service names further in a chain
                FunctionOutcome::NotDefined
            }
        } else {
            FunctionOutcome::NotDefined
        }
    }
}
