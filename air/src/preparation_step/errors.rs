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

use crate::ToErrorCode;
use air_interpreter_data::data_version;
use air_interpreter_data::verification::DataVerifierError;
use air_interpreter_data::CidStoreVerificationError;
use air_interpreter_data::DataDeserializationError;
use air_interpreter_data::Versions;
use air_interpreter_interface::CallResultsDeserializeError;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use thiserror::Error as ThisError;

/// Errors happened during the interpreter preparation step.
#[derive(Debug, EnumDiscriminants, ThisError)]
#[strum_discriminants(derive(EnumIter))]
pub enum PreparationError {
    /// Error occurred while parsing AIR script
    #[error("air can't be parsed:\n{0}")]
    AIRParseError(String),

    /// Errors occurred on executed trace deserialization.
    #[error(
        "an error occurred while data deserialization: {error:?}.\n\
        AquaVM version is {} and it expects {} version.",
        super::interpreter_version(),
        data_version()
    )]
    DataDeFailed { error: DataDeserializationError },

    /// Errors occurred on executed trace deserialization.
    #[error(
        "an error occurred while envelope deserialization: {error:?}.\n\
        AquaVM version is {} and it expects {} version.",
        super::interpreter_version(),
        data_version()
    )]
    EnvelopeDeFailed { error: DataDeserializationError },

    /// Errors occurred on executed trace deserialization
    /// when it was possible to recover versions.
    #[error(
        "an error occurred while data deserialization: {error:?}.\n\
        AquaVM's version is {} and it expects data of {} version.\n\
        Supplied data version is {}, it's produced by AquaVM of {} version.",
        super::interpreter_version(),
        data_version(),
        versions.data_version,
        versions.interpreter_version,
    )]
    EnvelopeDeFailedWithVersions {
        error: DataDeserializationError,
        versions: Versions,
    },

    /// Error occurred on call results deserialization.
    #[error("error occurred while deserialize call results: {error:?}.")]
    CallResultsDeFailed { error: CallResultsDeserializeError },

    /// Error occurred when a version of interpreter produced supplied data is less then minimal.
    #[error("supplied data was produced by `{actual_version}` version of interpreter, but minimum `{required_version}` version is required")]
    UnsupportedInterpreterVersion {
        actual_version: semver::Version,
        required_version: semver::Version,
    },

    /// Malformed keypair format data.
    #[error("malformed keypair format: {0}")]
    MalformedKeyPairData(#[from] air_interpreter_signatures::KeyError),

    /// Failed to verify CidStore contents of the current data.
    #[error(transparent)]
    CidStoreVerificationError(#[from] CidStoreVerificationError),

    /// Failed to check peers' signatures.
    #[error(transparent)]
    DataSignatureCheckError(#[from] DataVerifierError),

    /// RAM limits are excedeed.
    #[error(transparent)]
    SizeLimitsExceded(#[from] SizeLimitsExceded),
}

impl ToErrorCode for PreparationError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::PREPARATION_ERROR_START_ID;
        crate::generate_to_error_code!(self, PreparationError, PREPARATION_ERROR_START_ID)
    }
}

impl PreparationError {
    pub fn data_de_failed(error: DataDeserializationError) -> Self {
        Self::DataDeFailed { error }
    }

    pub fn envelope_de_failed(error: DataDeserializationError) -> Self {
        Self::EnvelopeDeFailed { error }
    }

    pub fn env_de_failed_with_versions(error: DataDeserializationError, versions: Versions) -> Self {
        Self::EnvelopeDeFailedWithVersions { error, versions }
    }

    pub fn call_results_de_failed(error: CallResultsDeserializeError) -> Self {
        Self::CallResultsDeFailed { error }
    }

    pub fn unsupported_interpreter_version(actual_version: semver::Version, required_version: semver::Version) -> Self {
        Self::UnsupportedInterpreterVersion {
            actual_version,
            required_version,
        }
    }

    pub fn air_size_limit(actual_size: usize, limit: u64) -> Self {
        Self::SizeLimitsExceded(SizeLimitsExceded::Air(actual_size, limit))
    }

    pub fn particle_size_limit(actual_size: usize, limit: u64) -> Self {
        Self::SizeLimitsExceded(SizeLimitsExceded::Particle(actual_size, limit))
    }

    pub fn call_result_size_limit(limit: u64) -> Self {
        Self::SizeLimitsExceded(SizeLimitsExceded::CallResult(limit))
    }
}

#[derive(Debug, ThisError)]
pub enum SizeLimitsExceded {
    /// AIR script size is bigger than the allowed limit.
    #[error("air size: {0} bytes is bigger than the limit allowed: {1} bytes")]
    Air(usize, u64),

    /// Current_data particle size is bigger than the allowed limit.
    #[error("Current_data particle size: {0} bytes is bigger than the limit allowed: {1} bytes")]
    Particle(usize, u64),

    /// CallResult size is bigger than the allowed limit.
    #[error("Call result size is bigger than the limit allowed: {0} bytes")]
    CallResult(u64),
}
