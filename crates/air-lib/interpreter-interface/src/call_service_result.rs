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

use air_interpreter_sede::define_simple_representation;
use air_interpreter_sede::derive_serialized_type;
use air_interpreter_sede::MsgPackMultiformat;
use air_interpreter_sede::Representation;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JValue;
use std::collections::HashMap;

/// This is a map from a String to a service result for compatibility with JavaScript.
/// Binary format implementations like `rmp-serde` do not convert keys from strings, unlike `serde_json`.
pub type CallResults = HashMap<String, CallServiceResult>;
pub const CALL_SERVICE_SUCCESS: i32 = 0;

pub type CallResultsFormat = MsgPackMultiformat;

derive_serialized_type!(SerializedCallResults);

define_simple_representation! {
    CallResultsRepr,
    CallResults,
    CallResultsFormat,
    SerializedCallResults
}

pub type CallResultsDeserializeError = <CallResultsRepr as Representation>::DeserializeError;
pub type CallResultsSerializeError = <CallResultsRepr as Representation>::SerializeError;

/// Represents an executed host function result.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallServiceResult {
    /// A error code service or builtin returned, where CALL_SERVICE_SUCCESS represents success.
    pub ret_code: i32,

    /// Resulted JValue serialized to a string. It's impossible to wrap it with the marine macro,
    /// inasmuch as it's a enum uses HashMap inside.
    pub result: String,
}

impl CallServiceResult {
    pub fn ok(result: &JValue) -> Self {
        Self {
            ret_code: CALL_SERVICE_SUCCESS,
            // for compatiblity with JavaScript with binary formats, string IDs are used
            result: result.to_string(),
        }
    }

    pub fn err(err_code: i32, result: &JValue) -> Self {
        Self {
            ret_code: err_code,
            result: result.to_string(),
        }
    }
}

use std::fmt;

impl fmt::Display for CallServiceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ret_code: {}, result: '{}'", self.ret_code, self.result)
    }
}
