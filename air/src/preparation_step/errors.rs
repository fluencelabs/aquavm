/*
 * Copyright 2020 Fluence Labs Limited
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

use crate::ToErrorCode;
use air_interpreter_data::DATA_FORMAT_VERSION;

use serde_json::Error as SerdeJsonError;
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
    #[error("an error occurred while executed trace deserialization on {1:?}:\n{0:?}.\
    Probably it's a data of an old version that couldn't be converted to '{}'", *DATA_FORMAT_VERSION)]
    DataDeFailed(SerdeJsonError, Vec<u8>),

    /// Error occurred on call results deserialization.
    #[error("error occurred while deserialize call results: {1:?}:\n{0:?}")]
    CallResultsDeFailed(SerdeJsonError, Vec<u8>),
}

impl ToErrorCode for PreparationError {
    fn to_error_code(&self) -> i64 {
        crate::generate_to_error_code!(self, Preparation, 1)
    }
}
