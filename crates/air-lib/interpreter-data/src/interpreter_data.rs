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

use super::ExecutedState;
use super::DATA_FORMAT_VERSION;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::Deref;

pub type StreamGenerations = HashMap<String, u32>;
pub type ExecutionTrace = Vec<ExecutedState>;

/// The AIR interpreter could be considered as a function
/// f(prev_data: InterpreterData, current_data: InterpreterData, ... ) -> (result_data: InterpreterData, ...).
/// This function receives prev and current data and produces a result data. All these data
/// have the following format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterData {
    /// Trace of AIR execution, which contains executed call, par and fold states.
    pub trace: ExecutionTrace,

    /// Contains maximum generation for each stream. This info will be used while merging
    /// values in streams.
    pub streams: StreamGenerations,

    /// Version of this data format.
    pub version: semver::Version,

    /// Last exposed to a peer call request id. All next call request ids will be bigger than this.
    #[serde(default)]
    #[serde(rename = "lcid")]
    pub last_call_request_id: u32,
}

impl InterpreterData {
    pub fn new() -> Self {
        Self {
            trace: <_>::default(),
            streams: <_>::default(),
            version: DATA_FORMAT_VERSION.deref().clone(),
            last_call_request_id: 0,
        }
    }

    pub fn from_execution_result(
        trace: ExecutionTrace,
        streams: StreamGenerations,
        last_call_request_id: u32,
    ) -> Self {
        Self {
            trace,
            streams,
            version: DATA_FORMAT_VERSION.deref().clone(),
            last_call_request_id,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        // treat empty slice as an empty interpreter data allows abstracting from
        // the internal format for empty data.
        if slice.is_empty() {
            return Ok(Self::default());
        }

        serde_json::from_slice(slice)
    }
}

impl Default for InterpreterData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;

    #[test]
    fn compatible_with_0_2_0_version() {
        #[derive(Serialize, Deserialize)]
        struct InterpreterData0_2_0 {
            pub trace: ExecutionTrace,
            pub streams: StreamGenerations,
            pub version: semver::Version,
        }

        // test 0.2.0 to 0.2.1 conversion
        let data_0_2_0 = InterpreterData0_2_0 {
            trace: ExecutionTrace::default(),
            streams: StreamGenerations::default(),
            version: semver::Version::new(0, 2, 0),
        };

        let data_0_2_0_se = serde_json::to_vec(&data_0_2_0).unwrap();
        let data_0_2_1 = serde_json::from_slice::<InterpreterData>(&data_0_2_0_se);
        assert!(data_0_2_1.is_ok());

        // test 0.2.1 to 0.2.1 conversion
        let data_0_2_1 = InterpreterData::default();
        let data_0_2_1_se = serde_json::to_vec(&data_0_2_1).unwrap();
        let data_0_2_0 = serde_json::from_slice::<InterpreterData0_2_0>(&data_0_2_1_se);
        assert!(data_0_2_0.is_ok());
    }
}
