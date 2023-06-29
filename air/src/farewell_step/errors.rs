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

use crate::ToErrorCode;

use air_interpreter_interface::CallResults;
use fluence_keypair::error::SigningError;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error as ThisError;

/// Errors happened during the interpreter farewell step.
#[derive(Debug, EnumDiscriminants, EnumCountMacro, ThisError)]
#[strum_discriminants(derive(EnumIter))]
pub enum FarewellError {
    /// Call results should be empty at the end of execution thanks to a execution invariant.
    #[error(
        "after finishing execution of supplied AIR, there are some unprocessed call results: `{0:?}`, probably a wrong call_id used"
    )]
    UnprocessedCallResult(CallResults),
}

impl ToErrorCode for FarewellError {
    fn to_error_code(&self) -> i64 {
        use crate::utils::FAREWELL_ERRORS_START_ID;
        crate::generate_to_error_code!(self, FarewellError, FAREWELL_ERRORS_START_ID)
    }
}

impl ToErrorCode for SigningError {
    fn to_error_code(&self) -> i64 {
        crate::utils::FAREWELL_ERRORS_START_ID + FarewellError::COUNT as i64
    }
}
