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

pub(crate) mod errors;
pub(crate) mod repr;
pub mod verification;

pub use self::repr::InterpreterDataEnvelopeFormat;
pub use self::repr::InterpreterDataEnvelopeRepr;
use crate::CidInfo;
use crate::ExecutionTrace;

use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::Representation;
use air_interpreter_signatures::SignatureStore;

use serde::Deserialize;
use serde::Serialize;

use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum DataDeserializationError {
    #[error("failed to deserialize envelope: {0}")]
    Envelope(rmp_serde::decode::Error),
    #[error("failed to deserialize data: {0}")]
    Data(crate::rkyv::RkyvDeserializeError),
}

/// An envelope for the AIR interpreter data that makes AIR data version info accessible in a stable way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterDataEnvelope<'a> {
    /// Versions of data and an interpreter produced this data.
    #[serde(flatten)]
    pub versions: Versions,
    #[serde(with = "serde_bytes", borrow)]
    pub inner_data: Cow<'a, [u8]>,
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
    #[tracing::instrument(skip_all, level = "info")]
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, DataDeserializationError> {
        let mut aligned_data = rkyv::AlignedVec::with_capacity(slice.len());
        aligned_data.extend_from_slice(slice);

        crate::rkyv::from_aligned_slice(&aligned_data).map_err(DataDeserializationError::Data)
    }

    #[tracing::instrument(skip_all, level = "info")]
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

impl InterpreterDataEnvelope<'_> {
    pub fn new(interpreter_version: semver::Version) -> Self {
        let versions = Versions::new(interpreter_version);

        let inner_data = InterpreterData::default()
            .serialize()
            .expect("shouldn't fail on empty data")
            .into();

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
            .expect("shouldn't fail on valid data")
            .into();

        Self {
            versions,
            inner_data,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    /// Tries to de only versions part of interpreter data.
    pub fn try_get_versions(slice: &[u8]) -> Result<Versions, DataDeserializationError> {
        FromSerialized::deserialize(&InterpreterDataEnvelopeRepr, slice)
            .map_err(DataDeserializationError::Envelope)
    }

    pub fn serialize(
        &self,
    ) -> Result<Vec<u8>, <InterpreterDataEnvelopeRepr as Representation>::SerializeError> {
        // use rmp_serde explicitely until interpreter-sede handles types with lifetimes
        rmp_serde::to_vec_named(self)
    }
}

impl<'data> InterpreterDataEnvelope<'data> {
    #[tracing::instrument(skip_all, level = "info")]
    pub fn try_from_slice(slice: &'data [u8]) -> Result<Self, DataDeserializationError> {
        // use rmp_serde explicitely until interpreter-sede handles types with lifetimes
        rmp_serde::from_slice(slice).map_err(DataDeserializationError::Envelope)
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
