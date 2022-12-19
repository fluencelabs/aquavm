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

use super::GlobalStreamGens;
use super::RestrictedStreamGens;
use crate::cid_store::CidStore;
use crate::CanonValueAggregate;
use crate::ExecutionTrace;
use crate::JValue;

use air_utils::measure;
use polyplets::SecurityTetraplet;

use serde::Deserialize;
use serde::Serialize;

/// The AIR interpreter could be considered as a function
/// f(prev_data: InterpreterData, current_data: InterpreterData, ... ) -> (result_data: InterpreterData, ...).
/// This function receives prev and current data and produces a result data. All these data
/// have the following format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterData {
    /// Trace of AIR execution, which contains executed call, par, fold, and ap states.
    pub trace: ExecutionTrace,

    /// Contains maximum generation for each global stream. This info will be used while merging
    /// values in streams. This field is also needed for backward compatibility with
    /// <= 0.2.1 versions.
    #[serde(rename = "streams")] // for compatibility with versions <= 0.2.1
    pub global_streams: GlobalStreamGens,

    /// Version of this data format.
    pub version: semver::Version,

    /// Last exposed to a peer call request id. All next call request ids will be bigger than this.
    #[serde(default)]
    #[serde(rename = "lcid")]
    pub last_call_request_id: u32,

    /// Contains maximum generation for each private stream. This info will be used while merging
    /// values in streams.
    #[serde(default)]
    #[serde(rename = "r_streams")]
    pub restricted_streams: RestrictedStreamGens,

    /// Version of interpreter produced this data.
    pub interpreter_version: semver::Version,

    /// Map CID to value
    pub cid_store: CidStore<JValue>,

    /// Map CID to a tetraplet
    pub tetraplet_store: CidStore<SecurityTetraplet>,

    /// Map CID to a canon value
    pub canon_store: CidStore<CanonValueAggregate>,
}

impl InterpreterData {
    pub fn new(interpreter_version: semver::Version) -> Self {
        Self {
            trace: ExecutionTrace::default(),
            global_streams: GlobalStreamGens::new(),
            version: crate::data_version().clone(),
            last_call_request_id: 0,
            restricted_streams: RestrictedStreamGens::new(),
            interpreter_version,
            cid_store: <_>::default(),
            tetraplet_store: <_>::default(),
            canon_store: <_>::default(),
        }
    }

    pub fn from_execution_result(
        trace: ExecutionTrace,
        streams: GlobalStreamGens,
        restricted_streams: RestrictedStreamGens,
        cid_store: impl Into<CidStore<JValue>>,
        tetraplet_store: impl Into<CidStore<SecurityTetraplet>>,
        canon_store: impl Into<CidStore<CanonValueAggregate>>,
        last_call_request_id: u32,
        interpreter_version: semver::Version,
    ) -> Self {
        let cid_store = cid_store.into();
        let tetraplet_store = tetraplet_store.into();
        let canon_store = canon_store.into();

        Self {
            trace,
            global_streams: streams,
            version: crate::data_version().clone(),
            last_call_request_id,
            restricted_streams,
            interpreter_version,
            cid_store,
            tetraplet_store,
            canon_store,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(
        slice: &[u8],
        min_support_version: &semver::Version,
    ) -> Result<Self, serde_json::Error> {
        // treat empty slice as an empty interpreter data allows abstracting from
        // the internal format for empty data.
        if slice.is_empty() {
            return Ok(Self::new(min_support_version.clone()));
        }

        measure!(
            serde_json::from_slice(slice),
            tracing::Level::INFO,
            "serde_json::from_slice"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;

    #[test]
    #[ignore] // TODO: fix tests
    fn compatible_with_0_2_0_version() {
        #[derive(Serialize, Deserialize)]
        struct InterpreterData0_2_0 {
            pub trace: ExecutionTrace,
            pub streams: GlobalStreamGens,
            pub version: semver::Version,
        }

        // test 0.2.0 to 0.2.2 conversion
        let data_0_2_0 = InterpreterData0_2_0 {
            trace: ExecutionTrace::default(),
            streams: GlobalStreamGens::default(),
            version: semver::Version::new(0, 2, 0),
        };

        let data_0_2_0_se = serde_json::to_vec(&data_0_2_0).unwrap();
        let data_0_2_1 = serde_json::from_slice::<InterpreterData>(&data_0_2_0_se);
        assert!(data_0_2_1.is_ok());

        // test 0.2.2 to 0.2.0 conversion
        let data_0_2_2 = InterpreterData::new(semver::Version::new(1, 1, 1));
        let data_0_2_2_se = serde_json::to_vec(&data_0_2_2).unwrap();
        let data_0_2_0 = serde_json::from_slice::<InterpreterData0_2_0>(&data_0_2_2_se);
        assert!(data_0_2_0.is_ok());
    }

    #[test]
    #[ignore] // TODO: fix tests
    fn compatible_with_0_2_1_version() {
        #[derive(Serialize, Deserialize)]
        struct InterpreterData0_2_1 {
            pub trace: ExecutionTrace,
            pub streams: GlobalStreamGens,
            pub version: semver::Version,
            #[serde(default)]
            #[serde(rename = "lcid")]
            pub last_call_request_id: u32,
        }

        // test 0.2.1 to 0.2.2 conversion
        let data_0_2_1 = InterpreterData0_2_1 {
            trace: ExecutionTrace::default(),
            streams: GlobalStreamGens::default(),
            version: semver::Version::new(0, 2, 1),
            last_call_request_id: 1,
        };

        let data_0_2_1_se = serde_json::to_vec(&data_0_2_1).unwrap();
        let data_0_2_2 = serde_json::from_slice::<InterpreterData>(&data_0_2_1_se);
        assert!(data_0_2_2.is_ok());

        // test 0.2.2 to 0.2.1 conversion
        let data_0_2_2 = InterpreterData::new(semver::Version::new(1, 1, 1));
        let data_0_2_2_se = serde_json::to_vec(&data_0_2_2).unwrap();
        let data_0_2_0 = serde_json::from_slice::<InterpreterData0_2_1>(&data_0_2_2_se);
        assert!(data_0_2_0.is_ok());
    }
}
