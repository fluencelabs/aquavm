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

use super::JValue;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub type CallResults = HashMap<u32, CallServiceResult>;
pub const CALL_SERVICE_SUCCESS: i32 = 0;

/// Represents an executed host function result.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn into_raw(self) -> air_interpreter_interface::CallServiceResult {
        let CallServiceResult { ret_code, result } = self;

        air_interpreter_interface::CallServiceResult {
            ret_code,
            // TODO serializer
            result: result.to_string(),
        }
    }
}

#[tracing::instrument(level = "debug", skip(call_results))]
pub fn into_raw_result(call_results: CallResults) -> air_interpreter_interface::CallResults {
    call_results
        .into_iter()
        .map(|(call_id, call_result)| (call_id.to_string(), call_result.into_raw()))
        .collect::<_>()
}

use std::fmt;

impl fmt::Display for CallServiceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ret_code: {}, result: '{}'", self.ret_code, self.result)
    }
}
