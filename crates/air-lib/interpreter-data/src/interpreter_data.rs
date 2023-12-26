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

pub use self::repr::InterpreterDataEnvFormat;
pub use self::repr::InterpreterDataEnvRepr;
pub use self::repr::InterpreterDataFormat;
pub use self::repr::InterpreterDataRepr;
use crate::CidInfo;
use crate::ExecutionTrace;

use air_interpreter_sede::FromSerialized;
use air_interpreter_sede::Representation;
use air_interpreter_sede::ToSerialized;
use air_interpreter_signatures::SignatureStore;
use air_utils::measure;

use serde::Deserialize;
use serde::Serialize;

/// An envelope for the AIR interpreter data that makes AIR data version info accessible in a stable way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpreterDataEnv {
    /// Versions of data and an interpreter produced this data.
    #[serde(flatten)]
    pub versions: Versions,
    #[serde(with = "serde_bytes")]
    pub inner_data: <InterpreterDataRepr as Representation>::SerializedValue,
}

/// The AIR interpreter could be considered as a function
/// f(prev_data: InterpreterData, current_data: InterpreterData, ... ) -> (result_data: InterpreterData, ...).
/// This function receives prev and current data and produces a result data. All these data
/// have the following format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versions {
    /// Version of this data format.
    #[serde(rename = "version")] // for compatibility with versions <= 0.6.0
    pub data_version: semver::Version,

    /// Version of an interpreter produced this data.
    pub interpreter_version: semver::Version,
}

impl InterpreterDataEnv {
    pub fn new(interpreter_version: semver::Version) -> Self {
        let versions = Versions::new(interpreter_version);

        let inner_data = InterpreterData::default();
        let inner_data = InterpreterDataRepr
            .serialize(&inner_data)
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

        // TODO return the error
        let inner_data = InterpreterDataRepr
            .serialize(&inner_data)
            .expect("shouldn't fail");

        Self {
            versions,
            inner_data,
        }
    }

    /// Tries to de InterpreterData from slice according to the data version.
    pub fn try_from_slice(
        slice: &[u8],
    ) -> Result<
        (Versions, InterpreterData),
        <InterpreterDataEnvRepr as Representation>::DeserializeError,
    > {
        let env: InterpreterDataEnv = measure!(
            InterpreterDataEnvRepr.deserialize(slice),
            tracing::Level::INFO,
            "InterpreterData::try_from_slice"
        )?;

        // TODO return error
        let inner_data = InterpreterDataRepr
            .deserialize(&env.inner_data)
            .expect("shouldn't fail");
        Ok((env.versions, inner_data))
    }

    /// Tries to de only versions part of interpreter data.
    pub fn try_get_versions(
        slice: &[u8],
    ) -> Result<Versions, <InterpreterDataEnvRepr as Representation>::DeserializeError> {
        InterpreterDataEnvRepr.deserialize(slice)
    }

    pub fn serialize(
        &self,
    ) -> Result<Vec<u8>, <InterpreterDataEnvRepr as Representation>::SerializeError> {
        InterpreterDataEnvRepr.serialize(self)
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
