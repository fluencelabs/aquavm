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

use air_interpreter_interface::CallResults;
use fluence_keypair::error::SigningError;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::EnumCount as EnumCountMacro;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
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
