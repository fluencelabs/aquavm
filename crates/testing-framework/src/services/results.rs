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

use super::{FunctionOutcome, MarineService};
use crate::asserts::ServiceDefinition;

use air_test_utils::{
    prelude::{echo_call_service, unit_call_service},
    CallRequestParams, CallServiceClosure, CallServiceResult,
};

use std::{cell::Cell, collections::HashMap, convert::TryInto, time::Duration};

pub struct ResultService {
    results: HashMap<u32, CallServiceClosure>,
}

impl TryInto<CallServiceClosure> for ServiceDefinition {
    type Error = String;

    fn try_into(self) -> Result<CallServiceClosure, String> {
        match self {
            ServiceDefinition::Ok(jvalue) => {
                Ok(Box::new(move |_| CallServiceResult::ok(jvalue.clone())))
            }
            ServiceDefinition::Error(call_result) => Ok(Box::new(move |_| call_result.clone())),
            ServiceDefinition::SeqResult(call_map) => Ok(seq_result_closure(call_map)),
            ServiceDefinition::Behaviour(name) => named_service_closure(name),
        }
    }
}

fn named_service_closure(name: String) -> Result<CallServiceClosure, String> {
    match name.as_str() {
        "echo" => Ok(echo_call_service()),
        "unit" => Ok(unit_call_service()),
        _ => Err(format!("unknown service name: {:?}", name)),
    }
}

fn seq_result_closure(call_map: HashMap<String, serde_json::Value>) -> CallServiceClosure {
    let call_number_seq = Cell::new(0);

    Box::new(move |_| {
        let call_number = call_number_seq.get();
        let call_num_str = call_number.to_string();
        call_number_seq.set(call_number + 1);

        CallServiceResult::ok(
            call_map
                .get(&call_num_str)
                .or_else(|| call_map.get("default"))
                .unwrap_or_else(|| {
                    panic!(
                        "neither value {} nor default value not found in the {:?}",
                        call_num_str, call_map
                    )
                })
                .clone(),
        )
    })
}

impl ResultService {
    pub(crate) fn new(results: HashMap<u32, ServiceDefinition>) -> Result<Self, String> {
        Ok(Self {
            results: results
                .into_iter()
                .map(|(id, service_def)| {
                    service_def
                        .try_into()
                        .map(move |s: CallServiceClosure| (id, s))
                })
                .collect::<Result<_, String>>()?,
        })
    }
}

impl MarineService for ResultService {
    fn call(&self, params: CallRequestParams) -> FunctionOutcome {
        if let Some((_, suffix)) = params.service_id.split_once("..") {
            if let Ok(key) = suffix.parse() {
                let service_desc = self.results.get(&key).expect("Unknown result id");
                FunctionOutcome::ServiceResult(service_desc(params), Duration::ZERO)
            } else {
                // Pass malformed service names further in a chain
                FunctionOutcome::NotDefined
            }
        } else {
            FunctionOutcome::NotDefined
        }
    }
}
