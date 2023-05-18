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
use crate::CidStoreVerificationError;
use crate::ExecutionTrace;
use crate::JValue;
use crate::ServiceResultCidAggregate;

use air_interpreter_signatures::SignatureStore;
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

    /// Signature store.
    ///
    /// Every peer signs call results and canon values it produced (all together), and stores the signatures
    /// in this store.
    pub signatures: SignatureStore,
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
            signatures: <_>::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_execution_result(
        trace: ExecutionTrace,
        streams: GlobalStreamGens,
        restricted_streams: RestrictedStreamGens,
        cid_info: CidInfo,
        signatures: SignatureStore,
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
            signatures,
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

impl CidInfo {
    #[tracing::instrument(skip_all)]
    pub fn verify(&self) -> Result<(), CidStoreVerificationError> {
        self.value_store.verify()?;
        self.tetraplet_store.verify()?;
        self.canon_element_store.verify()?;
        self.canon_result_store.verify()?;
        self.service_result_store.verify()?;
        Ok(())
    }
}
