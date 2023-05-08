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
use crate::CanonCidAggregate;
use crate::CanonResultCidAggregate;
use crate::ExecutionTrace;
use crate::JValue;
use crate::ServiceResultCidAggregate;

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
    /// Versions of data and an interpreter produced this data.
    #[serde(flatten)]
    pub versions: Versions,

    /// Trace of AIR execution, which contains executed call, par, fold, and ap states.
    pub trace: ExecutionTrace,

    /// Contains maximum generation for each global stream. This info will be used while merging
    /// values in streams. This field is also needed for backward compatibility with
    /// <= 0.2.1 versions.
    #[serde(rename = "streams")] // for compatibility with versions <= 0.2.1
    pub global_streams: GlobalStreamGens,

    /// Contains maximum generation for each private stream. This info will be used while merging
    /// values in streams.
    #[serde(default)]
    #[serde(rename = "r_streams")]
    pub restricted_streams: RestrictedStreamGens,

    /// Last exposed to a peer call request id. All next call request ids will be bigger than this.
    #[serde(default)]
    #[serde(rename = "lcid")]
    pub last_call_request_id: u32,

    /// CID-to-somethings mappings.
    pub cid_info: CidInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
    /// Version of this data format.
    #[serde(rename = "version")] // for compatibility with versions <= 0.6.0
    pub data_version: semver::Version,

    /// Version of an interpreter produced this data.
    pub interpreter_version: semver::Version,
}

impl InterpreterData {
    pub fn new(interpreter_version: semver::Version) -> Self {
        let versions = Versions::new(interpreter_version);

        Self {
            versions,
            trace: ExecutionTrace::default(),
            global_streams: GlobalStreamGens::new(),
            last_call_request_id: 0,
            restricted_streams: RestrictedStreamGens::new(),
            cid_info: <_>::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_execution_result(
        trace: ExecutionTrace,
        streams: GlobalStreamGens,
        restricted_streams: RestrictedStreamGens,
        cid_info: CidInfo,
        last_call_request_id: u32,
        interpreter_version: semver::Version,
    ) -> Self {
        let versions = Versions::new(interpreter_version);

        Self {
            versions,
            trace,
            global_streams: streams,
            last_call_request_id,
            restricted_streams,
            cid_info,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        measure!(
            serde_json::from_slice(slice),
            tracing::Level::INFO,
            "serde_json::from_slice"
        )
    }

    /// Tries to de only versions part of interpreter data.
    pub fn try_get_versions(slice: &[u8]) -> Result<Versions, serde_json::Error> {
        serde_json::from_slice(slice)
    }
}

impl Versions {
    pub fn new(interpreter_version: semver::Version) -> Self {
        Self {
            data_version: crate::data_version().clone(),
            interpreter_version,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CidInfo {
    /// Map CID to value.
    pub value_store: CidStore<JValue>,

    /// Map CID to a tetraplet.
    pub tetraplet_store: CidStore<SecurityTetraplet>,

    /// Map CID to a canon element value.
    pub canon_element_store: CidStore<CanonCidAggregate>,

    /// Map CID to a canon result.
    pub canon_result_store: CidStore<CanonResultCidAggregate>,

    /// Map CID to a service result aggregate.
    pub service_result_store: CidStore<ServiceResultCidAggregate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;

    #[test]
    fn compatible_with_0_6_0_version() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct InterpreterData0_6_0 {
            pub trace: ExecutionTrace,
            #[serde(rename = "streams")] // for compatibility with versions <= 0.2.1
            pub global_streams: GlobalStreamGens,
            pub version: semver::Version,
            #[serde(default)]
            #[serde(rename = "lcid")]
            pub last_call_request_id: u32,
            #[serde(default)]
            #[serde(rename = "r_streams")]
            pub restricted_streams: RestrictedStreamGens,
            pub interpreter_version: semver::Version,
            pub cid_info: CidInfo,
        }

        // test 0.6.0 to 0.6.1 conversion
        let data_0_6_0 = InterpreterData0_6_0 {
            trace: ExecutionTrace::default(),
            global_streams: GlobalStreamGens::default(),
            version: semver::Version::new(0, 2, 0),
            last_call_request_id: 0,
            restricted_streams: RestrictedStreamGens::default(),
            interpreter_version: semver::Version::new(0, 1, 1),
            cid_info: CidInfo::default(),
        };

        let data_0_6_0_se = serde_json::to_vec(&data_0_6_0).unwrap();
        let data_0_6_1 = serde_json::from_slice::<InterpreterData>(&data_0_6_0_se);
        assert!(data_0_6_1.is_ok());

        // test 0.6.1 to 0.6.0 conversion
        let data_0_6_1 = InterpreterData::new(semver::Version::new(1, 1, 1));
        let data_0_6_1_se = serde_json::to_vec(&data_0_6_1).unwrap();
        let data_0_6_0 = serde_json::from_slice::<InterpreterData0_6_0>(&data_0_6_1_se);
        assert!(data_0_6_0.is_ok());
    }
}
