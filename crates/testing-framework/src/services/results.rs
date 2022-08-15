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

use air_test_utils::CallRequestParams;

use super::{FunctionOutcome, JValue, Service};

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
                FunctionOutcome::Ok(
                    self.results.get(&key).expect("Unknown result id").clone(),
                    Duration::ZERO,
                )
            } else {
                // Pass malformed service names further in a chain
                FunctionOutcome::NotDefined
            }
        } else {
            FunctionOutcome::NotDefined
        }
    }
}
