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

use super::JValue;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type CallResults = HashMap<u32, CallServiceResult>;
pub const CALL_SERVICE_SUCCESS: i32 = 0;

/// Represents an executed host function result.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallServiceResult {
    /// A error code service or builtin returned, where CALL_SERVICE_SUCCESS represents success.
    pub ret_code: i32,

    /// Resulted JValue returned by a service string.
    pub result: JValue,
}

impl CallServiceResult {
    pub fn ok(result: JValue) -> Self {
        Self {
            ret_code: CALL_SERVICE_SUCCESS,
            result,
        }
    }

    pub fn err(err_code: i32, result: JValue) -> Self {
        Self {
            ret_code: err_code,
            result,
        }
    }

    pub(crate) fn into_raw(self) -> air_interpreter_interface::CallServiceResult {
        let CallServiceResult { ret_code, result } = self;

        air_interpreter_interface::CallServiceResult {
            ret_code,
            result: result.to_string(),
        }
    }
}

#[tracing::instrument(skip(call_results))]
pub fn into_raw_result(call_results: CallResults) -> air_interpreter_interface::CallResults {
    call_results
        .into_iter()
        .map(|(call_id, call_result)| (call_id, call_result.into_raw()))
        .collect::<_>()
}

use std::fmt;

impl fmt::Display for CallServiceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ret_code: {}, result: '{}'", self.ret_code, self.result)
    }
}
