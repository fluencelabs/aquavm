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

pub(crate) mod errors;
pub(crate) mod repr;
pub mod verification;

pub use self::repr::InterpreterDataEnvelopeFormat;
pub use self::repr::InterpreterDataEnvelopeRepr;
use crate::CidInfo;
use crate::ExecutionTrace;

use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::Representation;
use air_interpreter_sede::ToSerialized;
use air_interpreter_signatures::SignatureStore;
use air_utils::measure;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DataDeserializationError {
    #[error("failed to deserialize envelope: {0}")]
    Envelope(<InterpreterDataEnvelopeRepr as Representation>::DeserializeError),
    #[error("failed to deserialize data: {0}")]
    Data(crate::rkyv::RkyvDeserializeError),
}

/// An envelope for the AIR interpreter data that makes AIR data version info accessible in a stable way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterDataEnvelope {
    /// Versions of data and an interpreter produced this data.
    #[serde(flatten)]
    pub versions: Versions,
    #[serde(with = "serde_bytes")]
    pub inner_data: Vec<u8>,
}

/// The AIR interpreter could be considered as a function
/// f(prev_data: InterpreterData, current_data: InterpreterData, ... ) -> (result_data: InterpreterData, ...).
/// This function receives prev and current data and produces a result data. All these data
/// have the following format.
#[derive(
    Debug,
    Clone,
    Default,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
)]
#[archive(check_bytes)]
pub struct InterpreterData {
    /// Trace of AIR execution, which contains executed call, par, fold, and ap states.
    pub trace: ExecutionTrace,

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

impl InterpreterData {
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, crate::rkyv::RkyvDeserializeError> {
        let mut aligned_data = rkyv::AlignedVec::with_capacity(slice.len());
        aligned_data.extend_from_slice(slice);

        crate::rkyv::from_aligned_slice(&aligned_data)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, crate::rkyv::RkyvSerializeError> {
        crate::rkyv::to_vec(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
    /// Version of this data format.
    #[serde(rename = "version")] // for compatibility with versions <= 0.6.0
    pub data_version: semver::Version,

    /// Version of an interpreter produced this data.
    pub interpreter_version: semver::Version,
}

impl InterpreterDataEnvelope {
    pub fn new(interpreter_version: semver::Version) -> Self {
        let versions = Versions::new(interpreter_version);

        let inner_data = InterpreterData::default()
            .serialize()
            .expect("shouldn't fail on empty data");

        Self {
            versions,
            inner_data,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_execution_result(
        trace: ExecutionTrace,
        cid_info: CidInfo,
        signatures: SignatureStore,
        last_call_request_id: u32,
        interpreter_version: semver::Version,
    ) -> Self {
        let versions = Versions::new(interpreter_version);

        let inner_data = InterpreterData {
            trace,
            last_call_request_id,
            cid_info,
            signatures,
        };

        let inner_data = inner_data
            .serialize()
            .expect("shouldn't fail on valid data");

        Self {
            versions,
            inner_data,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(
        slice: &[u8],
    ) -> Result<(Versions, InterpreterData), DataDeserializationError> {
        let env: InterpreterDataEnvelope = measure!(
            FromSerialized::deserialize(&InterpreterDataEnvelopeRepr, slice),
            tracing::Level::INFO,
            "InterpreterData::try_from_slice"
        )
        .map_err(DataDeserializationError::Envelope)?;

        let mut aligned_data = rkyv::AlignedVec::with_capacity(env.inner_data.len());
        aligned_data.extend_from_slice(&env.inner_data);

        let inner_data = InterpreterData::try_from_slice(&env.inner_data)
            .map_err(DataDeserializationError::Data)?;
        Ok((env.versions, inner_data))
    }

    /// Tries to de only versions part of interpreter data.
    pub fn try_get_versions(slice: &[u8]) -> Result<Versions, DataDeserializationError> {
        FromSerialized::deserialize(&InterpreterDataEnvelopeRepr, slice)
            .map_err(DataDeserializationError::Envelope)
    }

    pub fn serialize(
        &self,
    ) -> Result<Vec<u8>, <InterpreterDataEnvelopeRepr as Representation>::SerializeError> {
        InterpreterDataEnvelopeRepr.serialize(self)
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
